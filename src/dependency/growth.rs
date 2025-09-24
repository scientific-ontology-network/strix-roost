use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotatedComponent, ClassExpression, ForIRI, SubClassOf};
use crate::dependency::base::{DependencyPair, DependencyMap, DependencyBuilder, OntologySymbol, SyntaxBasedDependency};
use crate::util::graph::transitive_closure;
pub struct GrowthDependency;

impl<T: ForIRI> DependencyBuilder<T> for GrowthDependency {
    fn dep<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<'a, T> {
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
            vec![(OntologySymbol::CE(&_sco.sub), OntologySymbol::CE(&_sco.sup))];
        let b: Vec<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sub);
        let c: Vec<DependencyPair<T>> = Self::dependencies_from_class_expression(&_sco.sup);
        a.into_iter().chain(b.into_iter().chain(c)).collect()
    }

    fn dependencies_from_class_expression(ce: &ClassExpression<T>) -> Vec<DependencyPair<T>> {
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
            _ => Vec::new(),
        }
    }
}
