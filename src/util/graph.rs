use std::collections::{HashMap, HashSet};

/// Compute the transitive closure of a directed graph.
/// Input: adjacency list as HashMap<T, HashSet<T>>.
/// Output: new adjacency list with reachability closure.
pub fn transitive_closure<T>(graph: &HashMap<T, HashSet<T>>) -> HashMap<T, HashSet<T>>
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut closure: HashMap<T, HashSet<T>> = HashMap::new();
    let mut memo: HashMap<T, HashSet<T>> = HashMap::new();

    for node in graph.keys() {
        let reachable = dfs_with_memo(node, graph, &mut memo);
        closure.insert(node.clone(), reachable);
    }

    closure
}

/// Perform DFS with memoization to compute all reachable nodes.
fn dfs_with_memo<T>(
    start: &T,
    graph: &HashMap<T, HashSet<T>>,
    memo: &mut HashMap<T, HashSet<T>>,
) -> HashSet<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    if let Some(cached) = memo.get(start) {
        return cached.clone();
    }

    let mut visited: HashSet<T> = HashSet::new();
    let mut stack: Vec<T> = vec![start.clone()];

    while let Some(node) = stack.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    stack.push(neighbor.clone());
                }
            }
        }
    }

    visited.remove(start); // remove self unless you want reflexive closure
    memo.insert(start.clone(), visited.clone());
    visited
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