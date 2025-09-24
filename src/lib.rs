use std::fs::File;
use std::io::{BufReader};
use std::sync::Arc;
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::{read_with_build, ConcreteRDFOntology};
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::{ArcAnnotatedComponent, ArcStr, Build};

mod dependency;
use dependency::base::DependencyBuilder;
use dependency::growth::GrowthDependency;

mod util;

fn main() {

    let path = "/home/glauer/Downloads/build-files/oeo/2.8.0/oeo-full.owl";
    let onto = match load_rdf_ontology(path) {
        Ok(onto) => onto,
        Err(e) => panic!("{:?}", e),
    };

    let (set_index,_,_) = onto.index();
    let dependencies = GrowthDependency::dep(set_index.iter().map(|arc|&**arc));
    for (a,b) in dependencies{
        println!("{:?} -> {:?}",a,b)
    }
}

fn load_rdf_ontology(path: &str) -> Result<ConcreteRDFOntology<ArcStr, ArcAnnotatedComponent>, HornedError>{
    let file = File::open(path)?;
    let reader = &mut BufReader::new(file);
    let build = Build::new_arc();
    let res = read_with_build::<ArcStr, ArcAnnotatedComponent, BufReader<File>>(reader, &build, ParserConfiguration {
        rdf: RDFParserConfiguration { lax: true },
        ..Default::default()
    },)?;
    Ok(res.0)
}