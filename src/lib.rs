use std::fs::File;
use std::io::{BufReader};
use std::sync::Arc;
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::{read_with_build, ConcreteRDFOntology};
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::{ArcAnnotatedComponent, ArcStr, Build};

pub mod dependency;
pub(crate) mod util;


