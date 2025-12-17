use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use clap::Parser;
use horned_owl::model::{ArcStr, Component};
use horned_owl::io::ofn::writer::AsFunctional;
use crate::ontology::processors::annotations::Annotations;
use crate::dependency::base::{reduce_map, DependencyBuilder};
use crate::dependency::growth::{remove_super_symbols, GrowthDependency};
use crate::ontology::io::load_set_ontology;
use serde_json::json;
use crate::ontology::visitor::AxiomVisitor;
use indicatif::ProgressIterator;
pub use crate::cli::base::Runnable;
use crate::dependency::empty::EmptinessDependency;
use crate::dependency::llm::ask;
use crate::dependency::symbol::{Term};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DependencyWriter {
    #[arg(short, long)]
    method: String,

    #[arg(short, long)]
    in_path: std::path::PathBuf,

    #[arg(short, long)]
    out_path: std::path::PathBuf,



    #[arg(short, long)]
    llm: Option<bool>,
}


impl Runnable<()> for DependencyWriter {
    fn run(&self) {
        let path = Path::new(&self.out_path);
        let file = File::create(path).expect("Failed to create file");
        let onto = load_set_ontology(self.in_path.to_str().unwrap());
        let set_index = onto.i();
        let dependencies;
        if self.method == "growth" {
            dependencies = GrowthDependency::build_dependencies(set_index.iter());
        } else if self.method == "emptiness" {
            dependencies = EmptinessDependency::build_dependencies(set_index.iter());
        } else {
            panic!("Unknown method {}", self.method);
        }
        //let dependencies = reduce_map(raw_dependencies);
        let cleaned_dependencies = remove_super_symbols(&dependencies, set_index.iter());
        let mut annotations = Annotations::<_>::default();
        let placeholder = ArcStr::from("");
        annotations.visit_components(set_index.iter(), &placeholder);
        let mut results = HashMap::new();
        for (a, vs) in cleaned_dependencies.iter().progress(){
            let a_t = a.underlying();
            match ask(a_t, vs, &annotations.definitions, &annotations.labels) {
                Ok(result) => {
                    if !result.is_empty() {
                        let mut r = Vec::new();
                        for (k, (k_ax, should_be_dependent)) in result {
                            let k_iri = k.underlying();
                            let ax_list: Vec<String> = k_ax.iter().map(|&ax| ax.as_functional().to_string()).collect();
                            r.push(json!({"iri": k_iri.to_string(), "llm": should_be_dependent, "cause": ax_list}));
                        }
                        results.insert(a_t.to_string(), r);
                    }
                }
                Err(e) => {println!("Error querying LLM for dependencies of <{}>: {} -- Skipping", a_t, e)}
            }
        }

        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &results).expect("Failed to write JSON to file");

    }


}

