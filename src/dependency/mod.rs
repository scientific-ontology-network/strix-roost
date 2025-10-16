use std::fs::File;
use std::io::BufReader;
use horned_owl::error::HornedError;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
use horned_owl::io::rdf::reader::{read_with_build, ConcreteRDFOntology};
use horned_owl::model::{ArcAnnotatedComponent, ArcStr, Build};

pub mod base;
pub mod growth;


