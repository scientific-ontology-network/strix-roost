use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use horned_owl::model::{AnnotatedComponent, ClassExpression, Component, EquivalentClasses, EquivalentObjectProperties, ForIRI, SubClassOf, SubObjectPropertyExpression};
use crate::dependency::base::{DependencyPair, DependencyMap, DependencyBuilder, OntologySymbol, SyntaxBasedDependency, reduce_map};
use crate::util::graph::transitive_closure;
pub struct GrowthDependency;

impl GrowthDependency {
    fn build_super_map<'a, T: ForIRI>(
        ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>,
    ) -> HashMap<OntologySymbol<'a, T>, HashSet<OntologySymbol<'a, T>>>
    where
    {
        let mut sup_map = DependencyMap::new();
        for ax in ontology_iter {
            match &ax.component {
                Component::SubClassOf(sco) => {
                    sup_map.entry(OntologySymbol::CE(&sco.sub)).or_insert(HashSet::new()).insert(OntologySymbol::CE(&sco.sup));
                }
                Component::EquivalentClasses(EquivalentClasses(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(OntologySymbol::CE(a)).or_insert(HashSet::new()).insert(OntologySymbol::CE(b));
                                sup_map.entry(OntologySymbol::CE(b)).or_insert(HashSet::new()).insert(OntologySymbol::CE(a));
                            }
                        }
                    }
                }
                Component::SubObjectPropertyOf(sco) => {
                    match &sco.sub {
                        SubObjectPropertyExpression::ObjectPropertyChain(_) => {}
                        SubObjectPropertyExpression::ObjectPropertyExpression(ope) => { sup_map.entry(OntologySymbol::Role(ope)).or_insert(HashSet::new()).insert(OntologySymbol::Role(&sco.sup)); }
                    }
                }
                Component::EquivalentObjectProperties(EquivalentObjectProperties(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(OntologySymbol::Role(a)).or_insert(HashSet::new()).insert(OntologySymbol::Role(b));
                                sup_map.entry(OntologySymbol::Role(b)).or_insert(HashSet::new()).insert(OntologySymbol::Role(a));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        sup_map
    }

    fn remove_targets<S: Hash + Eq + Clone + Debug>(dep_map: &HashMap<S, HashSet<S>>, sup_map: HashMap<S, HashSet<S>>) -> HashMap<S, HashSet<S>> {
        let mut new_map = HashMap::new();
        for (k, v) in dep_map.iter() {
            let supers_of_classes_in_v = v.into_iter().filter_map(|x| sup_map.get(x)).flatten().collect::<HashSet<_>>();
            let supers_of_k = match sup_map.get(k) {
                None => HashSet::new(),
                Some(k_supers) => k_supers.iter().map(|x| x).collect()
            };
            let irrelevant_dependencies = supers_of_classes_in_v.union(&supers_of_k).map(|v| *v).collect();
            let v_set: HashSet<_> = v.iter().map(|x| x).collect();
            let relevant_dependencies: HashSet<_> = v_set.difference(&irrelevant_dependencies).collect();
            new_map.insert(k.clone(), relevant_dependencies.iter().map(|v| (***v).clone()).collect());
        }
        new_map
    }

    pub fn remove_super_expressions<'a, T: ForIRI>(dep_map: DependencyMap<'a, T>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> DependencyMap<'a, T>
    where
        T: 'a,
    {
        let sup_map = transitive_closure(&Self::build_super_map(ontology_iter));
        Self::remove_targets(&dep_map, sup_map)
    }


    pub fn remove_super_symbols<'a, T: ForIRI>(dep_map: &HashMap<T, HashSet<T>>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> HashMap<T, HashSet<T>>
    where
        T: 'a,
    {
        let sup_map = reduce_map(&transitive_closure(&Self::build_super_map(ontology_iter)));
        Self::remove_targets(&dep_map, sup_map)
    }
}

impl<'a, T: ForIRI> DependencyBuilder<'a, T> for GrowthDependency {
    fn build_dependencies(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<'a, T>
    {
        let mut map = DependencyMap::new();
        for (a,b) in Self::dependencies_from_components(ontology_iter){
            if !map.contains_key(&a) {
                map.insert(a.clone(), HashSet::new());
            }
            map.get_mut(&a).unwrap().insert(b);
        }
        transitive_closure(&map)
    }
}

impl<'a, T: ForIRI> SyntaxBasedDependency<'a, T> for GrowthDependency {
    fn dependency_from_subsumption(_sco: &SubClassOf<T>) -> HashSet<DependencyPair<T>> {
        let a: HashSet<DependencyPair<T>> = [(OntologySymbol::CE(&_sco.sub), OntologySymbol::CE(&_sco.sup))].into();
        let b: HashSet<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sub);
        let c: HashSet<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sup);
        a.into_iter().chain(b.into_iter().chain(c)).collect()
    }

    fn dependencies_from_class_expression(ce: &ClassExpression<T>) -> HashSet<DependencyPair<T>> {
        match ce {
            ClassExpression::ObjectIntersectionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(OntologySymbol::CE(ce), OntologySymbol::CE(ce2))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectUnionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(OntologySymbol::CE(ce2), OntologySymbol::CE(ce))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => [
                (OntologySymbol::CE(ce), OntologySymbol::CE(bce)),
                (OntologySymbol::CE(ce), OntologySymbol::Role(ope)),
            ]
                .into_iter()
                .chain(Self::dependencies_from_class_expression(bce))
                .collect(),
            _ => HashSet::new(),
        }
    }
}
