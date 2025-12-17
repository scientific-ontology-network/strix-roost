use std::collections::HashMap;
use horned_owl::model::{Annotation, AnnotationSubject, AnnotationValue, ForIRI};
use crate::ontology::visitor::AxiomVisitor;
use crate::iris::iao::DEFINITION;
use crate::iris::rdfs::RDFS_LABEL;
use core::default::Default;

#[derive(Default)]
pub(crate) struct Annotations<T: ForIRI>{
    pub(crate) labels: HashMap<T, String>,
    pub(crate) definitions: HashMap<T, String>,
}

fn annotation_value_to_str<T: ForIRI>(annotation: &AnnotationValue<T>) -> String {
    match annotation {
        AnnotationValue::Literal(l) => { l.literal().to_string() }
        AnnotationValue::IRI(iri) => { iri.to_string() }
        AnnotationValue::AnonymousIndividual(a) => {a.to_string()}
    }
}

impl<T: ForIRI> AxiomVisitor<T> for Annotations<T> {
    fn visit_annotation_assertion(&mut self, _subject: &AnnotationSubject<T>, _ann: &Annotation<T>, _target: &T) {
        match _subject {
            AnnotationSubject::IRI(iri) => {
                let ap_iri = _ann.ap.0.underlying();
                match ap_iri.to_string().as_str() {
                    RDFS_LABEL => { self.labels.insert(iri.underlying(), annotation_value_to_str(&_ann.av)); },
                    DEFINITION => { self.definitions.insert(iri.underlying(), annotation_value_to_str(&_ann.av)); },
                    _ => {}
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => {}
        }
    }
}