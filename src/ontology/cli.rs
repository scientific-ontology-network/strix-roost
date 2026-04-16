pub use crate::cli::base::Runnable;
use crate::ontology::processors::annotations::{Annotations, AnnotationsVisitor};
use crate::ontology::visitor::AxiomVisitor;
use clap::Args;
use horned_owl::model::AnnotationValue::AnonymousIndividual;
use horned_owl::model::{ArcStr, ForIRI, Literal};
use horned_owl::ontology::set::SetOntology;
use itertools::Itertools;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufWriter;

// Extract labels and definitions
#[derive(Args)]
pub struct AnnotationWriter {
    #[arg(short, long)]
    out_path: std::path::PathBuf,
}

impl<T: ForIRI> Runnable<T> for AnnotationWriter {
    fn run(&self, onto: SetOntology<T>) {
        let annotations = Annotations::from(&onto);
        let render_literal = |l: &&Literal<T>| match l {
            Literal::Simple { literal } => json!({"type": "simple", "literal":literal.to_string()}),
            Literal::Language { literal, lang } => {
                json!({"type": "language", "literal":literal.to_string(), "lang":lang.to_string()})
            }
            Literal::Datatype {
                literal,
                datatype_iri,
            } => {
                json!({"type": "datatype", "literal":literal.to_string(), "datatype":datatype_iri.to_string()})
            }
        };
        let iris: HashSet<_> = annotations
            .definitions
            .keys()
            .chain(annotations.labels.keys())
            .collect();
        let json_anno = json!(iris.into_iter().map(|k| (k.to_string(), json!({
            "definitions": annotations.definitions.get(&k).unwrap_or(&Vec::new()).into_iter().map(render_literal).collect::<Vec<Value>>(),
            "labels": annotations.labels.get(&k).unwrap_or(&Vec::new()).iter().map(render_literal).collect::<Vec<Value>>()}))).collect::<Vec<_>>());
        let file = File::create(self.out_path.clone()).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &json_anno).expect("Failed to write JSON to file");
    }
}
