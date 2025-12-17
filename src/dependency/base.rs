//! This module provides functionality for building and managing dependency relationships in ontologies.
//! It defines structures and traits for representing and analyzing relationships between different
//! ontological components such as class and property symbols.

use std::collections::{HashMap, HashSet};
use horned_owl::model::SubClassOf as SCO;
use horned_owl::model::*;
use horned_owl::ontology::indexed::ForIndex;
use itertools::Itertools;
use crate::dependency::symbol::{Symbol, Term};


pub type ComplexDependencyMap<'a, T: ForIRI, C> = HashMap<Term<'a, T>, HashMap<Term<'a, T>, C>>;
pub type DependencyMap<T: ForIRI, C> = HashMap<Symbol<T>, HashMap<Symbol<T>, C>>;

fn get_symbol<T: ForIRI>(t: Term<T>) -> Symbol<T> {
    match t {
        Term::CE(ClassExpression::Class(Class(iri))) => Symbol::Class(iri.underlying()),
        Term::Role(ObjectPropertyExpression::ObjectProperty(ObjectProperty(iri))) => Symbol::Role(iri.underlying()),
        _ => panic!("Trying to symbolize non-atomic expression: {t:?}")
    }
}

/// Trait for building dependency relationships between ontological components
pub trait DependencyBuilder<T:ForIRI> {
    /// Constructs a dependency map from an iterator of annotated components
    ///
    /// # Arguments
    /// * `ontology_iter` - An iterator over annotated ontology components
    fn build_dependencies<'a> (
        ontology_iter: impl Iterator<Item = &'a AnnotatedComponent<T>>,
    ) -> HashMap<Symbol<T>, HashMap<Symbol<T>, HashSet<&'a Component<T>>>> where T: 'a;
}

