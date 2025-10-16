use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use horned_owl::model::{AnnotatedComponent, ClassExpression, Component, EquivalentClasses, EquivalentObjectProperties, ForIRI, SubClassOf, SubObjectPropertyExpression};
use itertools::cloned;
use crate::dependency::base::{DependencyPair, DependencyMap, DependencyBuilder, OntologySymbol, SyntaxBasedDependency};
use crate::util::graph::transitive_closure;
pub struct GrowthDependency;

impl GrowthDependency {

    fn build_super_map<'a, T: ForIRI>(
        ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>,
    ) -> HashMap<OntologySymbol<T>, HashSet<OntologySymbol<T>>>
    where T: 'a {
        let mut sup_map = DependencyMap::new();
        for ax in ontology_iter {
            match &ax.component {
                Component::SubClassOf(sco) => {
                    sup_map.entry(OntologySymbol::CE(sco.sub.clone())).or_insert(HashSet::new()).insert(OntologySymbol::CE(sco.sup.clone()));
                }
                Component::EquivalentClasses(EquivalentClasses(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if (a != b)
                            {
                                sup_map.entry(OntologySymbol::CE(a.clone())).or_insert(HashSet::new()).insert(OntologySymbol::CE(b.clone()));
                                sup_map.entry(OntologySymbol::CE(b.clone())).or_insert(HashSet::new()).insert(OntologySymbol::CE(a.clone()));
                            }
                        }
                    }
                }
                Component::SubObjectPropertyOf(sco) => {
                    match &sco.sub {
                        SubObjectPropertyExpression::ObjectPropertyChain(_) => {}
                        SubObjectPropertyExpression::ObjectPropertyExpression(ope) => {sup_map.entry(OntologySymbol::Role(ope.clone())).or_insert(HashSet::new()).insert(OntologySymbol::Role(sco.sup.clone()));}
                    }

                }
                Component::EquivalentObjectProperties(EquivalentObjectProperties(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if (a != b)
                            {
                                sup_map.entry(OntologySymbol::Role(a.clone())).or_insert(HashSet::new()).insert(OntologySymbol::Role(b.clone()));
                                sup_map.entry(OntologySymbol::Role(b.clone())).or_insert(HashSet::new()).insert(OntologySymbol::Role(a.clone()));
                            }
                        }
                    }

                }
                _ => {}
            }
        }
        sup_map
    }

    pub fn remove_supers<'a, T: ForIRI> (dep_map: DependencyMap<T>, ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>) -> DependencyMap<T> where T: 'a {
        let sup_map = transitive_closure(&Self::build_super_map(ontology_iter));
        let supers = |depends_on: &HashSet<OntologySymbol<T>>| depends_on.into_iter().filter_map(|v|sup_map.get(v)).flatten().cloned().collect::<HashSet<OntologySymbol<T>>>();
        HashMap::from_iter(dep_map.iter().map(|(k,v)| (k.clone(), v.difference(&supers(v).union(sup_map.get(k).unwrap_or(&HashSet::new())).cloned().collect()).cloned().collect())))
    }
}

impl<T: ForIRI> DependencyBuilder<T> for GrowthDependency {
    fn build_dependencies<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<T>
    where T: 'a
    {
        let mut map = DependencyMap::new();
        for (a,b) in Self::dependencies_from_components(ontology_iter){
            map.entry(a).or_insert(HashSet::new()).insert(b);
        }
        transitive_closure(&map)
    }
}

impl<T: ForIRI> SyntaxBasedDependency<T> for GrowthDependency {
    fn dependency_from_subsumption(_sco: &SubClassOf<T>) -> Vec<DependencyPair<T>> {
        let a: Vec<DependencyPair<T>> =
            vec![(OntologySymbol::CE(_sco.sub.clone()), OntologySymbol::CE(_sco.sup.clone()))];
        let b: Vec<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sub);
        let c: Vec<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sup);
        a.into_iter().chain(b.into_iter().chain(c)).collect()
    }

    fn dependencies_from_class_expression(ce: &ClassExpression<T>) -> Vec<DependencyPair<T>> {
        match ce {
            ClassExpression::ObjectIntersectionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(OntologySymbol::CE(ce.clone()), OntologySymbol::CE(ce2.clone()))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectUnionOf(exprs) => exprs
                .into_iter()
                .flat_map(|ce2| {
                    [(OntologySymbol::CE(ce2.clone()), OntologySymbol::CE(ce.clone()))]
                        .into_iter()
                        .chain(Self::dependencies_from_class_expression(ce2))
                })
                .collect(),
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => [
                (OntologySymbol::CE(ce.clone()), OntologySymbol::CE(*bce.clone())),
                (OntologySymbol::CE(ce.clone()), OntologySymbol::Role(ope.clone())),
            ]
                .into_iter()
                .chain(Self::dependencies_from_class_expression(bce))
                .collect(),
            _ => Vec::new(),
        }
    }
}
