use crate::util::error::StrixError;
use horned_owl::io::{rdf, ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::*;
use horned_owl::ontology::set::SetOntology;
use std::fs::File;
use std::io::BufReader;
use horned_owl::io::rdf::reader::ConcreteRDFOntology;

pub fn load_set_ontology(path: &str) -> SetOntology<ArcStr> {
    let ending = path.split(".").last().unwrap();
    let res = match ending {
        "owl" => load_rdf_ontology(&path),
        _ => Err(StrixError::InternalStrixError {
            message: format!("Unknown file ending: {}", ending),
        }),
    };
    match res {
        Ok(oc) => oc.into(),
        Err(e) => panic!("Error loading ontology: {}", e),
    }
}

fn load_rdf_ontology(path: &str) -> Result<ConcreteRDFOntology<ArcStr, ArcAnnotatedComponent>, StrixError> {
    let file = File::open(path)?;
    let reader = &mut BufReader::new(file);
    let build = Build::new_arc();
    let (ontology, _incomplete_parse) =
        rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, BufReader<File>>(
            reader,
            &build,
            ParserConfiguration {
                rdf: RDFParserConfiguration { lax: true },
                ..Default::default()
            },
        )?;

    Ok(ontology)
}
