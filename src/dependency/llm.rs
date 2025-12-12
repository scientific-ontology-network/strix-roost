use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::time::Duration;
use horned_owl::model::ForIRI;
use horned_owl::ontology::indexed::ForIndex;
use serde::{Deserialize, Serialize};
use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde_json::json;
use crate::dependency::symbol::{ForSymbol, OntologySymbol, SymbolContainer};

pub(crate) fn ask<'a, C, SC:SymbolContainer<OntologySymbol<'a, T>, C>, T: ForIRI + 'a>(a: &T, depends_on: &Vec<SC>, definitions: &HashMap<T, String>, labels: &HashMap<T, String> ) -> Result<HashMap<SC, bool>, Error> {
    let request_url = "http://localhost:11434/api/generate";

    let client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(3000)).build()?;
    let label_a = labels.get(&a).unwrap_or(&a.to_string()).clone();
    let def_a = definitions.get(&a).unwrap_or(&a.to_string()).clone();
    let mut prompt = format!["Here is an ontology class and its definition:\n \
                '{label_a}'\n\
                Definition: '{def_a}'\n\
                \
                For each of the following concepts or relations tell me with for concept number i with a simple simple `i:true` or `i:false`, whether the previous class (namely '{label_a}') should depend on it, in that if you change the definition of the listed concept or property, it may affect the possible interpretations of the former (namely '{label_a}'). Note that 'false' answers should be rare, so only mark them in obvious cases. Each answer MUST be on a new line. DO NOT output anything else.\n"];
    if depends_on.is_empty() {
        Ok(HashMap::new())
    } else {
        for (i, dep) in depends_on.iter().enumerate() {
            let d = dep.get_symbol().get_iri().unwrap();
            let l = labels.get(&d).unwrap_or(&d.to_string()).clone();
            let def = definitions.get(&d).unwrap_or(&"No definition".to_string()).clone();
            prompt += format!["\t{i}: '{l}' -- Definition: '{def}'\n"].as_str();
        }
        let query_data = json![{
                    "model": "qwen3:30b-instruct", //any models pulled from Ollama can be replaced here
                    "prompt":  prompt,
                    "stream": false,
           }];
        let body_text = query_data.to_string();
        let response = client.post(request_url)
            .body(body_text)
            .send();
        match response {
            Ok(r) => {
                let response_text = r.text().unwrap();
                let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();
                let answers = response_json.get("response").unwrap_or(&serde_json::Value::Null).to_string();
                let lines: Vec<_> = answers.replace("\"", "").split("\\n").map(|x| {
                    let y: Vec<_> = x.split(":").collect();
                    match y[1].trim() {
                        "true" => true,
                        "false" => false,
                        _ => panic!("Could not process response {answers}. {x} is neither 'true' nor 'false'!"),
                    }

                }).collect();
                assert_eq!(lines.len(), depends_on.len());
                Ok(depends_on.iter().cloned().zip(lines).collect())
            },
            Err(e) => {Err(e)}
        }
    }
}