use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotatedComponent, ClassExpression, Component, EquivalentClasses, EquivalentObjectProperties, ForIRI, SubClassOf, SubObjectPropertyExpression};
use crate::dependency::base::{DependencyBuilder, SyntaxBasedDependency, reduce_map};
use crate::dependency::symbol::{DependencyMap, ForSymbol, OntologySymbol, SymbolContainer};
use crate::util::graph::transitive_closure;

pub(crate) struct GrowthDependency {}


impl<'a, T: ForIRI + 'a> DependencyBuilder<'a, OntologySymbol<'a, T>, T> for GrowthDependency {
    fn build_dependencies<SC: SymbolContainer<OntologySymbol<'a, T>, &'a Component<T>>> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<OntologySymbol<'a, T>, SC>
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

impl<'a, T: ForIRI + 'a> SyntaxBasedDependency<'a, OntologySymbol<'a, T>, T> for GrowthDependency {
    fn dependency_from_subsumption(_sco: &'a SubClassOf<T>) -> HashSet<(OntologySymbol<'a, T>,OntologySymbol<'a, T>)> {
        let a: HashSet<_> = [(OntologySymbol::CE(&_sco.sub), OntologySymbol::CE(&_sco.sup))].into();
        let b: HashSet<_> = Self::dependencies_from_class_expression(&_sco.sub);
        let c: HashSet<_> = Self::dependencies_from_class_expression(&_sco.sup);
        a.into_iter().chain(b.into_iter().chain(c)).collect()
    }

    fn dependencies_from_class_expression(ce: &'a ClassExpression<T>) -> HashSet<(OntologySymbol<'a, T>,OntologySymbol<'a, T>)> {
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

fn remove_targets<'a, S: ForSymbol, T: ForIRI + 'a, SC: SymbolContainer<S, &'a Component<T>>>(dep_map: &DependencyMap<S, SC>, sup_map: HashMap<S, HashSet<SC>>) -> DependencyMap<S, SC> where {
    let mut new_map = HashMap::new();
    for (k, v) in dep_map.iter() {
        let supers_of_classes_in_v = v.into_iter().filter_map(|x| sup_map.get(x.get_symbol())).flatten().collect::<HashSet<_>>();
        let supers_of_k = match sup_map.get(k) {
            None => HashSet::new(),
            Some(k_supers) => k_supers.iter().map(|x| x).collect()
        };
        let irrelevant_dependencies: HashSet<_> = supers_of_classes_in_v.union(&supers_of_k).map(|v| *v).collect();
        let relevant_dependencies: HashSet<_> = v.iter().filter(|x| !irrelevant_dependencies.contains(x)).collect();
        new_map.insert(k.clone(), relevant_dependencies.iter().map(|v| (**v).clone()).collect());
    }
    new_map
}

pub fn remove_super_expressions<'a, S: ForSymbol, T: ForIRI, SC: SymbolContainer<S, &'a Component<T>>>(dep_map: DependencyMap<S, SC>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>, transform_symbol: fn(&OntologySymbol<'a,T>)->S) -> DependencyMap<S, SC>
where
    T: 'a,
{
    let sup_map = transitive_closure(&build_super_map(ontology_iter, transform_symbol));
    remove_targets(&dep_map, sup_map)
}


pub fn remove_super_symbols<'a, S: ForSymbol, T: ForIRI, SC: SymbolContainer<S, &'a Component<T>>>(dep_map: &HashMap<S, HashSet<SC>>, ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>, transform_symbol: fn(&OntologySymbol<'a,T>)->S) -> DependencyMap<S, SC>
where
    T: 'a,
{
    let sup_map = reduce_map(&transitive_closure(&build_super_map(ontology_iter,transform_symbol)));
    remove_targets(&dep_map, sup_map)
}

fn build_super_map<'a, 'b: 'a, S: ForSymbol, T: ForIRI + 'b, SC: SymbolContainer<S, &'a Component<T>>>(
        ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>, transform_symbol: fn(&OntologySymbol<'a,T>)->S
    ) -> HashMap<S, HashSet<SC>>
    where
    {
        let mut sup_map = HashMap::new();
        for ax in ontology_iter {
            match &ax.component {
                Component::SubClassOf(sco) => {
                    sup_map.entry(transform_symbol(&OntologySymbol::CE(&sco.sub))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::CE(&sco.sup)), &ax.component));
                }
                Component::EquivalentClasses(EquivalentClasses(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(transform_symbol(&OntologySymbol::CE(a))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::CE(b)), &ax.component));
                                sup_map.entry(transform_symbol(&OntologySymbol::CE(b))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::CE(a)), &ax.component));
                            }
                        }
                    }
                }
                Component::SubObjectPropertyOf(sco) => {
                    match &sco.sub {
                        SubObjectPropertyExpression::ObjectPropertyChain(_) => {}
                        SubObjectPropertyExpression::ObjectPropertyExpression(ope) => { sup_map.entry(transform_symbol(&OntologySymbol::Role(ope))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::Role(&sco.sup)), &ax.component)); }
                    }
                }
                Component::EquivalentObjectProperties(EquivalentObjectProperties(ecs)) => {
                    for a in ecs {
                        for b in ecs {
                            if a != b
                            {
                                sup_map.entry(transform_symbol(&OntologySymbol::Role(a))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::Role(b)), &ax.component));
                                sup_map.entry(transform_symbol(&OntologySymbol::Role(b))).or_insert(HashSet::new()).insert(SC::from_symbol_and_axiom(transform_symbol(&OntologySymbol::Role(a)), &ax.component));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        sup_map
    }