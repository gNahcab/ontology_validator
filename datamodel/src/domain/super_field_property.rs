use crate::error::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub enum  SuperFieldProperty {
    HasValue,
    // ExternalOntology(Prefix, Element)
    ExternalOntology(String, String)

}
pub struct SuperFieldPropWrapper(pub String);

impl SuperFieldPropWrapper {
    pub(crate) fn to_super_field_prop(&self) -> Result<SuperFieldProperty, DataModelError> {
        match self.0.as_str() {
            "hasValue" => {
                Ok(SuperFieldProperty::HasValue)
            }
            _ => {
                match_external_super_prop(self.0.as_str())
            }
        }
    }
}

fn match_external_super_prop(candidate: &str) -> Result<SuperFieldProperty, DataModelError> {
    let parts = candidate.split(":");
    let parts: Vec<_> = parts.into_iter().map(|value|value.to_owned()).collect();
    if parts.len() != 2 {
        return Err(DataModelError::ParsingError(format!("Unknown 'super'-value '{}', cannot match with existing super-field-property.", candidate)));
    }
    let prefix: String = parts.get(0).unwrap().to_string();
    let element: String = parts.get(1).unwrap().to_string();
    Ok(SuperFieldProperty::ExternalOntology(prefix, element))
}
