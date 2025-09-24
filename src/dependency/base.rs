use std::collections::{HashMap, HashSet};
use horned_owl::model::SubClassOf as SCO;
use horned_owl::model::*;
use itertools::Itertools;


#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub enum OntologySymbol<'a, T: ForIRI> {
    CE(&'a ClassExpression<T>),
    Role(&'a ObjectPropertyExpression<T>),
}

pub type DependencyPair<'a, T: ForIRI> = (OntologySymbol<'a, T>, OntologySymbol<'a, T>);
pub type DependencyMap<'a, T: ForIRI> = HashMap<OntologySymbol<'a, T>, HashSet<OntologySymbol<'a, T>>>;

pub trait DependencyBuilder<T: ForIRI> {
    fn dep<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<'a, T>;
}

pub trait SyntaxBasedDependency<T: ForIRI>: DependencyBuilder<T> {
    fn dependencies_from_components<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> Vec<DependencyPair<'a, T>> {
        ontology_iter
            .map(|ce| match &ce.component {
                Component::SubClassOf(sco) => Self::dependency_from_subsumption(sco),
                Component::EquivalentClasses(ecs) => Self::dependency_from_equivalences(ecs),
                Component::DisjointClasses(dcs) => Self::dependency_from_disjoint_classes(dcs),
                Component::DisjointUnion(du) => Self::dependency_from_disjoint_union(du),
                Component::SubObjectPropertyOf(spo) => {
                    Self::dependency_from_sub_object_property(spo)
                }
                Component::EquivalentObjectProperties(eops) => {
                    Self::dependency_from_equiv_object_properties(eops)
                }
                Component::DisjointObjectProperties(dops) => {
                    Self::dependency_from_disjoint_object_properties(dops)
                }
                Component::InverseObjectProperties(iop) => {
                    Self::dependency_from_inverse_object_properties(iop)
                }
                Component::ObjectPropertyDomain(opd) => {
                    Self::dependency_from_object_property_domain(opd)
                }
                Component::ObjectPropertyRange(opr) => {
                    Self::dependency_from_object_property_range(opr)
                }
                Component::FunctionalObjectProperty(fop) => {
                    Self::dependency_from_functional_object_property(fop)
                }
                Component::InverseFunctionalObjectProperty(ifop) => {
                    Self::dependency_from_inverse_functional_object_property(ifop)
                }
                Component::ReflexiveObjectProperty(rop) => {
                    Self::dependency_from_reflexive_object_property(rop)
                }
                Component::IrreflexiveObjectProperty(irop) => {
                    Self::dependency_from_irreflexive_object_property(irop)
                }
                Component::SymmetricObjectProperty(sop) => {
                    Self::dependency_from_symmetric_object_property(sop)
                }
                Component::AsymmetricObjectProperty(aop) => {
                    Self::dependency_from_asymmetric_object_property(aop)
                }
                Component::TransitiveObjectProperty(top) => {
                    Self::dependency_from_transitive_object_property(top)
                }
                Component::SubDataPropertyOf(sdp) => Self::dependency_from_sub_data_property(sdp),
                Component::EquivalentDataProperties(edp) => {
                    Self::dependency_from_equiv_data_properties(edp)
                }
                Component::DisjointDataProperties(ddp) => {
                    Self::dependency_from_disjoint_data_properties(ddp)
                }
                Component::DataPropertyDomain(dpd) => {
                    Self::dependency_from_data_property_domain(dpd)
                }
                Component::DataPropertyRange(dpr) => Self::dependency_from_data_property_range(dpr),
                Component::FunctionalDataProperty(fdp) => {
                    Self::dependency_from_functional_data_property(fdp)
                }
                Component::ClassAssertion(ca) => Self::dependency_from_class_assertion(ca),
                Component::ObjectPropertyAssertion(opa) => {
                    Self::dependency_from_object_property_assertion(opa)
                }
                Component::NegativeObjectPropertyAssertion(nopa) => {
                    Self::dependency_from_negative_object_property_assertion(nopa)
                }
                Component::DataPropertyAssertion(dpa) => {
                    Self::dependency_from_data_property_assertion(dpa)
                }
                Component::NegativeDataPropertyAssertion(ndpa) => {
                    Self::dependency_from_negative_data_property_assertion(ndpa)
                }
                Component::SameIndividual(si) => Self::dependency_from_same_individual(si),
                Component::DifferentIndividuals(di) => {
                    Self::dependency_from_different_individuals(di)
                }
                Component::AnnotationAssertion(aa) => {
                    Self::dependency_from_annotation_assertion(aa)
                }
                Component::SubAnnotationPropertyOf(sapo) => {
                    Self::dependency_from_sub_annotation_property(sapo)
                }
                Component::AnnotationPropertyDomain(apd) => {
                    Self::dependency_from_annotation_property_domain(apd)
                }
                Component::AnnotationPropertyRange(apr) => {
                    Self::dependency_from_annotation_property_range(apr)
                }
                _ => Vec::new(),
            })
            .flatten()
            .collect()
    }

    fn dependency_from_subsumption(_sco: &SCO<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_equivalences(_ecs: &EquivalentClasses<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_disjoint_classes(dcs: &DisjointClasses<T>) -> Vec<DependencyPair<T>> {
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

    fn dependency_from_disjoint_class_pair<'a>(
        _c1: &'a ClassExpression<T>,
        _c2: &'a ClassExpression<T>,
    ) -> Vec<DependencyPair<'a, T>> {
        Vec::new()
    }

    fn dependency_from_disjoint_union(_du: &DisjointUnion<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_sub_object_property(_spo: &SubObjectPropertyOf<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_equiv_object_properties(
        _eops: &EquivalentObjectProperties<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_disjoint_object_properties(
        _dops: &DisjointObjectProperties<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_inverse_object_properties(
        _iop: &InverseObjectProperties<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_object_property_domain(
        _opd: &ObjectPropertyDomain<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_object_property_range(_opr: &ObjectPropertyRange<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_functional_object_property(
        _fop: &FunctionalObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_inverse_functional_object_property(
        _ifop: &InverseFunctionalObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_reflexive_object_property(
        _rop: &ReflexiveObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_irreflexive_object_property(
        _irop: &IrreflexiveObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_symmetric_object_property(
        _sop: &SymmetricObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_asymmetric_object_property(
        _aop: &AsymmetricObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_transitive_object_property(
        _top: &TransitiveObjectProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_sub_data_property(_sdp: &SubDataPropertyOf<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_equiv_data_properties(
        _edp: &EquivalentDataProperties<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_disjoint_data_properties(
        _ddp: &DisjointDataProperties<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_data_property_domain(_dpd: &DataPropertyDomain<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_data_property_range(_dpr: &DataPropertyRange<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_functional_data_property(
        _fdp: &FunctionalDataProperty<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_class_assertion(_ca: &ClassAssertion<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_object_property_assertion(
        _opa: &ObjectPropertyAssertion<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_negative_object_property_assertion(
        _nopa: &NegativeObjectPropertyAssertion<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_data_property_assertion(
        _dpa: &DataPropertyAssertion<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_negative_data_property_assertion(
        _ndpa: &NegativeDataPropertyAssertion<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_same_individual(_si: &SameIndividual<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_different_individuals(_di: &DifferentIndividuals<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_annotation_assertion(_aa: &AnnotationAssertion<T>) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_sub_annotation_property(
        _sapo: &SubAnnotationPropertyOf<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_annotation_property_domain(
        _apd: &AnnotationPropertyDomain<T>,
    ) -> Vec<DependencyPair<T>> {
        Vec::new()
    }

    fn dependency_from_annotation_property_range(
        _apr: &'_ AnnotationPropertyRange<T>,
    ) -> Vec<DependencyPair<'_, T>> {
        Vec::new()
    }

    fn dependencies_from_class_expression(ce: &ClassExpression<T>) -> Vec<DependencyPair<T>>;
}

