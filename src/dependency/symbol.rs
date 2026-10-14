use horned_owl::model::{Class, ClassExpression, ForIRI, ObjectProperty, ObjectPropertyExpression};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub enum Symbol<T: ForIRI> {
    Class(T),
    Role(T),
}

impl<T: ForIRI> Symbol<T> {
    pub fn underlying(&self) -> &T {
        match self {
            Symbol::Class(t) => t,
            Symbol::Role(t) => t,
        }
    }
}

/// Represents a symbol in an ontology, which can be either a class expression or a role.
#[derive(Debug, Clone)]
pub enum Term<'a, T: ForIRI> {
    /// A reference to a class expression
    CE(&'a ClassExpression<T>),
    /// A reference to an object property expression (role)
    Role(&'a ObjectPropertyExpression<T>),
    /// A reference to a role composition
    RoleComposition(Vec<&'a ObjectPropertyExpression<T>>),
    /// A reference to a role composition
    InverseRole(&'a ObjectProperty<T>),
    Class(&'a Class<T>),
    ObjectProperty(&'a ObjectProperty<T>),
}


impl<'a, T: ForIRI> Term<'a, T> {
    pub fn get_iri(&self) -> Option<T> {
        match self {
            Term::CE(ClassExpression::Class(iri)) => Some(iri.underlying()),
            Term::Role(ObjectPropertyExpression::ObjectProperty(iri)) => Some(iri.underlying()),
            Term::Class(Class(iri)) => Some(iri.underlying()),
            Term::ObjectProperty(ObjectProperty(iri)) => Some(iri.underlying()),
            _ => None,
        }
    }

    pub fn get_symbol(&self) -> Option<Symbol<T>> {
        match self {
            Term::CE(ClassExpression::Class(iri)) => Some(Symbol::Class(iri.underlying())),
            Term::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {
                Some(Symbol::Role(iri.underlying()))
            },
            Term::Class(Class(iri)) => Some(Symbol::Class(iri.underlying())),
            Term::ObjectProperty(ObjectProperty(iri)) => Some(Symbol::Role(iri.underlying())),
            _ => None,
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Term::CE(ClassExpression::Class(_)) => true,
            Term::Role(ObjectPropertyExpression::ObjectProperty(_)) => true,
            Term::Class(_) => true,
            Term::ObjectProperty(_) => true,
            _ => false,
        }
    }
}

impl<'a, T: ForIRI> PartialEq<Self> for Term<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::CE(ClassExpression::Class(sc)),Term::Class(oc)) => sc == *oc,
            (Term::Class(oc),Term::CE(ClassExpression::Class(sc))) => sc == *oc,
            (Term::Role(ObjectPropertyExpression::ObjectProperty(sc)), Term::ObjectProperty(r)) => sc == *r,
            (Term::ObjectProperty(r), Term::Role(ObjectPropertyExpression::ObjectProperty(sc))) => sc == *r,
            (Term::CE(sc), Term::CE(oc)) => sc == oc,
            (Term::Role(sc), Term::Role(oc)) => sc == oc,
            (Term::RoleComposition(sc), Term::RoleComposition(oc)) => sc == oc,
            (Term::InverseRole(sc), Term::InverseRole(oc)) => sc == oc,
            (Term::Class(sc), Term::Class(oc)) => sc == oc,
            (Term::ObjectProperty(sc), Term::ObjectProperty(oc)) => sc == oc,
            (Term::Role(ObjectPropertyExpression::InverseObjectProperty(sc)), Term::InverseRole(oc)) => sc == *oc,
            (Term::InverseRole(sc), Term::Role(ObjectPropertyExpression::InverseObjectProperty(oc))) => *sc == oc,
            (_,_) => false,
        }
    }
}

impl<'a, T: ForIRI> Hash for Term<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Term::CE(sc) => (*sc).hash(state),
            Term::Class(sc) => (ClassExpression::Class((*sc).clone())).hash(state),

            Term::Role(sc) => sc.hash(state),
            Term::ObjectProperty(sc) => (ObjectPropertyExpression::ObjectProperty((*sc).clone())).hash(state),
            Term::InverseRole(sc) => (ObjectPropertyExpression::InverseObjectProperty((*sc).clone())).hash(state),

            Term::RoleComposition(sc) => (*sc).hash(state)
        }
    }
}

impl<'a, T: ForIRI> Eq for Term<'a, T> {}
