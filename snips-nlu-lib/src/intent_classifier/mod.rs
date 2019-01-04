mod featurizer;
mod log_reg_intent_classifier;
mod logreg;

use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use crate::errors::*;
use failure::ResultExt;
use snips_nlu_ontology::IntentClassifierResult;

pub use self::featurizer::{CooccurrenceVectorizer, Featurizer, TfidfVectorizer};
pub use self::log_reg_intent_classifier::LogRegIntentClassifier;
use crate::models::ProcessingUnitMetadata;
use crate::resources::SharedResources;

pub trait IntentClassifier: Send + Sync {
    fn get_intent(
        &self,
        input: &str,
        intents_filter: Option<&[&str]>,
    ) -> Result<IntentClassifierResult>;
}

pub fn build_intent_classifier<P: AsRef<Path>>(
    path: P,
    shared_resources: Arc<SharedResources>,
) -> Result<Box<IntentClassifier>> {
    let metadata_path = path.as_ref().join("metadata.json");
    let metadata_file = File::open(&metadata_path).with_context(|_| {
        format!(
            "Cannot open intent classifier metadata file '{:?}'",
            &metadata_path
        )
    })?;
    let metadata: ProcessingUnitMetadata = serde_json::from_reader(metadata_file)
        .with_context(|_| "Cannot deserialize intent classifier json data")?;
    match metadata {
        ProcessingUnitMetadata::LogRegIntentClassifier => {
            Ok(Box::new(LogRegIntentClassifier::from_path(path, shared_resources)?) as _)
        }
        _ => Err(format_err!("{:?} is not an intent classifier", metadata)),
    }
}
