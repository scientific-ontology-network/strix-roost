use crossbeam::{channel, scope};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::thread;
use horned_owl::model::{AnnotatedComponent, Build, Component, ForIRI, MutableOntology};
use horned_owl::ontology::set::SetOntology;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use whelk::whelk::model::{AtomicConcept, Axiom};
use whelk::whelk::owl::translate_ontology;
use whelk::whelk::reasoner::assert;
use crate::dependency::base::DependencyMap;
use crate::dependency::symbol::Symbol;

pub fn compute_semantic_dependency<'a,T:ForIRI + 'a + Send + Sync>(ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>, extra_axiom_builder: fn(T) -> Rc<Axiom>, derive_dependencies_from_inferred_axiom: fn((Rc<AtomicConcept>, Rc<AtomicConcept>)) -> Vec<String>) -> DependencyMap<T, HashSet<&'a Component<T>>>{
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

    let declared_classes = declared_classes.into_iter().map(|iri| iri.underlying()).collect::<Vec<_>>();

    let ontology : SetOntology<T> = SetOntology::from_iter(axioms.into_iter().cloned());
    let (tx, rx) = channel::bounded::<(Symbol<T>, Symbol<T>)>(1024);

    // keep one sender in this scope
    let tx_for_workers = tx.clone();

    let dependencies = scope(|s| {
        let consumer = s.spawn(|_| {
            let mut deps: HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<_>>> =
                HashMap::new();

            for (l, r) in rx.iter() {
                deps.entry(l)
                    .or_insert_with(HashMap::new)
                    .entry(r)
                    .or_insert_with(HashSet::new);
            }

            deps
        });

        declared_classes
            .into_par_iter()
            .progress()
            .flat_map(|c| {
                calculate_dependencies(
                    &ontology,
                    c,
                    extra_axiom_builder,
                    derive_dependencies_from_inferred_axiom,
                )
            })
            .for_each_with(tx_for_workers, |tx, item| {
                tx.send(item).expect("consumer alive");
            });

        drop(tx); // ✅ closes the channel when workers are done

        consumer.join().unwrap()
    })
        .unwrap();


    dependencies.into_iter().collect()
}

fn calculate_dependencies<T: ForIRI>(
    ontology: &SetOntology<T>,
    c: T,
    extra_axiom_builder: fn(T) -> Rc<Axiom>,
    derive_dependencies_from_inferred_axiom: fn((Rc<AtomicConcept>, Rc<AtomicConcept>)) -> Vec<String>)
    -> Vec<(Symbol<T>, Symbol<T>)>
{

    let mut result = Vec::new();
    let builder = Build::<T>::new();
    let mut whelk_axioms = translate_ontology(&ontology);
    let ax = extra_axiom_builder(c.clone());
    whelk_axioms.insert(ax.clone());

    let whelk = assert(&whelk_axioms);
    for sub in whelk.named_subsumptions() {
        let derived_dependencies = derive_dependencies_from_inferred_axiom(sub);
        for r in derived_dependencies.into_iter() {
            let l = Symbol::Class(c.clone());
            let r_iri = builder.iri(r);
            let r = Symbol::Class(r_iri.underlying());
            result.push((r,l));
        }

    }
    result
}