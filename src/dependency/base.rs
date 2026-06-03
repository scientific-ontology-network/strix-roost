//! This module provides functionality for building and managing dependency relationships in ontologies.
//! It defines structures and traits for representing and analyzing relationships between different
//! ontological components such as class and property symbols.

use crate::dependency::symbol::{Symbol, Term};
use crate::util::graph::transitive_closure;
use horned_owl::model::*;
use horned_owl::vocab::OWL;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub type TermDependencyPair<'a, T> = (Term<'a, T>, Term<'a, T>, HashSet<Vec<&'a Component<T>>>);
pub type TermDependencyMap<'a, T> = HashMap<Term<'a, T>, HashMap<Term<'a, T>, HashSet<Vec<&'a Component<T>>>>>;
pub type SymbolDependencyMap<'a, T> = HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<Vec<&'a Component<T>>>>>;

/// Trait for building dependency relationships between ontological components
pub trait DependencyBuilder<T: ForIRI> {
    /// Constructs a dependency map from an iterator of annotated components
    ///
    /// # Arguments
    /// * `ontology_iter` - An iterator over annotated ontology components
    fn build_dependencies<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> SymbolDependencyMap<'a, T>
    where
        T: 'a;
}

pub fn remove_targets<'a, S: Hash + Eq + Clone, C: Clone>(
    dep_map: &HashMap<S, HashMap<S, C>>,
    sup_map: &HashMap<S, HashMap<S, C>>,
) -> HashMap<S, HashMap<S, C>> where {
    let mut new_map = HashMap::new();
    for (k, v) in dep_map.iter() {
        let supers_of_classes_in_v: HashSet<&S> = v
            .keys()
            .filter_map(|x| sup_map.get(x))
            .map(|x| x.keys())
            .flatten()
            .collect();
        let supers_of_k = match sup_map.get(k) {
            None => HashSet::new(),
            Some(k_supers) => k_supers.keys().map(|x| x).collect(),
        };
        let irrelevant_dependencies: HashSet<&S> = supers_of_classes_in_v
            .union(&supers_of_k)
            .map(|v| *v)
            .collect();
        let relevant_dependencies: HashMap<&S, &C> = v
            .iter()
            .filter(|(x, _c)| !irrelevant_dependencies.contains(x))
            .collect();
        let rd: HashMap<S, C> = relevant_dependencies
            .iter()
            .map(|(&s, &c)| (s.clone(), c.clone()))
            .collect();
        new_map.insert(k.clone(), rd);
    }
    new_map
}

pub fn remove_super_symbols<'a, T: ForIRI>(
    dep_map: &SymbolDependencyMap<'a, T>,
    ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
) -> SymbolDependencyMap<'a, T>
where
    T: 'a,
{
    let deps = build_super_map(ontology_iter);
    let sup_map = transitive_closure(deps, 0);
    remove_targets(&dep_map, &sup_map)
}

fn build_super_map<'a, T: ForIRI>(
    ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
) -> TermDependencyMap<'a, T> {
    let mut sup_map = HashMap::new();
    for ax in ontology_iter {
        match &ax.component {
            Component::SubClassOf(sco) => {
                sup_map
                    .entry(Term::CE(&sco.sub))
                    .or_insert(HashMap::new())
                    .insert(Term::CE(&sco.sup), [[&ax.component].into()].into());
            }
            Component::EquivalentClasses(EquivalentClasses(ecs)) => {
                for a in ecs {
                    for b in ecs {
                        if a != b {
                            sup_map
                                .entry(Term::CE(a))
                                .or_insert(HashMap::new())
                                .insert(Term::CE(b), [[&ax.component].into()].into());
                            sup_map
                                .entry(Term::CE(b))
                                .or_insert(HashMap::new())
                                .insert(Term::CE(a), [[&ax.component].into()].into());
                        }
                    }
                }
            }
            Component::SubObjectPropertyOf(sco) => match &sco.sub {
                SubObjectPropertyExpression::ObjectPropertyChain(_) => {}
                SubObjectPropertyExpression::ObjectPropertyExpression(ope) => {
                    sup_map
                        .entry(Term::Role(ope))
                        .or_insert(HashMap::new())
                        .insert(Term::Role(&sco.sup), [[&ax.component].into()].into());
                }
            },
            Component::EquivalentObjectProperties(EquivalentObjectProperties(ecs)) => {
                for a in ecs {
                    for b in ecs {
                        if a != b {
                            sup_map
                                .entry(Term::Role(a))
                                .or_insert(HashMap::new())
                                .insert(Term::Role(b), [[&ax.component].into()].into());
                            sup_map
                                .entry(Term::Role(b))
                                .or_insert(HashMap::new())
                                .insert(Term::Role(a), [[&ax.component].into()].into());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    sup_map
}

pub fn invert_map<S: Hash + Eq + Clone, C: Clone>(
    map: &HashMap<S, HashMap<S, C>>,
) -> HashMap<S, HashMap<S, C>> {
    let mut new_map: HashMap<S, HashMap<S, C>> = HashMap::new();
    for (k, vset) in map {
        for (v, c) in vset {
            if !new_map.contains_key(v) {
                new_map.insert(v.clone(), HashMap::new());
            }
            let l = new_map.get_mut(v).unwrap();
            l.insert(k.clone(), c.clone());
        }
    }
    new_map
}

pub fn build_top<T: ForIRI>() -> ClassExpression<T> {
    let builder = Build::<T>::new();
    ClassExpression::Class(Class(builder.iri(OWL::Thing)))
}

pub fn build_bottom<T: ForIRI>() -> ClassExpression<T> {
    let builder = Build::<T>::new();
    ClassExpression::Class(Class(builder.iri(OWL::Nothing)))
}
