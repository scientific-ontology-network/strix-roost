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

pub trait AxiomVisitor<T: ForIRI> {
    fn match_class_list<'a>(
        target: &T,
        list: &'a Vec<ClassExpression<T>>,
    ) -> (bool, Vec<&'a ClassExpression<T>>) {
        let mut is_contained = false;
        let mut rest = Vec::new();
        for c in list.into_iter() {
            if let ClassExpression::Class(ref iri) = c {
                if iri.underlying() == *target {
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

    fn visit_ontology_id(&mut self, _oid: &OntologyID<T>, _target: &T) {}

    fn visit_ontology_annotation(&mut self, _ann: &Annotation<T>, _target: &T) {}

    fn visit_import(&mut self, _iri: &IRI<T>, _target: &T) {}

    fn visit_declare_class(&mut self, _cls: &Class<T>, _target: &T) {}

    fn visit_declare_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_declare_annotation_property(&mut self, _ap: &AnnotationProperty<T>, _target: &T) {}

    fn visit_declare_data_property(&mut self, _dp: &DataProperty<T>, _target: &T) {}

    fn visit_declare_named_individual(&mut self, _ni: &NamedIndividual<T>, _target: &T) {}

    fn visit_declare_datatype(&mut self, _dt: &Datatype<T>, _target: &T) {}

    fn visit_subclass_of(&mut self, _sco: &SubClassOf<T>, _target: &T) {}

    fn visit_equivalent_classes(&mut self, _cs: &Vec<ClassExpression<T>>, _target: &T) {}

    fn visit_disjoint_classes(&mut self, _cs: &Vec<ClassExpression<T>>, _target: &T) {}

    fn visit_disjoint_union(&mut self, _c: &Class<T>, _cs: &Vec<ClassExpression<T>>, _target: &T) {}

    fn visit_sub_object_property_of(
        &mut self,
        _sub: &SubObjectPropertyExpression<T>,
        _sup: &ObjectPropertyExpression<T>,
        _target: &T,
    ) {
    }

    fn visit_equivalent_object_properties(
        &mut self,
        _es: &Vec<ObjectPropertyExpression<T>>,
        _target: &T,
    ) {
    }

    fn visit_disjoint_object_properties(
        &mut self,
        _es: &Vec<ObjectPropertyExpression<T>>,
        _target: &T,
    ) {
    }

    fn visit_inverse_object_properties(
        &mut self,
        _a: &ObjectProperty<T>,
        _b: &ObjectProperty<T>,
        _target: &T,
    ) {
    }

    fn visit_object_property_domain(
        &mut self,
        _op: &ObjectProperty<T>,
        _ce: &ClassExpression<T>,
        _target: &T,
    ) {
    }

    fn visit_object_property_range(
        &mut self,
        _op: &ObjectProperty<T>,
        _ce: &ClassExpression<T>,
        _target: &T,
    ) {
    }

    fn visit_functional_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_inverse_functional_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_reflexive_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_irreflexive_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_symmetric_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}
    fn visit_asymmetric_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_transitive_object_property(&mut self, _op: &ObjectProperty<T>, _target: &T) {}

    fn visit_sub_data_property_of(
        &mut self,
        _sub: &DataProperty<T>,
        _sup: &DataProperty<T>,
        _target: &T,
    ) {
    }

    fn visit_equivalent_data_properties(&mut self, _es: &Vec<DataProperty<T>>, _target: &T) {}

    fn visit_disjoint_data_properties(&mut self, _es: &Vec<DataProperty<T>>, _target: &T) {}

    fn visit_data_property_domain(
        &mut self,
        _dp: &DataProperty<T>,
        _ce: &ClassExpression<T>,
        _target: &T,
    ) {
    }

    fn visit_data_property_range(
        &mut self,
        _dp: &DataProperty<T>,
        _dr: &DataRange<T>,
        _target: &T,
    ) {
    }

    fn visit_functional_data_property(&mut self, _dp: &DataProperty<T>, _target: &T) {}

    fn visit_datatype_definition(
        &mut self,
        _kind: &Datatype<T>,
        _range: &DataRange<T>,
        _target: &T,
    ) {
    }

    fn visit_same_individual(&mut self, _es: &Vec<Individual<T>>, _target: &T) {}

    fn visit_different_individuals(&mut self, _es: &Vec<Individual<T>>, _target: &T) {}

    fn visit_class_assertion(&mut self, _ce: &ClassExpression<T>, _i: &Individual<T>, _target: &T) {
    }

    fn visit_annotation_assertion<'a>(
        &mut self,
        _subject: &'a AnnotationSubject<T>,
        _ann: &'a Annotation<T>,
        _target: &'a T,
    ) {
    }

    fn visit_components<'a, S>(&mut self, components: S, target: &T)
    where
        T: 'a,
        S: Iterator<Item = &'a AnnotatedComponent<T>>,
    {
        for component in components {
            match &component.component {
                Component::OntologyID(oid) => {
                    self.visit_ontology_id(oid, target);
                }

                Component::DocIRI(_) => {}

                Component::OntologyAnnotation(OntologyAnnotation(ref ann)) => {
                    self.visit_ontology_annotation(ann, target)
                }

                Component::Import(Import(iri)) => self.visit_import(iri, target),

                Component::DeclareClass(DeclareClass(c)) => self.visit_declare_class(c, target),

                Component::DeclareObjectProperty(DeclareObjectProperty(op)) => {
                    self.visit_declare_object_property(op, target)
                }

                Component::DeclareAnnotationProperty(DeclareAnnotationProperty(ap)) => {
                    self.visit_declare_annotation_property(ap, target)
                }

                Component::DeclareDataProperty(DeclareDataProperty(dp)) => {
                    self.visit_declare_data_property(dp, target)
                }

                Component::DeclareNamedIndividual(DeclareNamedIndividual(ni)) => {
                    self.visit_declare_named_individual(ni, target)
                }

                Component::DeclareDatatype(DeclareDatatype(dt)) => {
                    self.visit_declare_datatype(dt, target)
                }

                Component::SubClassOf(sco) => self.visit_subclass_of(sco, target),

                Component::EquivalentClasses(EquivalentClasses(cs)) => {
                    self.visit_equivalent_classes(cs, target)
                }

                Component::DisjointClasses(DisjointClasses(cs)) => {
                    self.visit_disjoint_classes(cs, target)
                }

                Component::DisjointUnion(DisjointUnion(c, cs)) => {
                    self.visit_disjoint_union(c, cs, target)
                }

                Component::SubObjectPropertyOf(SubObjectPropertyOf { sub, sup }) => {
                    self.visit_sub_object_property_of(sub, sup, target)
                }

                Component::EquivalentObjectProperties(EquivalentObjectProperties(es)) => {
                    self.visit_equivalent_object_properties(es, target)
                }

                Component::DisjointObjectProperties(DisjointObjectProperties(es)) => {
                    self.visit_disjoint_object_properties(es, target)
                }

                Component::InverseObjectProperties(InverseObjectProperties(a, b)) => {
                    self.visit_inverse_object_properties(a, b, target)
                }

                Component::ObjectPropertyDomain(ObjectPropertyDomain {
                    ope: ObjectPropertyExpression::ObjectProperty(op),
                    ce,
                }) => self.visit_object_property_domain(op, ce, target),

                Component::ObjectPropertyRange(ObjectPropertyRange {
                    ope: ObjectPropertyExpression::ObjectProperty(op),
                    ce,
                }) => self.visit_object_property_range(op, ce, target),

                Component::FunctionalObjectProperty(FunctionalObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_functional_object_property(op, target),

                Component::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_inverse_functional_object_property(op, target),

                Component::ReflexiveObjectProperty(ReflexiveObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_reflexive_object_property(op, target),

                Component::IrreflexiveObjectProperty(IrreflexiveObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_irreflexive_object_property(op, target),

                Component::SymmetricObjectProperty(SymmetricObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_symmetric_object_property(op, target),

                Component::AsymmetricObjectProperty(AsymmetricObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_asymmetric_object_property(op, target),

                Component::TransitiveObjectProperty(TransitiveObjectProperty(
                    ObjectPropertyExpression::ObjectProperty(op),
                )) => self.visit_transitive_object_property(op, target),

                Component::SubDataPropertyOf(SubDataPropertyOf { sub, sup }) => {
                    self.visit_sub_data_property_of(sub, sup, target)
                }

                Component::EquivalentDataProperties(EquivalentDataProperties(es)) => {
                    self.visit_equivalent_data_properties(es, target)
                }

                Component::DisjointDataProperties(DisjointDataProperties(es)) => {
                    self.visit_disjoint_data_properties(es, target)
                }

                Component::DataPropertyDomain(DataPropertyDomain { dp, ce }) => {
                    self.visit_data_property_domain(dp, ce, target)
                }

                Component::DataPropertyRange(DataPropertyRange { dp, dr }) => {
                    self.visit_data_property_range(dp, dr, target)
                }

                Component::FunctionalDataProperty(FunctionalDataProperty(dp)) => {
                    self.visit_functional_data_property(dp, target)
                }

                Component::DatatypeDefinition(DatatypeDefinition { kind, range }) => {
                    self.visit_datatype_definition(kind, range, target)
                }

                Component::HasKey(_) => {}

                Component::SameIndividual(SameIndividual(es)) => {
                    self.visit_same_individual(es, target)
                }

                Component::DifferentIndividuals(DifferentIndividuals(es)) => {
                    self.visit_different_individuals(es, target)
                }

                Component::ClassAssertion(ClassAssertion { ce, i }) => {
                    self.visit_class_assertion(ce, i, target)
                }

                Component::ObjectPropertyAssertion(_) => {}
                Component::NegativeObjectPropertyAssertion(_) => {}
                Component::DataPropertyAssertion(_) => {}
                Component::NegativeDataPropertyAssertion(_) => {}

                Component::AnnotationAssertion(aa) => {
                    self.visit_annotation_assertion(&aa.subject, &aa.ann, target);
                }

                Component::SubAnnotationPropertyOf(_) => {}
                Component::AnnotationPropertyDomain(_) => {}
                Component::AnnotationPropertyRange(_) => {}
                Component::Rule(_) => {}
                _ => {}
            }
        }
    }
}
