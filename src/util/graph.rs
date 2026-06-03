use std::cmp::max;
use horned_owl::model::ForIRI;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash};
use indicatif::ProgressBar;
use petgraph::graph::{Graph, DiGraph, NodeIndex};
use petgraph::algo::{astar};
use petgraph::visit::{EdgeRef};
use graph_cycles::Cycles;
use crate::dependency::base::{SymbolDependencyMap, TermDependencyMap};

fn merge_paths<D: Eq + Hash + Clone + Sized + Debug>(a: HashSet<Vec<D>>, b: HashSet<Vec<D>>, limit:usize) -> HashSet<Vec<D>> {
    a.iter().flat_map(|ae| b.iter().map(|be| [&ae[..], &be[..]].concat())).take(limit).collect()
}

fn expand_map<K: Eq + Hash, C: Eq + Hash>(m1: &mut HashMap<K, HashSet<C>>, m2: HashMap<K,HashSet<C>>, limit:usize) {
    for (k,vs) in m2 {
        let c = m1.entry(k).or_insert(HashSet::new());
        c.extend(vs.into_iter().take(max(0,limit-c.len())));
    }
}

pub fn transitive_closure_slow<K: Hash + Eq + Clone + Debug, D: Eq + Hash + Clone + Sized + Debug>(
    depmap: HashMap<K, HashMap<K, HashSet<Vec<D>>>>, k: usize,
) -> HashMap<K, HashMap<K, HashSet<Vec<D>>>> {
    let nodes = depmap.keys().chain(depmap.values().flat_map(|v| v.keys())).collect::<HashSet<_>>();

    let mut graph = DiGraph::new();
    let mut index_map: HashMap<K, NodeIndex> = HashMap::new();
    let mut inv_index_map: HashMap<NodeIndex, K> = HashMap::new();
    let mut data_count = 0;
    let mut data_map = HashMap::new();
    for node in nodes.into_iter() {
        let idx = graph.add_node(node.clone());
        index_map.insert(node.clone(), idx.clone());
        inv_index_map.insert(idx, node.clone());
    }
    for (k,vs) in depmap.iter() {
        for (v, data) in vs{
            graph.add_edge(index_map[k],index_map[v], data_count);
            data_map.insert(data_count, data);
            data_count = data_count + 1;
        }
    }

    let mut representation_map: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut class_map = HashMap::new();
    let mut cycle_edges = HashSet::new();
    // Turn cycles into equivalence classes
    println!("Look for cycles");
    for cycle in graph.cycles(){
        let mut cycle_iter = cycle.iter();
        let progress_bar = ProgressBar::new(cycle_iter.len() as u64);
        let c0 = cycle_iter.next().unwrap();
        let r0 = *representation_map.get(c0).unwrap_or(c0);
        let mut r0_class : HashSet<NodeIndex> = HashSet::from([r0]);
        let mut prev = *c0;
        while let Some(c1) = cycle_iter.next() {

            progress_bar.inc(1);
            // Take representative of [c1]
            r0_class.insert(*c1);
            // Calculate all paths from
            for edge in graph.edges_connecting(prev,*c1) {
                cycle_edges.insert((prev, *c1, *edge.weight()));
            }

            let r1= *representation_map.get(&c1).unwrap_or(c1);
            if let Some(r1_class) = class_map.remove(&r1) {
                r0_class.extend(r1_class);
            }
            representation_map.insert(r1,r0);
            let to_replace = representation_map.iter().filter(|(_,r2)| **r2 == r1).map(|(k, _)| k.clone()).collect::<HashSet<_>>();
            for c2 in to_replace.into_iter() {
                representation_map.insert(c2.clone(), r0.clone());
            }
            prev = c1.clone();
        }
        for edge in graph.edges_connecting(prev,*c0) {
            cycle_edges.insert((prev, *c0, *edge.weight()));
        }
        let current_r0_class = class_map.entry(r0).or_insert(HashSet::new());
        current_r0_class.extend(r0_class);
    }

    let cycle_graph: DiGraph<NodeIndex, _> = DiGraph::from_edges(&cycle_edges);
    println!("Collapse cycles");
    // Collapse equivalence classes
    let edges = graph.edge_references().filter_map(|e| {
        let n1 = e.source();
        let n2 = e.target();
        let _paths_n1_n2 =  data_map[e.weight()].clone();
        let r1 = *representation_map.get(&n1).unwrap_or(&n1);
        let r2 = *representation_map.get(&n2).unwrap_or(&n2);
        if r1 != r2 {
            Some((r1, r2))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    let nodes = edges.iter().map(|(a,b)| [a,b]).flatten().cloned().collect::<HashSet<_>>();
    let graph: DiGraph<NodeIndex, _> = DiGraph::from_edges(edges);
    let mut reachable: HashMap<K, HashMap<K, HashSet<Vec<D>>>> = HashMap::new();
    // Get topological order
    let toposort : Vec<_> = petgraph::algo::toposort::<&Graph<NodeIndex, i32>>(&graph, None).expect("Graph is still cyclic!").into_iter().filter(|k| nodes.contains(k)).collect();
    println!("Compute transitive closure");
    let progress = ProgressBar::new(toposort.len() as u64);
    for top_node_index in toposort.into_iter().rev(){
        progress.inc(1);
        let equivalence_class_of_node: HashSet<NodeIndex> = match class_map.get(&top_node_index) {
            Some(c) => c.clone(), //Todo: Remove this clone
            None => HashSet::from([top_node_index]),
        };
        for node_idx in equivalence_class_of_node.iter() {
            let node = &inv_index_map[node_idx];
            let mut reachable_from_node = HashMap::new();
            if let Some(outgoing_edges) = depmap.get(node) {
                for (target, paths_node_target) in outgoing_edges {
                    if let Some(outgoing_from_target) = reachable.get(&target) {
                        for (reachable_node, paths_target_reachable_node) in outgoing_from_target.iter() {
                            if !equivalence_class_of_node.contains(&index_map[reachable_node]) {
                                reachable_from_node.entry(reachable_node.clone()).or_insert_with(HashSet::new).extend(merge_paths(paths_node_target.clone(), paths_target_reachable_node.clone(), k))
                            }
                        }
                    }
                    if !equivalence_class_of_node.contains(&index_map[target]) {
                        reachable_from_node.entry(target.clone()).or_insert_with(HashSet::new).extend(paths_node_target.clone());
                    }
                }
                expand_map(reachable.entry(node.clone()).or_insert_with(HashMap::new), reachable_from_node.clone(), k);
            }
        }
        let mut to_add = HashMap::new();
        for p_idx in equivalence_class_of_node.iter() {
            let p= &inv_index_map[p_idx];
            let mut reachable_from_p = HashMap::new();

            for q_idx in equivalence_class_of_node.iter() {
                if *p_idx != *q_idx {
                    let q = &inv_index_map[q_idx];
                    if let Some((_, node_path_p_q)) = astar(&cycle_graph, *p_idx, |r| r == *q_idx, |_|1, |r| match r == *q_idx {
                        true => 0,
                        false => 1
                    }) {
                        let mut paths_p_q = HashSet::from([vec![]]);
                        let mut path_iter = node_path_p_q.iter();
                        let mut prev = path_iter.next().unwrap();
                        while let Some(curr) = path_iter.next() {
                            paths_p_q = merge_paths(paths_p_q, cycle_graph.edges_connecting(*prev, *curr).flat_map(|edge| data_map[edge.weight()].clone()).collect(), k);
                            prev = curr;
                        }
                        for (reachable_node, paths_from_q) in reachable[q].iter() {
                            if !equivalence_class_of_node.contains(&index_map[reachable_node]) {
                                reachable_from_p.entry(reachable_node.clone()).or_insert_with(HashSet::new).extend(merge_paths(paths_p_q.clone(), paths_from_q.clone(), k))
                            }
                        }
                        reachable_from_p.entry(q.clone()).or_insert_with(HashSet::new).extend(paths_p_q.clone());
                    }
                }
            }
            expand_map(to_add.entry(p.clone()).or_insert_with(HashMap::new), reachable_from_p.clone(),k);
        }
        for (key, v) in to_add {
            expand_map(reachable.entry(key.clone()).or_insert_with(HashMap::new), v, k);
        }
    }

    reachable
}

pub fn transitive_closure<'a, 'b: 'a, T: ForIRI>(
    graph: TermDependencyMap<'a, T>, k: usize,
) -> SymbolDependencyMap<'a, T> {
    let tc = transitive_closure_slow(graph, k);
    // Reduce to symbolic parts
    let map = tc.into_iter().filter_map(|(k, vd)|
        match k.is_atomic() {
            true => Some((k.get_symbol().unwrap().clone(), vd.into_iter().filter_map(|(k2, d)| match k2.is_atomic() {
                true => Some((k2.get_symbol().unwrap().clone(), d)),
                false => None
            }).collect())),
            false => None,
        }).collect();
    map
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn construct_graph<T:Hash + Eq + Display + Clone>(tuples: Vec<(T, Vec<T>)>) -> HashMap<T, HashMap<T, HashSet<Vec<String>>>>{
        HashMap::from_iter(tuples.into_iter().map(|(a,bs)| (a.clone(), HashMap::from_iter(bs.into_iter().map(|b| (b.clone(), HashSet::from([vec![format!("{}->{}",a.clone(),b.clone()).into()]])))))))
    }

    #[test]
    fn test_merge() {
        let a = HashSet::from([vec!["1 -> 2"], vec!["1 -> 3", "3 -> 2"]]);
        let b = HashSet::from([vec!["2 -> 5", "5 -> 6"], vec!["2 -> 6"]]);
        let merged = merge_paths(a,b,8);
        println!("{:?}", merged);
        assert_eq!(merged.len(), 4);
        assert!(merged.contains(&vec!["1 -> 2", "2 -> 6"]));
        assert!(merged.contains(&vec!["1 -> 2", "2 -> 5", "5 -> 6"]));
        assert!(merged.contains(&vec!["1 -> 3", "3 -> 2", "2 -> 6"]));
        assert!(merged.contains(&vec!["1 -> 3", "3 -> 2", "2 -> 5", "5 -> 6"]));
    }

    #[test]
    fn test_merge_left_neutral() {
        let a = HashSet::from([vec![]]);
        let b = HashSet::from([vec!["2 -> 5", "5 -> 6"], vec!["2 -> 6"]]);
        assert_eq!(b,merge_paths(a,b.clone(), 8));
    }

    #[test]
    fn test_merge_right_neutral() {
        let a = HashSet::from([vec![]]);
        let b = HashSet::from([vec!["2 -> 5", "5 -> 6"], vec!["2 -> 6"]]);
        assert_eq!(b,merge_paths(b.clone(),a, 8));
    }

    #[test]
    fn test_transitive_closure_simple() {
        let graph = construct_graph(vec![
            ("a",vec!["b"]),
            ("b",vec!["c"]),
        ]);
        let tc = transitive_closure_slow(graph, 5);
        println!("{:?}", tc);
        assert_eq!(tc["a"]["c"], HashSet::from([vec!["a->b".into(), "b->c".into()]]));
    }

    #[test]
    fn test_transitive_closure_cycle() {
        let graph = construct_graph(vec![
            (String::from("a"),vec![String::from("b")]),
            (String::from("b"),vec![String::from("c")]),
            (String::from("c"),vec![String::from("a"), String::from("d")]),
        ]);
        let tc = transitive_closure_slow(graph, 5);
        println!("{:?}", tc);

        assert_eq!(tc["a"]["c"], HashSet::from([vec!["a->b".into(), "b->c".into()]]));
        assert_eq!(tc["c"]["a"], HashSet::from([vec!["c->a".into()]]));
        assert_eq!(tc["a"]["d"], HashSet::from([vec!["a->b".into(), "b->c".into(), "c->d".into()]]));
    }

    #[test]
    fn test_transitive_closure_cycle2() {
        let graph = construct_graph(vec![
            (String::from("a"),vec![String::from("b")]),
            (String::from("b"),vec![String::from("c")]),
            (String::from("c"),vec![String::from("d"), String::from("e")]),
            (String::from("e"),vec![String::from("a"), String::from("f")]),
        ]);
        let tc = transitive_closure_slow(graph, 5);
        println!("{:?}", tc);

        assert_eq!(tc["a"]["f"], HashSet::from([vec!["a->b".into(), "b->c".into(), "c->e".into(), "e->f".into()]]));
    }

    #[test]
    fn test_transitive_closure_nontree() {
        let graph:HashMap<&str, HashMap<&str, HashSet<Vec<&str>>>> = HashMap::from([
            ("a",HashMap::from([("b", [vec!["a->b"]].into())])),
            ("b",HashMap::from([("c", [vec!["b->c"]].into()), ("d", [vec!["b->d"]].into())])),

            ("c",HashMap::from([("e", [vec!["c->e"]].into())])),
            ("d",HashMap::from([("e", [vec!["d->e"]].into())])),
        ]);
        let tc = transitive_closure_slow(graph, 5);
        println!("{:?}", tc);

        assert_eq!(tc["a"]["e"], HashSet::from([vec!["a->b", "b->c", "c->e"], vec!["a->b", "b->d", "d->e"]]));
    }

}
