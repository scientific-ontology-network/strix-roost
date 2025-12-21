pub use crate::cli::base::Runnable;
use crate::dependency::base::{remove_super_symbols, DependencyBuilder};
use crate::dependency::empty::{SemanticEmptinessDependency, SyntacticEmptinessDependency};
use crate::dependency::everything::{SemanticEverythingDependency, SyntacticEverythingDependency};
use crate::dependency::growth::{GrowthDependency};
use crate::dependency::llm::ask;
use crate::ontology::processors::annotations::{filter_literals_by_language, Annotations};
use crate::ontology::visitor::AxiomVisitor;
use clap::{Args};
use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::model::ArcStr;
use indicatif::ProgressIterator;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use horned_owl::ontology::set::SetOntology;
use crate::dependency::hop::HopDependency;

// Derive dependencies
#[derive(Args)]
pub struct DependencyWriter {
    #[arg(short, long)]
    method: String,

    #[arg(short, long)]
    out_path: std::path::PathBuf,

    #[arg(short, long)]
    llm: bool,
}

impl Runnable for DependencyWriter {
    fn run(&self, onto: SetOntology<ArcStr>) {

        let set_index = onto.i();
        let dependency_mechanism = match self.method.as_str() {
            "growth" => GrowthDependency::build_dependencies,
            "empty_sem" => SemanticEmptinessDependency::build_dependencies,
            "empty" => SyntacticEmptinessDependency::build_dependencies,
            "everything_sem" => SemanticEverythingDependency::build_dependencies,
            "everything" => SyntacticEverythingDependency::build_dependencies,
            "hop" => HopDependency::build_dependencies,
            _ => panic!("Unknown dependency mechanism {}", self.method),
        };

        let dependencies = dependency_mechanism(set_index.iter());
        let cleaned_dependencies = remove_super_symbols(&dependencies, set_index.iter());
        let mut annotations = Annotations::<_>::default();
        let placeholder = ArcStr::from("");
        annotations.visit_components(set_index.iter(), &placeholder);
        let mut results = HashMap::new();
        if self.llm {
            for (a, vs) in cleaned_dependencies.iter().progress() {
                let a_t = a.underlying();
                let english_labels = annotations.labels.iter().map(|(k,v)| (k.clone(),filter_literals_by_language(v.iter().collect(), &"en".to_string(), true))).collect();
                let english_defs = annotations.definitions.iter().map(|(k,v)| (k.clone(),filter_literals_by_language(v.iter().collect(), &"en".to_string(), true))).collect();
                match ask(a_t, vs, &english_defs, &english_labels) {
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

        let file = File::create(self.out_path.clone()).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &results).expect("Failed to write JSON to file");
    }
}

