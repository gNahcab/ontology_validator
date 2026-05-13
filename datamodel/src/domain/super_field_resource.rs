use crate::error::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub enum SuperFieldResource {
    Resource,
    MovingImageRepresentation,
    StillImageRepresentation,
    AudioRepresentation,
    TextRepresentation,
    // ExternalOntology(Prefix, Element)
    ExternalOntology(String, String)
}


pub struct SuperFieldResWrapper(pub String);

impl SuperFieldResWrapper {
    pub(crate) fn to_super_field_res(&self) -> Result<SuperFieldResource, DataModelError> {
        match self.0.as_str() {
            "Resource" => {
                Ok(SuperFieldResource::Resource)
            }
            "MovingImageRepresentation" => {
                Ok(SuperFieldResource::MovingImageRepresentation)
            }
            "StillImageRepresentation" => {
                Ok(SuperFieldResource::StillImageRepresentation)
            }
            "AudioRepresentation" => {
                Ok(SuperFieldResource::AudioRepresentation)
            }
            "TextRepresentation" => {
                Ok(SuperFieldResource::TextRepresentation)
            }
            _ => {
                return match_external_ontology(self.0.as_str())
            }
        }
    }
}

fn match_external_ontology(candidate: &str) -> Result<SuperFieldResource, DataModelError> {
    let parts = candidate.split(":");
    let parts: Vec<_> = parts.into_iter().map(|value|value.to_owned()).collect();
    if parts.len() != 2 {
        return Err(DataModelError::ParsingError(format!("Unknown 'super'-value '{}', cannot match  with existing super-field.", candidate)));
    }
    let prefix: String = parts.get(0).unwrap().to_string();
    let element: String = parts.get(1).unwrap().to_string();
    return Ok(SuperFieldResource::ExternalOntology(prefix, element))




}