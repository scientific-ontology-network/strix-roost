use crate::dependency::symbol::{Symbol, Term};
use horned_owl::model::ForIRI;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Compute the transitive closure of a directed graph.
/// Input: adjacency list as HashMap<T, HashSet<T>>.
/// Output: new adjacency list with reachability closure.
pub fn transitive_closure<'a, T: ForIRI, C: Eq + Hash + Clone>(
    graph: &HashMap<Term<'a, T>, HashMap<Term<'a, T>, HashSet<C>>>,
) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<C>>> {
    let mut closure: HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<C>>> = HashMap::new();
    let mut memo: HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<C>>> = HashMap::new();

    for node in graph.keys().filter(|t| t.is_atomic()) {
        let symb = node.get_symbol().unwrap();
        let reachable = dfs_with_memo(&node, graph, &mut memo);
        closure.insert(symb, reachable);
    }
    closure
}

/// DFS with memoization: only one SC per symbol
fn dfs_with_memo<'a, T: ForIRI, C: Eq + Hash + Clone>(
    start: &Term<T>,
    graph: &HashMap<Term<'a, T>, HashMap<Term<'a, T>, HashSet<C>>>,
    memo: &mut HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<C>>>,
) -> HashMap<Symbol<T>, HashSet<C>> {
    let start_symbol = start.get_symbol().unwrap();
    // If already memoized, return immediately
    if let Some(cached) = memo.get(&start_symbol) {
        return cached.clone();
    }

    let mut visited: HashMap<Term<T>, HashSet<C>> = HashMap::new();
    let mut stack = vec![(start.clone(), HashSet::new())];

    while let Some((sym, ax)) = stack.pop() {
        if visited.contains_key(&sym) {
            continue; // already visited this symbol
        }

        visited.insert(sym.clone(), ax.clone());

        // push neighbors
        if let Some(neighbors) = graph.get(&sym) {
            for (neigh_sym, neigh_ax) in neighbors {
                if !visited.contains_key(neigh_sym) {
                    stack.push((
                        neigh_sym.clone(),
                        neigh_ax.union(&ax).into_iter().cloned().collect(),
                    ));
                }
            }
        }
    }

    // Remove self if irreflexive closure is desired
    visited.remove(start);

    let cleaned: HashMap<Symbol<T>, HashSet<C>> = visited
        .into_iter()
        .filter(|(k, _ax)| k.is_atomic())
        .map(|(k, ax)| (k.get_symbol().unwrap(), ax))
        .collect();

    // Memoize before returning
    memo.insert(start_symbol, cleaned.clone());

    cleaned
}
