use crate::dependency::base::{ComplexDependencyMap, DependencyBuilder, DependencyMap};
use crate::dependency::symbol::{Symbol, Term};
use crate::dependency::syntax_based::{reduce_map, SyntaxBasedDependency};
use crate::util::graph::transitive_closure;
use core::cmp::Eq;
use horned_owl::model::{
    AnnotatedComponent, ClassExpression, Component, EquivalentClasses, EquivalentObjectProperties,
    ForIRI, ObjectPropertyExpression, ObjectPropertyRange, SubObjectPropertyExpression,
};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct GrowthDependency {}

impl<T: ForIRI> DependencyBuilder<T> for GrowthDependency {
    fn build_dependencies<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>> {
        Self::derive_from_axioms(ontology_iter)
    }
}

impl<T: ForIRI> SyntaxBasedDependency<T> for GrowthDependency {
    fn dependencies_from_object_intersection_of<'a>(
        x: &'a ClassExpression<T>,
        ces: &'a Vec<ClassExpression<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(x), Term::CE(ce2))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_union_of<'a>(
        x: &'a ClassExpression<T>,
        ces: &'a Vec<ClassExpression<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(ce2), Term::CE(x))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_some_values_from<'a>(
        x: &'a ClassExpression<T>,
        ope: &'a ObjectPropertyExpression<T>,
        bce: &'a ClassExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        [(Term::CE(x), Term::CE(bce)), (Term::CE(x), Term::Role(ope))]
            .into_iter()
            .chain(Self::dependencies_from_class_expression(bce))
            .collect()
    }

    fn dependencies_from_object_property_expression(
        ope: &ObjectPropertyExpression<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(_op) => HashSet::new(),
            ObjectPropertyExpression::InverseObjectProperty(_op) => {
                println!("Inverse object properties are not supported in syntactic emptiness dependency yet. Skipping!");
                HashSet::new()
            }
        }
    }

    // range(r) <= C
    fn dependency_from_object_property_range(
        _opr: &ObjectPropertyRange<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        // X -> r and X -> C, but not r -> C or C-> r
        HashSet::new()
    }
}
