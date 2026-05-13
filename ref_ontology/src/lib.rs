use std::collections::HashMap;
use crate::error::RefOntologyError;
use crate::ref_ontology::RefOntology;

mod ref_ontology;
mod error;

pub fn prefix_to_ref_ontology(prefix_to_graph: HashMap<String, String>) -> Result<HashMap<String, RefOntology>, RefOntologyError> {
    todo!()
}
