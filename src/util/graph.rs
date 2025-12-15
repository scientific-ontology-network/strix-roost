use std::collections::{HashMap, HashSet};
use horned_owl::model::{Component, ForIRI};
use itertools::Itertools;
use crate::dependency::symbol::{ForSymbol, OntologySymbol, SymbolContainer};

/// Compute the transitive closure of a directed graph.
/// Input: adjacency list as HashMap<T, HashSet<T>>.
/// Output: new adjacency list with reachability closure.
pub fn transitive_closure<S: ForSymbol + Clone + Eq + std::hash::Hash, C, SC: SymbolContainer<S, C> + Clone>(
    graph: &HashMap<S, HashSet<SC>>,
) -> HashMap<S, HashSet<SC>> {
    let mut closure: HashMap<S, HashSet<SC>> = HashMap::new();
    let mut memo: HashMap<S, HashSet<SC>> = HashMap::new();

    for node in graph.keys() {
        let reachable = dfs_with_memo(node, graph, &mut memo);
        closure.insert(node.clone(), reachable);
    }
    closure
}

/// DFS with memoization: only one SC per symbol
fn dfs_with_memo<S: ForSymbol, C, SC: SymbolContainer<S, C> + Clone>(
    start: &S,
    graph: &HashMap<S, HashSet<SC>>,
    memo: &mut HashMap<S, HashSet<SC>>,
) -> HashSet<SC> {
    // If already memoized, return immediately
    if let Some(cached) = memo.get(start) {
        return cached.clone();
    }

    let mut visited: HashMap<S, SC> = HashMap::new();
    let mut stack: Vec<SC> = vec![SC::from(start.clone())];

    while let Some(node) = stack.pop() {
        let sym = node.get_symbol();

        if visited.contains_key(sym) {
            continue; // already visited this symbol
        }

        visited.insert(sym.clone(), node.clone());

        // push neighbors
        if let Some(neighbors) = graph.get(sym) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor.get_symbol()) {
                    stack.push(neighbor.merge_include_information(&node));
                }
            }
        }
    }

    // Remove self if irreflexive closure is desired
    visited.remove(start);

    // Memoize before returning
    let result: HashSet<SC> = visited.values().cloned().collect();
    memo.insert(start.clone(), result.clone());

    result
}

#[cfg(test)]
mod tests {
    use horned_owl::model::ArcStr;
    use crate::dependency::symbol::{DependencySymbol, StringSymbol};
    use super::*;

    #[test]
    fn test_transitive_closure() {

        let to_ds = |s: &str| StringSymbol::new(s.to_string());
        let mut graph: HashMap<_, _> = HashMap::new();
        let a =to_ds("A");
        let b = to_ds("B");
        let c = to_ds("C");
        let d = to_ds("D");
        graph.insert(a.clone(), [b.clone()].into_iter().collect());
        graph.insert(b.clone(), [c.clone()].into_iter().collect());
        graph.insert(c.clone(), [d.clone()].into_iter().collect());
        graph.insert(d.clone(), HashSet::new());

        let closure = transitive_closure::<StringSymbol, (), StringSymbol>(&graph);

        assert_eq!(
            closure.get(&a).unwrap(),
            &[b.clone(), c.clone(), d.clone()].into_iter().collect()
        );
        assert_eq!(
            closure.get(&b).unwrap(),
            &[c.clone(), d.clone()].into_iter().collect()
        );
        assert_eq!(
            closure.get(&c).unwrap(),
            &[d.clone()].into_iter().collect()
        );
        assert!(closure.get(&d).unwrap().is_empty());
    }
}