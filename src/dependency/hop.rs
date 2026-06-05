use crate::dependency::base::{build_top, DependencyBuilder, SymbolDependencyMap};
use crate::dependency::symbol::Term;
use crate::dependency::syntax_based::SyntaxBasedDependency;
use horned_owl::model::{AnnotatedComponent, AnnotationAssertion, AnnotationPropertyDomain, AnnotationPropertyRange, AsymmetricObjectProperty, ClassAssertion, ClassExpression, DataProperty, DataPropertyAssertion, DataPropertyDomain, DataPropertyRange, DataRange, DifferentIndividuals, DisjointDataProperties, DisjointObjectProperties, DisjointUnion, EquivalentDataProperties, ForIRI, FunctionalDataProperty, FunctionalObjectProperty, Individual, InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty, Literal, NegativeDataPropertyAssertion, NegativeObjectPropertyAssertion, ObjectPropertyAssertion, ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, ReflexiveObjectProperty, SameIndividual, SubAnnotationPropertyOf, SubDataPropertyOf, SymmetricObjectProperty, TransitiveObjectProperty};

use std::collections::HashSet;

pub struct HopDependency {}

impl<T: ForIRI> DependencyBuilder<T> for HopDependency {
    fn build_dependencies<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> SymbolDependencyMap<'a, T> {
        Self::derive_from_axioms(ontology_iter)
    }
}

impl<T: ForIRI> SyntaxBasedDependency<T> for HopDependency {

    fn dependency_from_disjoint_class_pair<'a>(
        _c1: &'a ClassExpression<T>,
        _c2: &'a ClassExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_union(
        _du: &DisjointUnion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
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

    fn dependency_from_disjoint_object_properties(
        _dops: &DisjointObjectProperties<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_inverse_object_properties(
        _iop: &InverseObjectProperties<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_functional_object_property(
        _fop: &FunctionalObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_inverse_functional_object_property(
        _ifop: &InverseFunctionalObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_reflexive_object_property(
        _rop: &ReflexiveObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_irreflexive_object_property(
        _irop: &IrreflexiveObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_symmetric_object_property(
        _sop: &SymmetricObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_asymmetric_object_property(
        _aop: &AsymmetricObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_transitive_object_property(
        _top: &TransitiveObjectProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_property_chain<'a>(_opes: Vec<&'a ObjectPropertyExpression<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependency_from_same_individual(_si: &SameIndividual<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_different_individuals(_di: &DifferentIndividuals<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_complement_of<'a>(_x: &'a ClassExpression<T>, _ce: &'a ClassExpression<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_one_of<'a>(_x: &'a ClassExpression<T>, _is: &'a Vec<Individual<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_all_values_from<'a>(_x: &'a ClassExpression<T>, _ope: &'a ObjectPropertyExpression<T>, _bce: &'a ClassExpression<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_has_value<'a>(_x: &'a ClassExpression<T>, _ope: &'a ObjectPropertyExpression<T>, _i: &'a Individual<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_has_self<'a>(_x: &'a ClassExpression<T>, _ope: &'a ObjectPropertyExpression<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_max_cardinality<'a>(_x: &'a ClassExpression<T>, _ope: &'a ObjectPropertyExpression<T>, _bce: &'a ClassExpression<T>, _n: &'a u32) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }
}
