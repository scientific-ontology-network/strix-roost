use crate::dependency::base::{ComplexDependencyMap, DependencyBuilder, DependencyMap};
use crate::dependency::symbol::{Symbol, Term};
use crate::util::graph::transitive_closure;
use horned_owl::model::{
    AnnotatedComponent, AnnotationAssertion, AnnotationPropertyDomain, AnnotationPropertyRange,
    AsymmetricObjectProperty, ClassAssertion, ClassExpression, Component, DataProperty,
    DataPropertyAssertion, DataPropertyDomain, DataPropertyRange, DataRange, DifferentIndividuals,
    DisjointClasses, DisjointDataProperties, DisjointObjectProperties, DisjointUnion,
    EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties, ForIRI,
    FunctionalDataProperty, FunctionalObjectProperty, Individual, InverseFunctionalObjectProperty,
    InverseObjectProperties, IrreflexiveObjectProperty, Literal, NegativeDataPropertyAssertion,
    NegativeObjectPropertyAssertion, ObjectPropertyAssertion, ObjectPropertyDomain,
    ObjectPropertyExpression, ObjectPropertyRange, ReflexiveObjectProperty, SameIndividual,
    SubAnnotationPropertyOf, SubClassOf, SubDataPropertyOf, SubObjectPropertyExpression,
    SubObjectPropertyOf, SymmetricObjectProperty, TransitiveObjectProperty,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn quasiproduct<T: Eq + Clone>(it: Vec<T>) -> Vec<(T, T)> {
    it.iter()
        .zip(it.iter())
        .map(|(a, b)| (a.clone(), b.clone()))
        .filter(|(a, b)| a != b)
        .collect()
}
/// Trait for analyzing syntax-based dependencies in ontological components
pub trait SyntaxBasedDependency<T: ForIRI>: DependencyBuilder<T> {
    /// Extracts dependency pairs from ontology components based on their syntactic structure
    ///
    /// # Arguments
    /// * `ontology_iter` - An iterator over annotated ontology components
    ///
    /// # Returns
    /// A vector of dependency pairs representing relationships between ontological elements
    ///

    fn derive_from_axioms<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>> {
        let mut map = HashMap::new();
        for (a, b, c) in Self::dependencies_from_components(ontology_iter) {
            if !map.contains_key(&a) {
                map.insert(a.clone(), HashMap::new());
            }
            map.get_mut(&a).unwrap().insert(b, c);
        }
        transitive_closure(&map)
    }

    fn dependencies_from_components<'a>(
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> Vec<(Term<'a, T>, Term<'a, T>, HashSet<&'a Component<T>>)> where {
        ontology_iter
            .flat_map(|ce| {
                (match &ce.component {
                    Component::SubClassOf(ref sco) => Self::dependency_from_subsumption(sco),
                    Component::EquivalentClasses(ref ecs) => {
                        Self::dependency_from_equivalences(ecs)
                    }
                    Component::DisjointClasses(ref dcs) => {
                        Self::dependency_from_disjoint_classes(dcs)
                    }
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
                    Component::SubDataPropertyOf(ref sdp) => {
                        Self::dependency_from_sub_data_property(sdp)
                    }
                    Component::EquivalentDataProperties(ref edp) => {
                        Self::dependency_from_equiv_data_properties(edp)
                    }
                    Component::DisjointDataProperties(ref ddp) => {
                        Self::dependency_from_disjoint_data_properties(ddp)
                    }
                    Component::DataPropertyDomain(ref dpd) => {
                        Self::dependency_from_data_property_domain(dpd)
                    }
                    Component::DataPropertyRange(ref dpr) => {
                        Self::dependency_from_data_property_range(dpr)
                    }
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
                })
                .into_iter()
                .map(|(k, v)| (k, v, [&ce.component].iter().cloned().collect()))
            })
            .collect()
    }

    // The following methods extract dependencies from specific ontological constructs.
    // Each method takes a reference to a particular type of ontological component
    // and returns a vector of dependency pairs.

    /// Extracts dependencies from subsumption relationships (SubClassOf axioms)
    fn dependency_from_subsumption(sco: &SubClassOf<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        [(Term::CE(&sco.sub), Term::CE(&sco.sup))]
            .into_iter()
            .chain(Self::dependencies_from_class_expression(&sco.sub))
            .chain(Self::dependencies_from_class_expression(&sco.sup))
            .collect()
    }

    fn dependency_from_equivalences(
        _ecs: &EquivalentClasses<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        let paired = quasiproduct(_ecs.0.iter().map(|c| Term::CE(c)).collect());
        let derived_dependencies = _ecs
            .0
            .iter()
            .flat_map(|ce| Self::dependencies_from_class_expression(ce));
        paired.into_iter().chain(derived_dependencies).collect()
    }

    fn dependency_from_disjoint_classes(
        dcs: &DisjointClasses<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
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
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_union(
        _du: &DisjointUnion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_sub_object_property(
        spo: &SubObjectPropertyOf<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        let l = match spo.sub {
            SubObjectPropertyExpression::ObjectPropertyChain(ref c) => {
                Term::RoleComposition(c.iter().collect())
            }
            SubObjectPropertyExpression::ObjectPropertyExpression(ref ope) => Term::Role(ope),
        };
        [(l, Term::Role(&spo.sup))]
            .into_iter()
            .chain(Self::dependencies_from_sub_object_propterty_expression(
                &spo.sub,
            ))
            .chain(Self::dependencies_from_object_property_expression(&spo.sup))
            .collect()
    }

    fn dependencies_from_sub_object_propterty_expression<'a>(
        sops: &'a SubObjectPropertyExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        match sops {
            SubObjectPropertyExpression::ObjectPropertyChain(c) => {
                Self::dependencies_from_object_property_chain(c.iter().collect())
            }
            SubObjectPropertyExpression::ObjectPropertyExpression(ope) => {
                Self::dependencies_from_object_property_expression(ope)
            }
        }
    }

    fn dependencies_from_object_property_chain<'a>(
        _opes: Vec<&'a ObjectPropertyExpression<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependency_from_equiv_object_properties(
        eops: &EquivalentObjectProperties<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        let paired = quasiproduct(eops.0.iter().map(|c| Term::Role(c)).collect());
        let derived_dependencies = eops
            .0
            .iter()
            .flat_map(|ce| Self::dependencies_from_object_property_expression(ce));
        paired.into_iter().chain(derived_dependencies).collect()
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

    fn dependency_from_object_property_domain(
        _opd: &ObjectPropertyDomain<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_object_property_range(
        _opr: &ObjectPropertyRange<T>,
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

    fn dependency_from_sub_data_property(
        _sdp: &SubDataPropertyOf<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_equiv_data_properties(
        _edp: &EquivalentDataProperties<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_disjoint_data_properties(
        _ddp: &DisjointDataProperties<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_data_property_domain(
        _dpd: &DataPropertyDomain<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_data_property_range(
        _dpr: &DataPropertyRange<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_functional_data_property(
        _fdp: &FunctionalDataProperty<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_class_assertion(
        _ca: &ClassAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_object_property_assertion(
        _opa: &ObjectPropertyAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_negative_object_property_assertion(
        _nopa: &NegativeObjectPropertyAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_data_property_assertion(
        _dpa: &DataPropertyAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_negative_data_property_assertion(
        _ndpa: &NegativeDataPropertyAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_same_individual(
        _si: &SameIndividual<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_different_individuals(
        _di: &DifferentIndividuals<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_annotation_assertion(
        _aa: &AnnotationAssertion<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_sub_annotation_property(
        _sapo: &SubAnnotationPropertyOf<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_annotation_property_domain(
        _apd: &AnnotationPropertyDomain<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    fn dependency_from_annotation_property_range(
        _apr: &AnnotationPropertyRange<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        HashSet::new()
    }

    /// Analyzes and extracts dependencies from a class expression
    fn dependencies_from_class_expression(
        ce0: &ClassExpression<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        match ce0 {
            ClassExpression::Class(_c) => HashSet::new(),
            ClassExpression::ObjectIntersectionOf(ces) => {
                Self::dependencies_from_object_intersection_of(ce0, ces)
            }
            ClassExpression::ObjectUnionOf(ces) => {
                Self::dependencies_from_object_union_of(ce0, ces)
            }
            ClassExpression::ObjectComplementOf(ce) => {
                Self::dependencies_from_object_complement_of(ce0, ce)
            }
            ClassExpression::ObjectOneOf(is) => Self::dependencies_from_object_one_of(ce0, is),
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
                Self::dependencies_from_object_some_values_from(ce0, ope, bce)
            }
            ClassExpression::ObjectAllValuesFrom { ope, bce } => {
                Self::dependencies_from_object_all_values_from(ce0, ope, bce)
            }
            ClassExpression::ObjectHasValue { ope, i } => {
                Self::dependencies_from_object_has_value(ce0, ope, i)
            }
            ClassExpression::ObjectHasSelf(ope) => {
                Self::dependencies_from_object_has_self(ce0, ope)
            }
            ClassExpression::ObjectMinCardinality { ope, bce, n } => {
                Self::dependencies_from_object_min_cardinality(ce0, ope, bce, n)
            }
            ClassExpression::ObjectMaxCardinality { ope, bce, n } => {
                Self::dependencies_from_object_max_cardinality(ce0, ope, bce, n)
            }
            ClassExpression::ObjectExactCardinality { ope, bce, n } => {
                Self::dependencies_from_object_exact_cardinality(ce0, ope, bce, n)
            }
            ClassExpression::DataSomeValuesFrom { dp, dr } => {
                Self::dependencies_from_data_some_values_from(ce0, dp, dr)
            }
            ClassExpression::DataAllValuesFrom { dp, dr } => {
                Self::dependencies_from_data_all_values_from(ce0, dp, dr)
            }
            ClassExpression::DataHasValue { dp, l } => {
                Self::dependencies_from_data_has_value(ce0, dp, l)
            }
            ClassExpression::DataMinCardinality { dp, dr, n } => {
                Self::dependencies_from_data_min_cardinality(ce0, dp, dr, n)
            }
            ClassExpression::DataMaxCardinality { dp, dr, n } => {
                Self::dependencies_from_data_max_cardinality(ce0, dp, dr, n)
            }
            ClassExpression::DataExactCardinality { dp, dr, n } => {
                Self::dependencies_from_data_exact_cardinality(ce0, dp, dr, n)
            }
        }
    }

    // [x] = C1 & ... & Cn
    // Extracts dependencies from object intersection of class expressions
    fn dependencies_from_object_intersection_of<'a>(
        _x: &'a ClassExpression<T>,
        _ces: &'a Vec<ClassExpression<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = C1 v ... v Cn
    // Extracts dependencies from object union of class expressions
    fn dependencies_from_object_union_of<'a>(
        _x: &'a ClassExpression<T>,
        _ces: &'a Vec<ClassExpression<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = not [ce]
    // Extracts dependencies from object complement of a class expression
    fn dependencies_from_object_complement_of<'a>(
        _x: &'a ClassExpression<T>,
        _ce: &'a ClassExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = {i1, ..., in}
    // Extracts dependencies from object one-of enumeration of individuals
    fn dependencies_from_object_one_of<'a>(
        _x: &'a ClassExpression<T>,
        _is: &'a Vec<Individual<T>>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = some [ope].[ce]
    // Extracts dependencies from existential restriction (object some values from)
    fn dependencies_from_object_some_values_from<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _bce: &'a ClassExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = all [ope].[ce]
    // Extracts dependencies from universal restriction (object all values from)
    fn dependencies_from_object_all_values_from<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _bce: &'a ClassExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = [ope] value [i]
    // Extracts dependencies from object has value restriction
    fn dependencies_from_object_has_value<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _i: &'a Individual<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = [ope] Self
    // Extracts dependencies from object has self restriction
    fn dependencies_from_object_has_self<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &ObjectPropertyExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = >= n [ope].[ce]
    // Extracts dependencies from minimum cardinality restriction
    fn dependencies_from_object_min_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _bce: &'a ClassExpression<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = <= n [ope].[ce]
    // Extracts dependencies from maximum cardinality restriction
    fn dependencies_from_object_max_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _bce: &'a ClassExpression<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = = n [ope].[ce]
    // Extracts dependencies from exact cardinality restriction
    fn dependencies_from_object_exact_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _ope: &'a ObjectPropertyExpression<T>,
        _bce: &'a ClassExpression<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = some [dp].[dr]
    // Extracts dependencies from data some values from restriction
    fn dependencies_from_data_some_values_from<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = all [dp].[dr]
    // Extracts dependencies from data all values from restriction
    fn dependencies_from_data_all_values_from<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = [dp] value [l]
    // Extracts dependencies from data has value restriction
    fn dependencies_from_data_has_value<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _l: &'a Literal<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = >= n [dp].[dr]
    // Extracts dependencies from data minimum cardinality restriction
    fn dependencies_from_data_min_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = <= n [dp].[dr]
    // Extracts dependencies from data maximum cardinality restriction
    fn dependencies_from_data_max_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    // [x] = = n [dp].[dr]
    // Extracts dependencies from data exact cardinality restriction
    fn dependencies_from_data_exact_cardinality<'a>(
        _x: &'a ClassExpression<T>,
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
        _n: &'a u32,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        HashSet::new()
    }

    fn dependencies_from_object_property_expression<'a>(
        ope: &'a ObjectPropertyExpression<T>,
    ) -> HashSet<(Term<'a, T>, Term<'a, T>)>;
}

pub fn reduce_map<T: ForIRI, C: Clone>(map: &ComplexDependencyMap<T, C>) -> DependencyMap<T, C> {
    // Get all dependencies with atomic left-hand sides
    let non_atomic_left_sides = map
        .into_iter()
        .filter(|(k, _)| k.is_atomic())
        .map(|(k, v)| (k.get_symbol().unwrap(), v));
    // Filter out non-atomic right-hand sides
    let non_atomic_right_sides: Vec<(Symbol<T>, HashMap<Symbol<T>, C>)> = non_atomic_left_sides
        .map(|(k, vmap)| {
            (
                k.clone(),
                vmap.into_iter()
                    .filter(|(s, _)| s.is_atomic())
                    .map(|(s, c)| (s.get_symbol().unwrap(), (*c).clone()))
                    .collect::<HashMap<Symbol<T>, C>>(),
            )
        })
        .collect();
    // Filter all entries with empty left-hand sides
    let non_empty_right_sides: DependencyMap<T, C> = non_atomic_right_sides
        .into_iter()
        .filter(|(_, vs)| !vs.is_empty())
        .collect();
    non_empty_right_sides
}
