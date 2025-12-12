use std::collections::{HashMap, HashSet};
use horned_owl::model::{Component, ForIRI};
use itertools::Itertools;
use crate::dependency::symbol::{ForSymbol, OntologySymbol, SymbolContainer};

/// Compute the transitive closure of a directed graph.
/// Input: adjacency list as HashMap<T, HashSet<T>>.
/// Output: new adjacency list with reachability closure.
pub fn transitive_closure<'a, S: ForSymbol, T:ForIRI + 'a, SC: SymbolContainer<S, &'a Component<T>>>(graph: &HashMap<S, HashSet<SC>>) -> HashMap<S, HashSet<SC>>
where
{
    let mut closure: HashMap<S, HashSet<SC>> = HashMap::new();
    let mut memo: HashMap<S, HashSet<SC>> = HashMap::new();

    for node in graph.keys() {
        let reachable = dfs_with_memo(node, graph, &mut memo);
        closure.insert(node.clone(), reachable);
    }
    closure
}

/// Perform DFS with memoization to compute all reachable nodes.
fn dfs_with_memo<'a, S: ForSymbol, T: ForIRI + 'a, SC: SymbolContainer<S, &'a Component<T>>>(
    start: &S,
    graph: &HashMap<S, HashSet<SC>>,
    memo: &mut HashMap<S, HashSet<SC>>,
) -> HashSet<SC>
{
    let mut visited: HashMap<S, SC> = HashMap::new();
    let mut stack: Vec<SC> = vec![SC::from(start.clone())];

    while let Some(node) = stack.pop() {

        if let Some(cached) = memo.get(&node.get_symbol()) {
            for c in cached {
                if !visited.contains_key(c.get_symbol()){
                    visited.insert(c.get_symbol().clone(), node.merge_include_information(c));
                }
            }
        } else {
            if !visited.contains_key(node.get_symbol()) {
                continue;
            }
            if let Some(neighbors) = graph.get(&node.get_symbol()) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor.get_symbol()) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }
    }

    visited.remove(start); // remove self unless you want reflexive closure
    memo.insert(start.clone(), visited.values().cloned().collect());
    visited.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transitive_closure() {
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
        graph.insert("A", ["B"].into_iter().collect());
        graph.insert("B", ["C"].into_iter().collect());
        graph.insert("C", HashSet::new());

        let closure = transitive_closure(&graph);

        assert_eq!(
            closure.get("A").unwrap(),
            &["B", "C"].into_iter().collect()
        );
        assert_eq!(
            closure.get("B").unwrap(),
            &["C"].into_iter().collect()
        );
        assert!(closure.get("C").unwrap().is_empty());
    }
}