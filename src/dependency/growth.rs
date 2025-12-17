use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use core::cmp::Eq;
use horned_owl::model::{AnnotatedComponent, ClassExpression, Component, EquivalentClasses, EquivalentObjectProperties, ForIRI, SubClassOf, SubObjectPropertyExpression};
use crate::dependency::base::{DependencyBuilder, SyntaxBasedDependency, reduce_map, ComplexDependencyMap, DependencyMap};
use crate::dependency::symbol::{Term, Symbol};
use crate::util::graph::transitive_closure;

pub struct GrowthDependency {}


impl<T: ForIRI> DependencyBuilder<T> for GrowthDependency {
    fn build_dependencies<'a> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>>
    {
        let mut map = HashMap::new();
        for (a,b,c) in Self::dependencies_from_components(ontology_iter){
            if !map.contains_key(&a) {
                map.insert(a.clone(), HashMap::new());
            }
            map.get_mut(&a).unwrap().insert(b,c);
        }
        reduce_map(&transitive_closure(&map))
    }
}

impl<T: ForIRI> SyntaxBasedDependency<T> for GrowthDependency {
    fn dependency_from_subsumption(_sco: &SubClassOf<T>) -> HashSet<(Term<T>, Term<T>)> {
        let a: HashSet<_> = HashSet::from_iter([(Term::CE(&_sco.sub), Term::CE(&_sco.sup))]);
        let b: HashSet<_> = Self::dependencies_from_class_expression(&_sco.sub);
        let c: HashSet<_> = Self::dependencies_from_class_expression(&_sco.sup);
        a.into_iter().chain(b.into_iter().chain(c)).collect()
    }

    fn dependencies_from_class_expression(ce: &ClassExpression<T>) -> HashSet<(Term<T>, Term<T>)> {
        match ce {
            ClassExpression::ObjectIntersectionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(Term::CE(ce), Term::CE(ce2))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectUnionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(Term::CE(ce2), Term::CE(ce))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => [
                (Term::CE(ce), Term::CE(bce)),
                (Term::CE(ce), Term::Role(ope)),
            ]
                .into_iter()
                .chain(Self::dependencies_from_class_expression(bce))
                .collect(),
            _ => HashSet::new(),
        }
    }
}

fn remove_targets<'a, S: Hash + Eq + Clone, C: Clone>(dep_map: &HashMap<S, HashMap<S, C>>, sup_map: &HashMap<S, HashMap<S, C>>) -> HashMap<S, HashMap<S, C>> where {
    let mut new_map = HashMap::new();
    for (k, v) in dep_map.iter() {
        let supers_of_classes_in_v: HashSet<&S> = v.keys().filter_map(|x| sup_map.get(x)).map(|x| x.keys()).flatten().collect();
        let supers_of_k = match sup_map.get(k) {
            None => HashSet::new(),
            Some(k_supers) => k_supers.keys().map(|x| x).collect()
        };
        let irrelevant_dependencies: HashSet<&S> = supers_of_classes_in_v.union(&supers_of_k).map(|v| *v).collect();
        let relevant_dependencies: HashMap<&S,&C> = v.iter().filter(|(x,c)| !irrelevant_dependencies.contains(x)).collect();
        let rd: HashMap<S,C> = relevant_dependencies.iter().map(|(&s,&c)| (s.clone(), c.clone())).collect();
        new_map.insert(k.clone(), rd);
    }
    new_map
}

pub fn remove_super_expressions<'a, T: ForIRI>(dep_map: &ComplexDependencyMap<'a, T, HashSet<&'a Component<T>>>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> ComplexDependencyMap<'a, T, HashSet<&'a Component<T>>>
where
    T: 'a,
{
    let sup_map: ComplexDependencyMap<'a, T, HashSet<&'a Component<T>>> = transitive_closure(&build_super_map(ontology_iter));
    remove_targets(&dep_map, &sup_map)
}


pub fn remove_super_symbols<'a, T: ForIRI>(dep_map: &DependencyMap<T, HashSet<&'a Component<T>>>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> DependencyMap<T, HashSet<&'a Component<T>>>
where
    T: 'a,
{
    let sup_map: DependencyMap<T, HashSet<&'a Component<T>>> = reduce_map(&transitive_closure(&build_super_map(ontology_iter)));
    remove_targets(&dep_map, &sup_map)
}

fn build_super_map<'a, T: ForIRI>(ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> ComplexDependencyMap<'a, T, HashSet<&'a Component<T>>>
    where
    {
        let mut sup_map = HashMap::new();
        for ax in ontology_iter {
            match &ax.component {
                Component::SubClassOf(sco) => {
                    sup_map.entry(Term::CE(&sco.sub)).or_insert(HashMap::new()).insert(Term::CE(&sco.sup), [&ax.component].into());
                }
                Component::EquivalentClasses(EquivalentClasses(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(Term::CE(a)).or_insert(HashMap::new()).insert(Term::CE(b), [&ax.component].into());
                                sup_map.entry(Term::CE(b)).or_insert(HashMap::new()).insert(Term::CE(a), [&ax.component].into());
                            }
                        }
                    }
                }
                Component::SubObjectPropertyOf(sco) => {
                    match &sco.sub {
                        SubObjectPropertyExpression::ObjectPropertyChain(_) => {}
                        SubObjectPropertyExpression::ObjectPropertyExpression(ope) => { sup_map.entry(Term::Role(ope)).or_insert(HashMap::new()).insert(Term::Role(&sco.sup), [&ax.component].into()); }
                    }
                }
                Component::EquivalentObjectProperties(EquivalentObjectProperties(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(Term::Role(a)).or_insert(HashMap::new()).insert(Term::Role(b), [&ax.component].into());
                                sup_map.entry(Term::Role(b)).or_insert(HashMap::new()).insert(Term::Role(a), [&ax.component].into());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        sup_map
    }

pub fn invert_map<S: Hash + Eq + Clone, C: Clone>(map: &HashMap<S, HashMap<S, C>>) -> HashMap<S, HashMap<S, C>> {
    let mut new_map: HashMap<S, HashMap<S,C>> = HashMap::new();
    for (k,vset) in map {
        for (v,c) in vset {
            if !new_map.contains_key(v) {
                new_map.insert(v.clone(), HashMap::new());
            }
            let l = new_map.get_mut(v).unwrap();
            l.insert(k.clone(), c.clone());
        }
    }
    new_map
}