use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use horned_owl::model::{AnnotatedComponent, ArcStr, Build, Class, ClassExpression, Component, ForIRI, MutableOntology, SubClassOf, IRI};
use horned_owl::ontology::set::SetOntology;
use horned_owl::vocab::OWL;
use indicatif::ProgressIterator;
use rayon::prelude::*;
use whelk::whelk::model::{AtomicConcept, Axiom};
use whelk::whelk::owl::translate_ontology;
use whelk::whelk::reasoner::assert;
use crate::dependency::base::DependencyMap;
use crate::dependency::symbol::Symbol;

pub fn compute_semantic_dependency<'a,T:ForIRI + 'a>(ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>, extra_axiom_builder: fn(T) -> Rc<Axiom>, derive_dependencies_from_inferred_axiom: fn((Rc<AtomicConcept>, Rc<AtomicConcept>)) -> Vec<String>) -> DependencyMap<T, HashSet<&'a Component<T>>>{
    let axioms : Vec<_> = ontology_iter.collect();

    let mut declared_classes = HashSet::new();
    let mut declared_roles = HashSet::new();
    for ac in axioms.iter() {
        match &ac.component {
            Component::DeclareClass(dc) => {
                declared_classes.insert(&dc.0.0);
            },
            Component::DeclareObjectProperty(dop) => {
                declared_roles.insert(&dop.0.0);
            },
            _ => {}
        }
    }
    let builder = Build::<T>::new();
    let mut dependencies = HashMap::new();
    let ontology : SetOntology<T> = SetOntology::from_iter(axioms.into_iter().cloned());
    let mut whelk_axioms = translate_ontology(&ontology);
    for c in declared_classes.into_iter().progress() {

        let ax = extra_axiom_builder(c.underlying());
        whelk_axioms.insert(ax.clone());

        let whelk = assert(&whelk_axioms);
        for sub in whelk.named_subsumptions() {
            let derived_dependencies = derive_dependencies_from_inferred_axiom(sub);
            for r in derived_dependencies.into_iter() {
                let l = Symbol::Class(c.underlying());
                let r_iri = builder.iri(r);
                let r = Symbol::Class(r_iri.underlying());
                if !dependencies.contains_key(&l) {
                    dependencies.insert(l.clone(), HashMap::new());
                }
                dependencies.get_mut(&l).unwrap().insert(r.clone(), HashSet::new());
            }

        }
        whelk_axioms.remove(&ax);
    }
    dependencies.into_iter().collect()
}