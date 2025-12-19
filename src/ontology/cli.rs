pub use crate::cli::base::Runnable;
use crate::ontology::processors::annotations::Annotations;
use crate::ontology::visitor::AxiomVisitor;
use clap::Parser;
use horned_owl::model::{ArcStr, Literal};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;
use horned_owl::ontology::set::SetOntology;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AnnotationWriter {
    #[arg(short, long)]
    out_path: std::path::PathBuf,
}

impl Runnable for AnnotationWriter {
    fn run(&self, onto: SetOntology<ArcStr>) {

        let mut annotations = Annotations::<_>::default();
        let placeholder = ArcStr::from("");
        annotations.visit_components(onto.i().iter(), &placeholder);
        let render_literal = |l: &Literal<ArcStr>| match l {
            Literal::Simple { literal } => json!({"type": "simple", "literal":literal.to_string()}),
            Literal::Language { literal , lang} => json!({"type": "language", "literal":literal.to_string(), "lang":lang.to_string()}),
            Literal::Datatype { literal, datatype_iri } => json!({"type": "datatype", "literal":literal.to_string(), "datatype":datatype_iri.to_string()})
        };
        let iris : HashSet<_> = annotations.definitions.keys().chain(annotations.labels.keys()).collect();
        let result: HashMap<_,_> = iris.iter().map(|&k| (k.to_string(), json!({
            "definitions": annotations.definitions.get(k).unwrap_or(&Vec::new()).iter().map(render_literal).collect::<Vec<_>>(),
            "labels": annotations.labels.get(k).unwrap_or(&Vec::new()).iter().map(render_literal).collect::<Vec<_>>()}))).collect();

        let result = json!(result);

        let file = File::create(self.out_path.clone()).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &result).expect("Failed to write JSON to file");
    }
}
