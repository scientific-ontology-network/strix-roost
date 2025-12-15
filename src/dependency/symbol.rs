use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use horned_owl::model::{ArcStr, ClassExpression, ForIRI, ObjectPropertyExpression};
use crate::util::error::StrixError;

/// Represents a symbol in an ontology, which can be either a class expression or a role.
#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub enum OntologySymbol<'a, T: ForIRI> {
    /// A reference to a class expression
    CE(&'a ClassExpression<T>),
    /// A reference to an object property expression (role)
    Role(&'a ObjectPropertyExpression<T>),
}

impl<'a, T:ForIRI> OntologySymbol<'a, T>{
    pub(crate) fn get_iri(&self) -> Option<T>{
        match self {
            OntologySymbol::CE(ClassExpression::Class(iri)) => {Some(iri.underlying())}
            OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {Some(iri.underlying())}
            _ => None
        }
    }
}

pub trait ForSymbol: Eq + Hash + PartialEq + Debug + Clone {
    fn is_atomic(&self) -> bool;

}

#[derive(Debug, Eq, Clone, Hash, PartialEq)]
pub struct StringSymbol {
    symbol: String,
}

impl StringSymbol {
    pub fn new(symbol: String) -> StringSymbol {
        StringSymbol{symbol}
    }
}


impl ForSymbol for StringSymbol {
    fn is_atomic(&self) -> bool {true}
}


impl<'a,T: ForIRI> ForSymbol for OntologySymbol<'a, T> {
    fn is_atomic(&self) -> bool {
        match self {
            OntologySymbol::CE(ClassExpression::Class(_)) => true,
            OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(_)) => true,
            _ => false,
        }
    }
}



pub trait SymbolContainer<S: ForSymbol, C>: ForSymbol {
    fn get_symbol(&self) -> &S;

    fn get_underlying(&self) -> Option<&C>{
        None
    }

    fn merge_include_information(&self, _other: &Self) -> Self{
        self.clone()
    }

    fn from(x:S) -> Self;

    fn from_symbol_and_axiom(x: S, _c: C) -> Self{
        Self::from(x)
    }

}

impl<'a, T: ForIRI, C> SymbolContainer<OntologySymbol<'a, T>, C> for OntologySymbol<'a, T> {
    fn get_symbol(&self) -> &OntologySymbol<'a, T> {
        &self
    }

    fn from(x: OntologySymbol<'a, T>) -> Self {
        x
    }

}

impl<C> SymbolContainer<StringSymbol, C> for StringSymbol {
    fn get_symbol(&self) -> &StringSymbol {
        &self
    }

    fn from(x: StringSymbol) -> Self {
        x
    }

}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct DependencySymbol<S> where S: Hash + Eq + PartialEq + Debug + Clone {
    symbol: S,
}

impl<S: ForSymbol> ForSymbol for DependencySymbol<S> {
    fn is_atomic(&self) -> bool {
        self.symbol.is_atomic()
    }
}


impl<S: ForSymbol, C> SymbolContainer<S, C> for DependencySymbol<S> {
    fn get_symbol(&self) -> &S{
        &self.symbol
    }

    fn from(x: S) -> Self {
        Self { symbol: x}
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct DependencySymbolWithAxioms<S, Ax: Hash + Eq + PartialEq + Debug + Clone> {
    symbol: S,
    axioms: Vec<Ax>,
}

impl <S: ForSymbol, Ax: Hash + Eq + PartialEq + Debug + Clone> DependencySymbolWithAxioms<S, Ax> {
    pub(crate) fn get_axioms(&self) -> &Vec<Ax> {
        &self.axioms
    }
}

impl<S: ForSymbol, C: Hash + Eq + PartialEq + Debug + Clone> ForSymbol for DependencySymbolWithAxioms<S, C> {
    fn is_atomic(&self) -> bool {
        self.symbol.is_atomic()
    }
}

impl<S: ForSymbol, Ax:  Hash + Eq + PartialEq + Debug + Clone> SymbolContainer<S, Vec<Ax>> for DependencySymbolWithAxioms<S, Ax> {
    fn get_symbol(&self) -> &S {
        &self.symbol
    }

    fn get_underlying(&self) -> Option<&Vec<Ax>> {
        Some(&self.axioms)
    }

    fn merge_include_information(&self, other: &Self) -> Self{
        DependencySymbolWithAxioms {
            symbol: self.symbol.clone(),
            axioms: self.axioms.iter().chain(other.axioms.iter()).cloned().collect(),
        }
    }

    fn from(x: S) -> Self {
        Self { symbol: x, axioms: vec![] }
    }

    fn from_symbol_and_axiom(x: S, _c: Vec<Ax>) -> Self {
        Self {symbol: x, axioms:_c}
    }
}

pub type DependencyMap<S, D> = HashMap<S, HashSet<D>>;