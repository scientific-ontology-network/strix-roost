use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use horned_owl::model::{AnnotatedComponent, ClassExpression, Component, ForIRI, ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange};
use horned_owl::vocab::OWL;
use whelk::whelk::model::{Axiom, Concept, ConceptInclusion, AtomicConcept};

use crate::dependency::base::{build_top, DependencyBuilder, DependencyMap};
use crate::dependency::symbol::{Term, Symbol};
use crate::dependency::semantics_based::compute_semantic_dependency;
use crate::dependency::syntax_based::SyntaxBasedDependency;

pub struct SemanticEmptinessDependency {}


impl<T:ForIRI + Send + Sync> DependencyBuilder<T> for SemanticEmptinessDependency {

    fn build_dependencies<'a> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> DependencyMap<T, HashSet<&'a Component<T>>>

    {
        let ax_builder = |c:T| Rc::new(Axiom::ConceptInclusion(Rc::new(
        ConceptInclusion{
            subclass: Rc::new(Concept::AtomicConcept(Rc::new(AtomicConcept{id: c.to_string()}))),
            superclass: Rc::new(Concept::AtomicConcept(Rc::new(AtomicConcept{id: OWL::Nothing.to_string()})))})));
        compute_semantic_dependency(ontology_iter, ax_builder, derive_dependencies_from_inferred_axiom)
    }
}

fn derive_dependencies_from_inferred_axiom(sub: (Rc<AtomicConcept>, Rc<AtomicConcept>)) -> Vec<String>{
    let (a,b) = sub;
    if b.id == OWL::Nothing.to_string().as_str()  {
        [(*a).id.clone()].into()
    } else {
        Vec::new()
    }
}

pub struct SyntacticEmptinessDependency {}

impl<T:ForIRI> DependencyBuilder<T> for SyntacticEmptinessDependency {
    fn build_dependencies<'a>(ontology_iter: impl Iterator<Item=&'a AnnotatedComponent<T>>) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>>
    where
        T: 'a
    {
        Self::derive_from_axioms(ontology_iter)
    }
}

impl<T:ForIRI> SyntaxBasedDependency<T> for SyntacticEmptinessDependency {
    
    fn dependencies_from_object_property_expression(ope: &ObjectPropertyExpression<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(_op) => { HashSet::new() },
            ObjectPropertyExpression::InverseObjectProperty(_op) => {
                println!("Inverse object properties are not supported in syntactic emptiness dependency yet. Skipping!");
                HashSet::new() },
        }
    }

    fn dependencies_from_object_intersection_of<'a>(x: &'a ClassExpression<T>, ces: &'a Vec<ClassExpression<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(x), Term::CE(ce2))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_union_of<'a>(x: &'a ClassExpression<T>, ces: &'a Vec<ClassExpression<T>>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        ces.into_iter()
            .flat_map(|ce2| {
                [(Term::CE(ce2), Term::CE(x))]
                    .into_iter()
                    .chain(Self::dependencies_from_class_expression(ce2))
            })
            .collect()
    }

    fn dependencies_from_object_some_values_from<'a>(x: &'a ClassExpression<T>, ope: &'a ObjectPropertyExpression<T>, bce: &'a ClassExpression<T>) -> HashSet<(Term<'a, T>, Term<'a, T>)> {
        let base = match *bce == build_top() {
            true => {vec![(Term::Role(ope), Term::CE(x))]}
            false => {vec![]}
        };
        [
            (Term::CE(x), Term::CE(bce)),
            (Term::CE(x), Term::Role(ope)),
        ]
            .into_iter()
            .chain(base)
            .chain(Self::dependencies_from_class_expression(bce))
            .chain(Self::dependencies_from_object_property_expression(ope))
            .collect()
    }

    // domain(r) <= C
    fn dependency_from_object_property_domain(
        opd: &ObjectPropertyDomain<T>,
    ) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        ([(Term::Role(&opd.ope), Term::CE(&opd.ce))]) // r -> X, X -> C
            .into_iter()
            .chain(Self::dependencies_from_class_expression(&opd.ce))
            .chain(Self::dependencies_from_object_property_expression(&opd.ope))
            .collect()
    }

    // range(r) <= C
    fn dependency_from_object_property_range(_opr: &ObjectPropertyRange<T>) -> HashSet<(Term<'_, T>, Term<'_, T>)> {
        [
            (Term::Role(&_opr.ope), Term::CE(&_opr.ce)) // r -> X, X -> C
        ]
            .into_iter()
            .chain(Self::dependencies_from_class_expression(&_opr.ce))
            .chain(Self::dependencies_from_object_property_expression(&_opr.ope))
            .collect()
    }
}