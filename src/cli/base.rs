use horned_owl::model::ForIRI;
use horned_owl::ontology::set::SetOntology;

pub trait Runnable<T: ForIRI> {
    fn run(&self, onto: SetOntology<T>);
}
