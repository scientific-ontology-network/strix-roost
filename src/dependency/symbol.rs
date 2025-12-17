use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use horned_owl::model::{ClassExpression, ForIRI, ObjectPropertyExpression};

#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub enum Symbol<T: ForIRI> {
    Class(T),
    Role(T)
}

impl<T: ForIRI> Symbol<T> {
    pub(crate) fn underlying(&self) -> &T {
        match self {
            Symbol::Class(t) => {t}
            Symbol::Role(t) => {t}
        }
    }
}

/// Represents a symbol in an ontology, which can be either a class expression or a role.
#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub enum Term<'a, T: ForIRI> {
    /// A reference to a class expression
    CE(&'a ClassExpression<T>),
    /// A reference to an object property expression (role)
    Role(&'a ObjectPropertyExpression<T>),
}

impl<'a, T:ForIRI> Term<'a, T>{
    pub fn get_iri(&self) -> Option<T>{
        match self {
            Term::CE(ClassExpression::Class(iri)) => {Some(iri.underlying())}
            Term::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {Some(iri.underlying())}
            _ => None
        }
    }

    pub fn get_symbol(&self) -> Option<Symbol<T>>{
        match self {
            Term::CE(ClassExpression::Class(iri)) => {Some( Symbol::Class(iri.underlying()))}
            Term::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {Some(Symbol::Role(iri.underlying()))}
            _ => None
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Term::CE(ClassExpression::Class(_)) => true,
            Term::Role(ObjectPropertyExpression::ObjectProperty(_)) => true,
            _ => false,
        }
    }
}
