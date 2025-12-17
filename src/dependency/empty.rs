use std::any::Any;
use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotatedComponent, Build, Class, ClassExpression, Component, ForIRI, MutableOntology, ObjectPropertyExpression, ObjectPropertyRange, SubClassOf, SubObjectPropertyOf};
use horned_owl::ontology::indexed::ForIndex;
use horned_owl::ontology::set::SetOntology;
use horned_owl::vocab::OWL;
use indicatif::ProgressIterator;
use whelk::whelk::model::AtomicConcept;
use crate::dependency::base::{DependencyBuilder, DependencyMap, ComplexDependencyMap};
use crate::dependency::symbol::{Term, Symbol};
use crate::util::graph::transitive_closure;
use whelk::whelk::owl::translate_ontology;
use whelk::whelk::reasoner::assert;
use crate::dependency::syntax_based::SyntaxBasedDependency;

pub struct SemanticEmptinessDependency {}


impl<T:ForIRI> DependencyBuilder<T> for SemanticEmptinessDependency {
    fn build_dependencies<'a> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<T, HashSet<&'a Component<T>>>
    {
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
        let mut ontology : SetOntology<T> = SetOntology::from_iter(axioms.into_iter().cloned());
        for c in declared_classes.into_iter().progress() {
            let ax = AnnotatedComponent { component: Component::SubClassOf(SubClassOf{sub: ClassExpression::Class(Class(c.clone())), sup:ClassExpression::Class(Class(builder.iri(OWL::Nothing))) }), ann: Default::default() };
            ontology.insert(ax.clone());
            let whelk_axioms = translate_ontology(&ontology);
            let whelk = assert(&whelk_axioms);
            for (sub, sup) in whelk.named_subsumptions() {
                if (*sup).id == OWL::Nothing.to_string().as_str() {
                    let l = Symbol::Class(c.underlying());
                    let r_iri = builder.iri((*sub).id.clone());
                    let r = Symbol::Class(r_iri.underlying());
                    if !dependencies.contains_key(&l) {
                        dependencies.insert(l.clone(), HashMap::new());
                    }
                    dependencies.get_mut(&l).unwrap().insert(r.clone(), HashSet::new());
                }
            }
            ontology.remove(&ax);
        }
        dependencies.into_iter().collect()
    }
}

pub struct SyntacticEmptinessDependency {}

impl<T:ForIRI> DependencyBuilder<T> for SyntacticEmptinessDependency {
    fn build_dependencies<'a>(ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>>
    where
        T: 'a
    {
        todo!()
    }
}

impl<T:ForIRI> SyntaxBasedDependency<T> for SyntacticEmptinessDependency {
    
    fn dependencies_from_object_property_expression(ope: &ObjectPropertyExpression<T>) -> HashSet<(Term<T>, Term<T>)> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(op) => { HashSet::new() },
            ObjectPropertyExpression::InverseObjectProperty(op) => { panic!("Inverse object properties are not supported in syntactic emptiness dependency yet") },
        }
    }


    // X = range(r)
    fn dependency_from_object_property_range(_opr: &ObjectPropertyRange<T>) -> HashSet<(Term<T>, Term<T>)> {
        [
            (Term::CE(&_opr.ce),Term::Role(&_opr.ope)), // X -> r
            (Term::Role(&_opr.ope), Term::CE(&_opr.ce)) // r -> X
        ].into_iter().chain(Self::dependencies_from_class_expression(&_opr.ce)).chain(Self::dependencies_from_object_property_expression(&_opr.ope)).collect()
    }
}