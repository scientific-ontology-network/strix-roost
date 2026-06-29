use crate::ontology::processors::annotations::Annotations;
use anyhow::{Error as E, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use horned_owl::model::ForIRI;
use tokenizers::Tokenizer;

#[allow(dead_code)]
pub struct DependencyCheckModel {
    use_pth: bool,
    approximate_gelu: bool,
    device: Device,
    tokenizer: Tokenizer,
    model: BertModel,
}

#[allow(dead_code)]
fn format_dependency<'a, T: ForIRI>(iri: &T, anno: &'a Annotations<'a, T>) -> String {
    format!(
        "{:?}; Definition: {:?}",
        anno.labels[iri], anno.definitions[iri]
    )
}

impl DependencyCheckModel {

    #[allow(dead_code)]
    pub(crate) fn new(device: Device) -> Self {
        let tokenizer =
            Tokenizer::from_file("model_data/sentence-transformers/minilm/tokenizer.json")
                .map_err(E::msg)
                .unwrap();
        let config: BertConfig = serde_json::from_str(
            &std::fs::read_to_string("model_data/sentence-transformers/minilm/tokenizer.json")
                .unwrap(),
        )
        .unwrap();
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&["minilm/model.safetensors"], DType::F32, &device)
                .unwrap()
        };

        let model = BertModel::load(vb, &config).unwrap();
        DependencyCheckModel {
            use_pth: false,
            approximate_gelu: false,
            device,
            tokenizer,
            model,
        }
    }

    pub(crate) fn run<'a, T: ForIRI>(
        &self,
        left: &T,
        rights: &Vec<&T>,
        anno: &'a Annotations<'a, T>,
    ) -> Result<Tensor> {
        let do_tokenize = |i| {
            let r = self.tokenizer.encode(i, true).map_err(E::msg);
            match r {
                Ok(r) => Ok(r.get_ids().to_vec()),
                Err(e) => Err(e),
            }
        };
        let left_tokens = do_tokenize(format_dependency(left, anno))?;
        let right_tokens = rights
            .iter()
            .map(|&iri| do_tokenize(format_dependency(iri, anno)).unwrap())
            .collect::<Vec<_>>();

        let left_token_ids = Tensor::new(left_tokens, &self.device)?.unsqueeze(0)?;
        let left_token_type_ids = left_token_ids.zeros_like()?;
        let left_embeddings = self
            .model
            .forward(&left_token_ids, &left_token_type_ids, None)?;

        let right_token_ids = Tensor::new(right_tokens, &self.device)?.unsqueeze(0)?;
        let right_token_type_ids = right_token_ids.zeros_like()?;
        let right_embeddings = self
            .model
            .forward(&right_token_ids, &right_token_type_ids, None)?;

        cosine_similarity(&left_embeddings, &right_embeddings)
    }
}

#[allow(dead_code)]
fn cosine_similarity(a: &Tensor, b: &Tensor) -> Result<Tensor> {
    let dot = (a * b)?.sum_all()?;
    let norm_a = (a * a)?.sum_all()?.sqrt()?;
    let norm_b = (b * b)?.sum_all()?.sqrt()?;
    let sim = (dot / (norm_a * norm_b))?;
    Ok(sim)
}
