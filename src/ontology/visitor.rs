use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationProperty, AnnotationSubject,
    AsymmetricObjectProperty, Class, ClassAssertion, ClassExpression, Component, DataProperty,
    DataPropertyDomain, DataPropertyRange, DataRange, Datatype, DatatypeDefinition,
    DeclareAnnotationProperty, DeclareClass, DeclareDataProperty, DeclareDatatype,
    DeclareNamedIndividual, DeclareObjectProperty, DifferentIndividuals, DisjointClasses,
    DisjointDataProperties, DisjointObjectProperties, DisjointUnion, EquivalentClasses,
    EquivalentDataProperties, EquivalentObjectProperties, ForIRI, FunctionalDataProperty,
    FunctionalObjectProperty, Import, Individual, InverseFunctionalObjectProperty,
    InverseObjectProperties, IrreflexiveObjectProperty, NamedIndividual, ObjectProperty,
    ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, OntologyAnnotation,
    OntologyID, ReflexiveObjectProperty, SameIndividual, SubClassOf, SubDataPropertyOf,
    SubObjectPropertyExpression, SubObjectPropertyOf, SymmetricObjectProperty,
    TransitiveObjectProperty, IRI,
};
use std::iter;

pub trait AxiomVisitor<'a, T: ForIRI, OT> {
    fn match_class_list(
        target: Option<&'a T>,
        list: &'a Vec<ClassExpression<T>>,
    ) -> (bool, Vec<&'a ClassExpression<T>>) {
        let mut is_contained = false;
        let mut rest = Vec::new();
        for c in list.into_iter() {
            if let ClassExpression::Class(ref iri) = c {
                if Some(&iri.underlying()) == target {
                    is_contained = true;
                } else {
                    rest.push(c)
                }
            } else {
                rest.push(c)
            }
        }
        (is_contained, rest)
    }

    fn visit_ontology_id(_oid: &'a OntologyID<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_ontology_annotation(_ann: &'a Annotation<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_import(_iri: &'a IRI<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_declare_class(_cls: &'a Class<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_declare_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_declare_annotation_property(
        _ap: &'a AnnotationProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_declare_data_property(_dp: &'a DataProperty<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_declare_named_individual(
        _ni: &'a NamedIndividual<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_declare_datatype(_dt: &'a Datatype<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_subclass_of(_sco: &'a SubClassOf<T>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_equivalent_classes(
        _cs: &'a Vec<ClassExpression<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_disjoint_classes(
        _cs: &'a Vec<ClassExpression<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_disjoint_union(
        _c: &'a Class<T>,
        _cs: &'a Vec<ClassExpression<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_sub_object_property_of(
        _sub: &'a SubObjectPropertyExpression<T>,
        _sup: &'a ObjectPropertyExpression<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_equivalent_object_properties(
        _es: &'a Vec<ObjectPropertyExpression<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_disjoint_object_properties(
        _es: &'a Vec<ObjectPropertyExpression<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_inverse_object_properties(
        _a: &'a ObjectProperty<T>,
        _b: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_object_property_domain(
        _op: &'a ObjectProperty<T>,
        _ce: &'a ClassExpression<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_object_property_range(
        _op: &'a ObjectProperty<T>,
        _ce: &'a ClassExpression<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_functional_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_inverse_functional_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_reflexive_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_irreflexive_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_symmetric_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }
    fn visit_asymmetric_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_transitive_object_property(
        _op: &'a ObjectProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_sub_data_property_of(
        _sub: &'a DataProperty<T>,
        _sup: &'a DataProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_equivalent_data_properties(
        _es: &'a Vec<DataProperty<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_disjoint_data_properties(
        _es: &'a Vec<DataProperty<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_data_property_domain(
        _dp: &'a DataProperty<T>,
        _ce: &'a ClassExpression<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_data_property_range(
        _dp: &'a DataProperty<T>,
        _dr: &'a DataRange<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_functional_data_property(
        _dp: &'a DataProperty<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_datatype_definition(
        _kind: &'a Datatype<T>,
        _range: &'a DataRange<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_same_individual(_es: &'a Vec<Individual<T>>, _target: Option<&'a T>) -> Option<OT> {
        None
    }

    fn visit_different_individuals(
        _es: &'a Vec<Individual<T>>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_class_assertion(
        _ce: &'a ClassExpression<T>,
        _i: &'a Individual<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_annotation_assertion(
        _subject: &'a AnnotationSubject<T>,
        _ann: &'a Annotation<T>,
        _target: Option<&'a T>,
    ) -> Option<OT> {
        None
    }

    fn visit_components<S>(components: S, target: Option<&'a T>) -> impl Iterator<Item = OT>
    where
        T: 'a,
        S: Iterator<Item = &'a AnnotatedComponent<T>>,
    {
        components.filter_map(move |component| match &component.component {
            Component::OntologyID(oid) => Self::visit_ontology_id(oid, target),

            Component::DocIRI(_) => None,

            Component::OntologyAnnotation(OntologyAnnotation(ref ann)) => {
                Self::visit_ontology_annotation(ann, target)
            }

            Component::Import(Import(iri)) => Self::visit_import(iri, target),

            Component::DeclareClass(DeclareClass(c)) => Self::visit_declare_class(c, target),

            Component::DeclareObjectProperty(DeclareObjectProperty(op)) => {
                Self::visit_declare_object_property(op, target)
            }

            Component::DeclareAnnotationProperty(DeclareAnnotationProperty(ap)) => {
                Self::visit_declare_annotation_property(ap, target)
            }

            Component::DeclareDataProperty(DeclareDataProperty(dp)) => {
                Self::visit_declare_data_property(dp, target)
            }

            Component::DeclareNamedIndividual(DeclareNamedIndividual(ni)) => {
                Self::visit_declare_named_individual(ni, target)
            }

            Component::DeclareDatatype(DeclareDatatype(dt)) => {
                Self::visit_declare_datatype(dt, target)
            }

            Component::SubClassOf(sco) => Self::visit_subclass_of(sco, target),

            Component::EquivalentClasses(EquivalentClasses(cs)) => {
                Self::visit_equivalent_classes(cs, target)
            }

            Component::DisjointClasses(DisjointClasses(cs)) => {
                Self::visit_disjoint_classes(cs, target)
            }

            Component::DisjointUnion(DisjointUnion(c, cs)) => {
                Self::visit_disjoint_union(c, cs, target)
            }

            Component::SubObjectPropertyOf(SubObjectPropertyOf { sub, sup }) => {
                Self::visit_sub_object_property_of(sub, sup, target)
            }

            Component::EquivalentObjectProperties(EquivalentObjectProperties(es)) => {
                Self::visit_equivalent_object_properties(es, target)
            }

            Component::DisjointObjectProperties(DisjointObjectProperties(es)) => {
                Self::visit_disjoint_object_properties(es, target)
            }

            Component::InverseObjectProperties(InverseObjectProperties(a, b)) => {
                Self::visit_inverse_object_properties(a, b, target)
            }

            Component::ObjectPropertyDomain(ObjectPropertyDomain {
                ope: ObjectPropertyExpression::ObjectProperty(op),
                ce,
            }) => Self::visit_object_property_domain(op, ce, target),

            Component::ObjectPropertyRange(ObjectPropertyRange {
                ope: ObjectPropertyExpression::ObjectProperty(op),
                ce,
            }) => Self::visit_object_property_range(op, ce, target),

            Component::FunctionalObjectProperty(FunctionalObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_functional_object_property(op, target),

            Component::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_inverse_functional_object_property(op, target),

            Component::ReflexiveObjectProperty(ReflexiveObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_reflexive_object_property(op, target),

            Component::IrreflexiveObjectProperty(IrreflexiveObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_irreflexive_object_property(op, target),

            Component::SymmetricObjectProperty(SymmetricObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_symmetric_object_property(op, target),

            Component::AsymmetricObjectProperty(AsymmetricObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_asymmetric_object_property(op, target),

            Component::TransitiveObjectProperty(TransitiveObjectProperty(
                ObjectPropertyExpression::ObjectProperty(op),
            )) => Self::visit_transitive_object_property(op, target),

            Component::SubDataPropertyOf(SubDataPropertyOf { sub, sup }) => {
                Self::visit_sub_data_property_of(sub, sup, target)
            }

            Component::EquivalentDataProperties(EquivalentDataProperties(es)) => {
                Self::visit_equivalent_data_properties(es, target)
            }

            Component::DisjointDataProperties(DisjointDataProperties(es)) => {
                Self::visit_disjoint_data_properties(es, target)
            }

            Component::DataPropertyDomain(DataPropertyDomain { dp, ce }) => {
                Self::visit_data_property_domain(dp, ce, target)
            }

            Component::DataPropertyRange(DataPropertyRange { dp, dr }) => {
                Self::visit_data_property_range(dp, dr, target)
            }

            Component::FunctionalDataProperty(FunctionalDataProperty(dp)) => {
                Self::visit_functional_data_property(dp, target)
            }

            Component::DatatypeDefinition(DatatypeDefinition { kind, range }) => {
                Self::visit_datatype_definition(kind, range, target)
            }

            Component::HasKey(_) => None,

            Component::SameIndividual(SameIndividual(es)) => {
                Self::visit_same_individual(es, target)
            }

            Component::DifferentIndividuals(DifferentIndividuals(es)) => {
                Self::visit_different_individuals(es, target)
            }

            Component::ClassAssertion(ClassAssertion { ce, i }) => {
                Self::visit_class_assertion(ce, i, target)
            }

            Component::ObjectPropertyAssertion(_) => None,
            Component::NegativeObjectPropertyAssertion(_) => None,
            Component::DataPropertyAssertion(_) => None,
            Component::NegativeDataPropertyAssertion(_) => None,

            Component::AnnotationAssertion(aa) => {
                Self::visit_annotation_assertion(&aa.subject, &aa.ann, target)
            }

            Component::SubAnnotationPropertyOf(_) => None,
            Component::AnnotationPropertyDomain(_) => None,
            Component::AnnotationPropertyRange(_) => None,
            Component::Rule(_) => None,
            _ => None,
        })
    }
}
