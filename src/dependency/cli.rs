pub use crate::cli::base::Runnable;
use crate::dependency::base::{remove_super_symbols, DependencyBuilder};
use crate::dependency::empty::{SyntacticEmptinessDependency};
use crate::dependency::everything::{SyntacticEverythingDependency};
use crate::dependency::growth::{GrowthDependency};
use clap::{Args};
use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::model::ArcStr;
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
            "empty" => SyntacticEmptinessDependency::build_dependencies,
            "everything" => SyntacticEverythingDependency::build_dependencies,
            "hop" => HopDependency::build_dependencies,
            _ => panic!("Unknown dependency mechanism {}", self.method),
        };
        let dependencies = dependency_mechanism(set_index.iter());

        let cleaned_dependencies = remove_super_symbols(&dependencies, set_index.iter());

        let mut results = HashMap::new();

        for (a, vs) in cleaned_dependencies.iter() {
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

        let file = File::create(self.out_path.clone()).expect("Failed to create file");
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &results).expect("Failed to write JSON to file");
    }
}

