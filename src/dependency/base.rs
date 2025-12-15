//! This module provides functionality for building and managing dependency relationships in ontologies.
//! It defines structures and traits for representing and analyzing relationships between different
//! ontological components such as class and property symbols.

use std::collections::{HashMap, HashSet};
use horned_owl::model::SubClassOf as SCO;
use horned_owl::model::*;
use itertools::Itertools;
use crate::dependency::symbol::{DependencyMap, ForSymbol, SymbolContainer};


/// Trait for building dependency relationships between ontological components
pub trait DependencyBuilder<'a, S: ForSymbol, T:ForIRI + 'a> {
    /// Constructs a dependency map from an iterator of annotated components
    ///
    /// # Arguments
    /// * `ontology_iter` - An iterator over annotated ontology components
    fn build_dependencies<SC: SymbolContainer<S, Vec<&'a Component<T>>>> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<S, SC>;
}

/// Trait for analyzing syntax-based dependencies in ontological components
pub trait SyntaxBasedDependency<'a, S: ForSymbol, T:ForIRI + 'a>: DependencyBuilder<'a, S, T> {
    /// Extracts dependency pairs from ontology components based on their syntactic structure
    ///
    /// # Arguments
    /// * `ontology_iter` - An iterator over annotated ontology components
    ///
    /// # Returns
    /// A vector of dependency pairs representing relationships between ontological elements
    fn dependencies_from_components<SC: SymbolContainer<S, Vec<&'a Component<T>>>>
    (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashSet<(S, SC)> where {
        ontology_iter
            .flat_map(|ce| (match &ce.component {
                Component::SubClassOf(ref sco) => Self::dependency_from_subsumption(sco),
                Component::EquivalentClasses(ref ecs) => Self::dependency_from_equivalences(ecs),
                Component::DisjointClasses(ref dcs) => Self::dependency_from_disjoint_classes(dcs),
                Component::DisjointUnion(ref du) => Self::dependency_from_disjoint_union(du),
                Component::SubObjectPropertyOf(ref spo) => {
                    Self::dependency_from_sub_object_property(spo)
                }
                Component::EquivalentObjectProperties(ref eops) => {
                    Self::dependency_from_equiv_object_properties(eops)
                }
                Component::DisjointObjectProperties(ref dops) => {
                    Self::dependency_from_disjoint_object_properties(dops)
                }
                Component::InverseObjectProperties(ref iop) => {
                    Self::dependency_from_inverse_object_properties(iop)
                }
                Component::ObjectPropertyDomain(ref opd) => {
                    Self::dependency_from_object_property_domain(opd)
                }
                Component::ObjectPropertyRange(ref opr) => {
                    Self::dependency_from_object_property_range(opr)
                }
                Component::FunctionalObjectProperty(ref fop) => {
                    Self::dependency_from_functional_object_property(fop)
                }
                Component::InverseFunctionalObjectProperty(ref ifop) => {
                    Self::dependency_from_inverse_functional_object_property(ifop)
                }
                Component::ReflexiveObjectProperty(ref rop) => {
                    Self::dependency_from_reflexive_object_property(rop)
                }
                Component::IrreflexiveObjectProperty(ref irop) => {
                    Self::dependency_from_irreflexive_object_property(irop)
                }
                Component::SymmetricObjectProperty(ref sop) => {
                    Self::dependency_from_symmetric_object_property(sop)
                }
                Component::AsymmetricObjectProperty(ref aop) => {
                    Self::dependency_from_asymmetric_object_property(aop)
                }
                Component::TransitiveObjectProperty(ref top) => {
                    Self::dependency_from_transitive_object_property(top)
                }
                Component::SubDataPropertyOf(ref sdp) => Self::dependency_from_sub_data_property(sdp),
                Component::EquivalentDataProperties(ref edp) => {
                    Self::dependency_from_equiv_data_properties(edp)
                }
                Component::DisjointDataProperties(ref ddp) => {
                    Self::dependency_from_disjoint_data_properties(ddp)
                }
                Component::DataPropertyDomain(ref dpd) => {
                    Self::dependency_from_data_property_domain(dpd)
                }
                Component::DataPropertyRange(ref dpr) => Self::dependency_from_data_property_range(dpr),
                Component::FunctionalDataProperty(ref fdp) => {
                    Self::dependency_from_functional_data_property(fdp)
                }
                Component::ClassAssertion(ref ca) => Self::dependency_from_class_assertion(ca),
                Component::ObjectPropertyAssertion(ref opa) => {
                    Self::dependency_from_object_property_assertion(opa)
                }
                Component::NegativeObjectPropertyAssertion(ref nopa) => {
                    Self::dependency_from_negative_object_property_assertion(nopa)
                }
                Component::DataPropertyAssertion(ref dpa) => {
                    Self::dependency_from_data_property_assertion(dpa)
                }
                Component::NegativeDataPropertyAssertion(ref ndpa) => {
                    Self::dependency_from_negative_data_property_assertion(ndpa)
                }
                Component::SameIndividual(ref si) => Self::dependency_from_same_individual(si),
                Component::DifferentIndividuals(ref di) => {
                    Self::dependency_from_different_individuals(di)
                }
                Component::AnnotationAssertion(ref aa) => {
                    Self::dependency_from_annotation_assertion(aa)
                }
                Component::SubAnnotationPropertyOf(ref sapo) => {
                    Self::dependency_from_sub_annotation_property(sapo)
                }
                Component::AnnotationPropertyDomain(ref apd) => {
                    Self::dependency_from_annotation_property_domain(apd)
                }
                Component::AnnotationPropertyRange(ref apr) => {
                    Self::dependency_from_annotation_property_range(apr)
                }
                _ => HashSet::new(),
            }).into_iter().map(|(k,v)| (k,SC::from_symbol_and_axiom(v, vec![&ce.component])))
            )
            .collect()
    }

    // The following methods extract dependencies from specific ontological constructs.
    // Each method takes a reference to a particular type of ontological component
    // and returns a vector of dependency pairs.

    /// Extracts dependencies from subsumption relationships (SubClassOf axioms)
    fn dependency_from_subsumption(_sco: &'a SCO<T>) -> HashSet<(S, S)>  {
        HashSet::new()
    }

    fn dependency_from_equivalences(_ecs: &'a EquivalentClasses<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_classes(dcs: &'a DisjointClasses<T>) -> HashSet<(S, S)> {
        dcs.0
            .iter()
            .combinations(2)
            .flat_map(|v| {
                Self::dependency_from_disjoint_class_pair(v[0], v[1])
                    .into_iter()
                    .chain(Self::dependency_from_disjoint_class_pair(v[1], v[0]).into_iter())
            })
            .collect()
    }

    fn dependency_from_disjoint_class_pair(
        _c1: &'a ClassExpression<T>,
        _c2: &'a ClassExpression<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_union(_du: &DisjointUnion<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_sub_object_property(_spo: &SubObjectPropertyOf<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_equiv_object_properties(
        _eops: &EquivalentObjectProperties<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_object_properties(
        _dops: &DisjointObjectProperties<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_inverse_object_properties(
        _iop: &InverseObjectProperties<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_object_property_domain(
        _opd: &ObjectPropertyDomain<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_object_property_range(_opr: &ObjectPropertyRange<T>) -> HashSet<(S, S)> where S: ForSymbol{
        HashSet::new()
    }

    fn dependency_from_functional_object_property(
        _fop: &FunctionalObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_inverse_functional_object_property(
        _ifop: &InverseFunctionalObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_reflexive_object_property(
        _rop: &ReflexiveObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_irreflexive_object_property(
        _irop: &IrreflexiveObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_symmetric_object_property(
        _sop: &SymmetricObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_asymmetric_object_property(
        _aop: &AsymmetricObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_transitive_object_property(
        _top: &TransitiveObjectProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_sub_data_property(_sdp: &SubDataPropertyOf<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_equiv_data_properties(
        _edp: &EquivalentDataProperties<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_data_properties(
        _ddp: &DisjointDataProperties<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_data_property_domain(_dpd: &DataPropertyDomain<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_data_property_range(_dpr: &DataPropertyRange<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_functional_data_property(
        _fdp: &FunctionalDataProperty<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_class_assertion(_ca: &ClassAssertion<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_object_property_assertion(
        _opa: &ObjectPropertyAssertion<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_negative_object_property_assertion(
        _nopa: &NegativeObjectPropertyAssertion<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_data_property_assertion(
        _dpa: &DataPropertyAssertion<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_negative_data_property_assertion(
        _ndpa: &NegativeDataPropertyAssertion<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_same_individual(_si: &SameIndividual<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_different_individuals(_di: &DifferentIndividuals<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_annotation_assertion(_aa: &AnnotationAssertion<T>) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_sub_annotation_property(
        _sapo: &SubAnnotationPropertyOf<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_annotation_property_domain(
        _apd: &AnnotationPropertyDomain<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    fn dependency_from_annotation_property_range(
        _apr: &AnnotationPropertyRange<T>,
    ) -> HashSet<(S, S)> {
        HashSet::new()
    }

    /// Analyzes and extracts dependencies from a class expression
    fn dependencies_from_class_expression(ce: &'a ClassExpression<T>) -> HashSet<(S, S)> where S: ForSymbol;
}

pub fn reduce_map<S:ForSymbol, SC: ForSymbol>(map: &HashMap<S,HashSet<SC>>) -> HashMap<S, HashSet<SC>> {
    // Get all dependencies with atomic left-hand sides
    let non_atomic_left_sides = map.into_iter().filter( |(k,_)| k.is_atomic());
    // Filter out non-atomic right-hand sides
    let non_atomic_right_sides =non_atomic_left_sides.map(|(k,vs)| (k.clone(), vs.into_iter().filter(|v| v.is_atomic()).cloned().collect::<HashSet<_>>()));
    // Filter all entries with empty left-hand sides
    let non_empty_right_sides = non_atomic_right_sides.filter(|(_, vs)| !vs.is_empty());
    non_empty_right_sides.collect()
}
