use clap::Parser;
use horned_owl::model::ArcStr;
use horned_owl::ontology::set::SetOntology;

pub trait Runnable: Parser {
    fn run(&self, onto: SetOntology<ArcStr>);
}
