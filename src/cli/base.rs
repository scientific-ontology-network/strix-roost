use horned_owl::model::ArcStr;
use horned_owl::ontology::set::SetOntology;

pub trait Runnable {
    fn run(&self, onto: SetOntology<ArcStr>);
}
