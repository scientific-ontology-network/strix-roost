use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Compute the transitive closure of a directed graph.
/// Input: adjacency list as HashMap<T, HashSet<T>>.
/// Output: new adjacency list with reachability closure.
pub fn transitive_closure<S: Eq + Hash + Clone, C: Eq+ Hash + Clone>(
    graph: &HashMap<S, HashMap<S, HashSet<C>>>,
) -> HashMap<S, HashMap<S, HashSet<C>>> {
    let mut closure: HashMap<S, HashMap<S, HashSet<C>>> = HashMap::new();
    let mut memo: HashMap<S, HashMap<S, HashSet<C>>> = HashMap::new();

    for node in graph.keys() {
        let reachable = dfs_with_memo(node, graph, &mut memo);
        closure.insert(node.clone(), reachable);
    }
    closure
}

/// DFS with memoization: only one SC per symbol
fn dfs_with_memo<S: Eq + Hash + Clone, C: Eq+ Hash + Clone>(
    start: &S,
    graph: &HashMap<S, HashMap<S, HashSet<C>>>,
    memo: &mut HashMap<S, HashMap<S, HashSet<C>>>,
) -> HashMap<S, HashSet<C>> {
    // If already memoized, return immediately
    if let Some(cached) = memo.get(start) {
        return cached.clone();
    }

    let mut visited: HashMap<S, HashSet<C>> = HashMap::new();
    let mut stack= vec![(start.clone(), HashSet::new())];

    while let Some((sym, ax)) = stack.pop() {

        if visited.contains_key(&sym) {
            continue; // already visited this symbol
        }

        visited.insert(sym.clone(), ax.clone());

        // push neighbors
        if let Some(neighbors) = graph.get(&sym) {
            for (neigh_sym, neigh_ax) in neighbors {
                if !visited.contains_key(neigh_sym) {
                    stack.push((neigh_sym.clone(), neigh_ax.union(&ax).into_iter().cloned().collect()) );
                }
            }
        }
    }

    // Remove self if irreflexive closure is desired
    visited.remove(start);

    // Memoize before returning
    memo.insert(start.clone(), visited.clone());

    visited
}

#[cfg(test)]
mod tests {
    
    use crate::dependency::symbol::{Symbol};
    use super::*;

    #[test]
    fn test_transitive_closure() {

        let to_ds = |s: &str| Symbol::Class(s.to_string());
        let mut graph: HashMap<Symbol<String>, HashMap<Symbol<String>, HashSet<()>>> = HashMap::new();
        let a =to_ds("A");
        let b = to_ds("B");
        let c = to_ds("C");
        let d = to_ds("D");
        graph.insert(a.clone(), [(b.clone(), HashSet::new())].into_iter().collect());
        graph.insert(b.clone(), [(c.clone(), HashSet::new())].into_iter().collect());
        graph.insert(c.clone(), [(d.clone(), HashSet::new())].into_iter().collect());
        graph.insert(d.clone(), HashMap::new());

        let closure = transitive_closure(&graph);

        assert_eq!(
            closure.get(&a).unwrap(),
            &[(b.clone(), HashSet::new()),(c.clone(), HashSet::new()), (d.clone(), HashSet::new())].into_iter().collect()
        );
        assert_eq!(
            closure.get(&b).unwrap(),
            &[(c.clone(), HashSet::new()), (d.clone(), HashSet::new())].into_iter().collect()
        );
        assert_eq!(
            closure.get(&c).unwrap(),
            &[(d.clone(), HashSet::new())].into_iter().collect()
        );
        assert!(closure.get(&d).unwrap().is_empty());
    }
}