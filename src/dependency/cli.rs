pub use crate::cli::base::Runnable;
use crate::dependency::base::DependencyBuilder;
use crate::dependency::empty::{SemanticEmptinessDependency, SyntacticEmptinessDependency};
use crate::dependency::everything::{SemanticEverythingDependency, SyntacticEverythingDependency};
use crate::dependency::growth::{remove_super_symbols, GrowthDependency};
use crate::dependency::llm::ask;
use crate::ontology::io::load_set_ontology;
use crate::ontology::processors::annotations::Annotations;
use crate::ontology::visitor::AxiomVisitor;
use clap::Parser;
use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::model::ArcStr;
use indicatif::ProgressIterator;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DependencyWriter {
    #[arg(short, long)]
    method: String,

    #[arg(short, long)]
    in_path: std::path::PathBuf,

    #[arg(short, long)]
    llm: Option<bool>,
}

impl Runnable<()> for DependencyWriter {
    fn run(&self) {
        let path = Path::new(&self.in_path);
        let fname = format!(
            "{}_dep_{}.json",
            path.file_stem().unwrap().to_str().unwrap(),
            self.method
        );
        let path = path.with_file_name(fname);
        println!("Writing results to {:?}", path);
        let file = File::create(path).expect("Failed to create file");
        let onto =
            load_set_ontology(self.in_path.to_str().unwrap()).expect("Failed to load ontology");
        let set_index = onto.i();
        let dependency_mechanism = match self.method.as_str() {
            "growth" => GrowthDependency::build_dependencies,
            "empty_sem" => SemanticEmptinessDependency::build_dependencies,
            "empty" => SyntacticEmptinessDependency::build_dependencies,
            "everything_sem" => SemanticEverythingDependency::build_dependencies,
            "everything" => SyntacticEverythingDependency::build_dependencies,
            _ => panic!("Unknown dependency mechanism {}", self.method),
        };

        let dependencies = dependency_mechanism(set_index.iter());
        let cleaned_dependencies = remove_super_symbols(&dependencies, set_index.iter());
        let mut annotations = Annotations::<_>::default();
        let placeholder = ArcStr::from("");
        annotations.visit_components(set_index.iter(), &placeholder);
        let mut results = HashMap::new();
        if self.llm.unwrap_or(false) {
            for (a, vs) in cleaned_dependencies.iter().progress() {
                let a_t = a.underlying();
                match ask(a_t, vs, &annotations.definitions, &annotations.labels) {
                    Ok(result) => {
                        if !result.is_empty() {
                            let mut r = Vec::new();
                            for (k, (k_ax, should_be_dependent)) in result {
                                let k_iri = k.underlying();
                                let ax_list: Vec<String> = k_ax
                                    .iter()
                                    .map(|&ax| ax.as_functional().to_string())
                                    .collect();
                                r.push(json!({"iri": k_iri.to_string(), "llm": should_be_dependent, "cause": ax_list}));
                            }
                            results.insert(a_t.to_string(), r);
                        }
                    }
                    Err(e) => {
                        println!(
                            "Error querying LLM for dependencies of <{}>: {} -- Skipping",
                            a_t, e
                        )
                    }
                }
            }
        } else {
            for (a, vs) in cleaned_dependencies.iter().progress() {
                let mut r = Vec::new();
                for (k, k_ax) in vs {
                    let k_iri = k.underlying();
                    let ax_list: Vec<String> = k_ax
                        .iter()
                        .map(|&ax| ax.as_functional().to_string())
                        .collect();
                    r.push(json!({"iri": k_iri.to_string(), "cause": ax_list}));
                }
                results.insert(a.underlying().to_string(), r);
            }
        }

        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &results).expect("Failed to write JSON to file");
    }
}
