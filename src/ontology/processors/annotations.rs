use crate::iris::iao::DEFINITION;
use crate::iris::rdfs::RDFS_LABEL;
use crate::ontology::visitor::AxiomVisitor;
use core::default::Default;
use horned_owl::model::{Annotation, AnnotationSubject, AnnotationValue, ForIRI, Literal};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct Annotations<T: ForIRI> {
    pub(crate) labels: HashMap<T, Vec<Literal<T>>>,
    pub(crate) definitions: HashMap<T, Vec<Literal<T>>>,
}

impl<T: ForIRI> AxiomVisitor<T> for Annotations<T> {
    fn visit_annotation_assertion(
        &mut self,
        _subject: &AnnotationSubject<T>,
        _ann: &Annotation<T>,
        _target: &T,
    ) {
        let get_lit =  |av| match av {
            AnnotationValue::Literal(l) => l,
            _ => panic!("Annotation value is not a literal"),
        };
        match _subject {
            AnnotationSubject::IRI(iri) => {
                let ap_iri = _ann.ap.0.underlying();
                match ap_iri.to_string().as_str() {
                    RDFS_LABEL => {
                        self.labels
                            .entry(iri.underlying()).or_insert(Vec::new()).push(get_lit(_ann.av.clone()));
                    }
                    DEFINITION => {
                        self.definitions
                            .entry(iri.underlying()).or_insert(Vec::new()).push(get_lit(_ann.av.clone()));
                    }
                    _ => {}
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => {}
        }
    }
}


pub fn filter_literals_by_language<'a, T:ForIRI + 'a>(literals: Vec<&'a Literal<T>>, target_lang: &String, include_untagged:bool) -> Vec<&'a String>{
    literals.iter().filter_map(|l| match l{
        Literal::Simple { literal } => { if include_untagged {Some(literal)} else {None}}
        Literal::Language { literal  , lang} => { if (lang == target_lang) {Some(literal)} else {None}}
        Literal::Datatype { .. } => {None}
    }).collect()
}