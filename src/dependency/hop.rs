use crate::dependency::base::{build_top, DependencyBuilder};
use crate::dependency::symbol::{Symbol, Term};
use crate::dependency::syntax_based::SyntaxBasedDependency;
use horned_owl::model::{
    AnnotatedComponent, ClassExpression, Component, ForIRI, ObjectPropertyDomain,
    ObjectPropertyExpression, ObjectPropertyRange,
};
use std::collections::{HashMap, HashSet};

pub struct HopDependency {}

impl<T: ForIRI> DependencyBuilder<T> for HopDependency {
    fn build_dependencies<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>> {
        Self::derive_from_axioms(ontology_iter)
    }
}

impl<T: ForIRI> SyntaxBasedDependency<T> for HopDependency {
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
        let base = match *bce == build_top() {
            true => {
                vec![(Term::Role(ope), Term::CE(x))]
            }
            false => {
                vec![]
            }
        };
        [(Term::CE(x), Term::CE(bce)), (Term::CE(x), Term::Role(ope))]
            .into_iter()
            .chain(base)
            .chain(Self::dependencies_from_class_expression(bce))
            .chain(Self::dependencies_from_object_property_expression(ope))
            .collect()
    }

    // domain(r) <= C
    fn dependency_from_object_property_domain(
        opd: &ObjectPropertyDomain<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        ([(Term::Role(&opd.ope), Term::CE(&opd.ce))]) // r -> X, X -> C
            .into_iter()
            .chain(Self::dependencies_from_class_expression(&opd.ce))
            .chain(Self::dependencies_from_object_property_expression(&opd.ope))
            .collect()
    }

    // range(r) <= C
    fn dependency_from_object_property_range(
        _opr: &ObjectPropertyRange<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        [
            (Term::Role(&_opr.ope), Term::CE(&_opr.ce)), // r -> X, X -> C
        ]
        .into_iter()
        .chain(Self::dependencies_from_class_expression(&_opr.ce))
        .chain(Self::dependencies_from_object_property_expression(
            &_opr.ope,
        ))
        .collect()
    }
}
