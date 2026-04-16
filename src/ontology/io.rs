use crate::util::error::StrixError;
use horned_owl::io::{ofn, rdf, ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use std::fs::File;
use std::io::BufReader;

pub fn load_set_ontology(path: &str) -> Result<SetOntology<ArcStr>, StrixError> {
    let ending = path.split(".").last().unwrap();
    match ending {
        "owl" => load_rdf_ontology(&path),
        "ofn" => load_ofn_ontology(&path),
        _ => Err(StrixError::InternalStrixError {
            message: format!("Unknown file ending: {}", ending),
        }),
    }
}

fn load_rdf_ontology(path: &str) -> Result<SetOntology<ArcStr>, StrixError> {
    let file = File::open(path)?;
    let reader = &mut BufReader::new(file);
    let build = Build::new_arc();
    let res = rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, BufReader<File>>(
        reader,
        &build,
        ParserConfiguration {
            rdf: RDFParserConfiguration {
                format: None,
                lax: true,
            },
            ..Default::default()
        },
    );
    match res {
        Ok(oc) => Ok(oc.0.into()),
        Err(e) => panic!("Error loading ontology: {}", e),
    }
}

fn load_ofn_ontology(path: &str) -> Result<SetOntology<ArcStr>, StrixError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let build = Build::new_arc();
    let res = ofn::reader::read_with_build::<ArcStr, SetOntology<ArcStr>, BufReader<File>>(
        reader, &build,
    );
    match res {
        Ok(oc) => Ok(oc.0),
        Err(e) => panic!("Error loading ontology: {}", e),
    }
}
