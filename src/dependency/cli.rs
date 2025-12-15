use std::collections::{HashMap, HashSet};
use clap::{Error, Parser};
use horned_owl::model::{ArcStr, Component};
use itertools::Itertools;
use serde::Serialize;
use crate::ontology::processors::annotations::Annotations;
use crate::dependency::base::{reduce_map, DependencyBuilder};
use crate::dependency::growth::{remove_super_symbols, GrowthDependency};
use crate::ontology::io::load_set_ontology;
use serde_json::json;
use crate::ontology::visitor::AxiomVisitor;

use crate::dependency::llm::ask;
use crate::dependency::symbol::{DependencySymbolWithAxioms, OntologySymbol, SymbolContainer};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Print {
    #[arg(short, long)]
    path: std::path::PathBuf,

    #[arg(short, long, default_value = "json")]
    format: String,

    #[arg(short, long)]
    llm: Option<bool>,
}

pub trait Runnable<T> : Parser{

    fn run(&self)  -> T;
    fn try_run() -> Result<T, Error> {
        {
            let cli = Self::try_parse();
            match cli {
                Ok(o) => Ok(o.run()),
                Err(e) => Err(e),
            }
        }
    }

}

impl Runnable<()> for Print {
    fn run(&self) {
        let onto = load_set_ontology(self.path.to_str().unwrap());
        let set_index = onto.i();
        let raw_dependencies = &GrowthDependency::build_dependencies(set_index.iter());
        let dependencies = reduce_map::<OntologySymbol<ArcStr>, DependencySymbolWithAxioms<OntologySymbol<ArcStr>, &Component<ArcStr>>>(raw_dependencies);
        let cleaned_dependencies = remove_super_symbols(&dependencies, set_index.iter(), |v|v.clone());
        let serialized_dependencies: HashMap<String, HashSet<String>> = cleaned_dependencies.iter().map(| (k,vs)| (k.get_iri().unwrap().to_string(), vs.iter().map(|v| v.get_symbol().get_iri().unwrap().to_string()).collect())).collect();
        let mut annotations = Annotations::<_>::default();
        let placeholder = ArcStr::from("");
        annotations.visit_components(set_index.iter(), &placeholder);
        for (a, vs) in cleaned_dependencies.iter(){
            let a_t = a.get_iri().unwrap();
            match ask(&a_t, &vs.iter().cloned().collect(), &annotations.definitions, &annotations.labels) {
                Ok(result) => {
                    if !result.is_empty() {
                        println!("## [{}]({})", &annotations.labels.get(&a_t).unwrap_or(&a_t.to_string()), a_t.to_string());
                        println!("| label | llm evaluation |");
                        println!("| --- | --- | --- |");
                        for (k, should_be_dependent) in result {
                            let mut dep_str = should_be_dependent.to_string();
                            let k_iri = k.get_symbol().get_iri().unwrap();
                            let k_ax = k.get_axioms();
                            let ax_string = k_ax.iter().map(|ax| format!("{:?}",**ax)).join(", ");
                            if !should_be_dependent {
                                dep_str = format!("**{dep_str}**");
                            }
                            println!("| [{}]({}) | {} | {} |", &annotations.labels.get(&k_iri).unwrap_or(&"-".to_string()), k_iri.to_string(), dep_str, ax_string);
                        }
                        println!("");
                    }
                }
                Err(e) => {panic!("Error querying LLM: {}", e)}
            }
        }
        if self.format == "json" {
            Self::print_json(serialized_dependencies);
        }
    }


}

impl Print {
    fn print_json<T: Serialize>(dep: HashMap<T, HashSet<T>>) {
        let jsn = json!(dep);
        let mut buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        jsn.serialize(&mut ser).unwrap();
        println!("{}", String::from_utf8(buf).unwrap());
    }
}