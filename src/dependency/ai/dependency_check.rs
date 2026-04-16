use crate::dependency::ai::model::DependencyCheckModel;
use crate::dependency::symbol::Symbol;
use crate::ontology::processors::annotations::Annotations;
use horned_owl::model::ForIRI;
use horned_owl::ontology::indexed::ForIndex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

pub fn check_dependencies<'a, T: ForIRI>(
    dependencies: &mut HashMap<&T, HashMap<&T, Value>>,
    anno: &'a Annotations<'a, T>,
) {
    let dep_checker = DependencyCheckModel::new(candle_core::Device::Cpu);
    let ls = dependencies.keys().collect::<HashSet<_>>();
    for (l, rs) in dependencies.iter_mut() {
        let ordered_rs = rs.keys().cloned().collect::<Vec<_>>();
        let answers = dep_checker.run(*l, &ordered_rs, anno);
        for (r, b) in ordered_rs.iter().zip(answers.iter()) {
            let d = rs.get_mut(r).unwrap();
            d["llm-result"] = json!(b.to_scalar::<f64>().unwrap() > 0.5);
        }
    }
}
