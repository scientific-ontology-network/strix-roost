use crate::util::error::StrixError;
use clap::Parser;
use horned_owl::model::{ArcStr, ForIRI};
use horned_owl::ontology::set::SetOntology;

pub trait Runnable: Parser {
    fn run(&self, onto: SetOntology<ArcStr>);
}
