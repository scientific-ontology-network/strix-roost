use crate::iris::iao::DEFINITION;
use crate::iris::rdfs::RDFS_LABEL;
use crate::ontology::visitor::AxiomVisitor;
use core::default::Default;
use horned_owl::model::{Annotation, AnnotationSubject, AnnotationValue, ArcStr, ForIRI, Literal};
use horned_owl::ontology::set::SetOntology;
use std::collections::{HashMap, HashSet};

pub(crate) struct Annotations<'a, T: ForIRI> {
    pub(crate) labels: HashMap<T, Vec<&'a Literal<T>>>,
    pub(crate) definitions: HashMap<T, Vec<&'a Literal<T>>>,
}

impl<'a, T: ForIRI> Default for Annotations<'a, T> {
    fn default() -> Self {
        Annotations {
            labels: HashMap::new(),
            definitions: HashMap::new(),
        }
    }
}

impl<'a, T: ForIRI> From<&'a SetOntology<T>> for Annotations<'a, T> {
    fn from(ontology: &'a SetOntology<T>) -> Self {
        let mut labels = HashMap::new();
        let mut definitions = HashMap::new();
        for anno in AnnotationsVisitor::visit_components(ontology.i().iter(), None) {
            match anno {
                AnnotationPair::Labels((t, lit)) => labels.entry(t).or_insert(Vec::new()).push(lit),
                AnnotationPair::Definitions((t, lit)) => {
                    definitions.entry(t).or_insert(Vec::new()).push(lit)
                }
            }
        }
        Annotations {
            labels,
            definitions,
        }
    }
}

fn get_lit<'a, T: ForIRI>(av: &'a AnnotationValue<T>) -> &'a Literal<T> {
    match av {
        AnnotationValue::Literal(l) => l,
        _ => panic!("Annotation value is not a literal"),
    }
}

enum AnnotationPair<'a, T: ForIRI> {
    Labels((T, &'a Literal<T>)),
    Definitions((T, &'a Literal<T>)),
}
pub struct AnnotationsVisitor {}

impl<'a, T: ForIRI> AxiomVisitor<'a, T, AnnotationPair<'a, T>> for AnnotationsVisitor {
    fn visit_annotation_assertion(
        _subject: &'a AnnotationSubject<T>,
        _ann: &'a Annotation<T>,
        _target: Option<&'a T>,
    ) -> Option<AnnotationPair<'a, T>> {
        match _subject {
            AnnotationSubject::IRI(iri) => {
                let ap_iri = _ann.ap.0.underlying();
                match ap_iri.to_string().as_str() {
                    RDFS_LABEL => Some(AnnotationPair::Labels((
                        iri.underlying(),
                        get_lit(&_ann.av),
                    ))),
                    DEFINITION => Some(AnnotationPair::Definitions((
                        iri.underlying(),
                        get_lit(&_ann.av),
                    ))),
                    _ => None,
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => None,
        }
    }
}

fn get_english_literal<'a, T: ForIRI>(
    iri: &'a T,
    annotations: HashMap<T, Vec<&'a Literal<T>>>,
) -> String {
    let annos = match annotations.get(iri) {
        Some(a) => a,
        None => return iri.to_string(),
    };

    annos
        .iter()
        .find_map(|l| match l {
            Literal::Language { lang, literal } if lang == "en" => Some((*literal).clone()),
            _ => None,
        })
        .or_else(|| {
            annos.iter().find_map(|l| match l {
                Literal::Simple { literal } => Some((*literal).clone()),
                _ => None,
            })
        })
        .unwrap_or_else(|| iri.to_string())
}
