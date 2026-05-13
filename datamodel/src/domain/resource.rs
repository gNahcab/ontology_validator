use serde_json::Value;
use crate::domain::label::{Label, LabelWrapper};
use crate::domain::res_property::{ResProperty, ResPropertyWrapper};
use crate::domain::super_field_resource::{SuperFieldResource, SuperFieldResWrapper};
use crate::error::DataModelError;

#[derive(Debug, PartialEq, Clone)]
pub struct DMResource {
    pub name: String,
    labels: Vec<Label>,
    pub super_fields: Vec<SuperFieldResource>,
    pub properties: Vec<ResProperty>,
}

impl DMResource {
    fn new(transient_resource: TransientResource) -> Self {
        DMResource{
            name: transient_resource.name.unwrap(),
            labels: transient_resource.labels,
            super_fields: transient_resource.super_fields,
            properties: transient_resource.res_props,
        }
    }
}

struct TransientResource {
    name: Option<String>,
    labels: Vec<Label>,
    super_fields: Vec<SuperFieldResource>,
    res_props: Vec<ResProperty>
}


impl TransientResource {
    fn new() -> Self {
        TransientResource{
            name: None,
            labels: vec![],
            super_fields: vec![],
            res_props: vec![],
        }
    }

    fn add_name(&mut self, name: String) {
        self.name = Some(name);
    }
    fn add_label(&mut self, label: Label) {
        self.labels.push(label);
    }
    fn add_super(&mut self, super_field: SuperFieldResource) {
        self.super_fields.push(super_field);
    }
    fn add_res_prop(&mut self, res_prop: ResProperty) {
        self.res_props.push(res_prop);
    }
    fn is_complete(&self) -> Result<(), DataModelError> {
        // resource is complete if name, label, super, at least one res_prop exist
        if self.name.is_none() {
            return Err(DataModelError::ParsingError(format!("name is required for resource with labels: '{:?}'", self.labels)));
        }
        if self.labels.is_empty() {
            return Err(DataModelError::ParsingError(format!("at least one label is required for resource with name: '{:?}'", self.name)));
        }
        if self.super_fields.is_empty() {
            return Err(DataModelError::ParsingError(format!("one super is required for resource with name: '{:?}'", self.name)));
        }
        if self.res_props.is_empty() {
            return Err(DataModelError::ParsingError(format!("at least one res_prop is required for resource with name: '{:?}'", self.name)));
        }
        super_fields_not_only_external_ontology(&self.super_fields, &self.name)?;
        Ok(())
    }
}

fn super_fields_not_only_external_ontology(super_fields: &Vec<SuperFieldResource>, res_name: &Option<String>) -> Result<(), DataModelError> {
    for super_field in super_fields {
        match super_field {
            SuperFieldResource::ExternalOntology(_, _) => {}
            _ => {
                // not an external ontology
                return Ok(());
            }
        }
    }
    return Err(DataModelError::ParsingError(format!("Superfield of resource '{:?}' contains only external-ontologies: '{:?}'", res_name, super_fields)));
}

pub fn match_supers(value: &Value, res_or_prop_name: &Option<String>, res_or_prop: String) -> Result<Vec<String>, DataModelError>{
    let mut supers = vec![];
    match value {
        Value::String(super_) => {
            supers.push(super_.to_owned());
            Ok(supers)
        }
        Value::Array(super_) => {
            for value in super_ {
                match value {
                    Value::String(super_) => {
                        supers.push(super_.to_owned());
                    }
                    _ => {
                        return Err(DataModelError::ParsingError(format!("super-array '{:?}' of {} with name '{:?}' is not a String.", value, res_or_prop, res_or_prop_name)));
                    }
                }
            }
            Ok(supers)
        }
        _ => {
            Err(DataModelError::ParsingError(format!("super '{:?}' of resource with name '{:?}' is not a String.", value, res_or_prop_name)))
        }
    }
}
pub(crate) struct ResourceWrapper(pub(crate) Value);
impl ResourceWrapper{
    pub fn to_resource(&self) -> Result<DMResource, DataModelError> {
        let resource_raw = self.0.as_object().expect("resource should be an object");
        let mut transient_resource = TransientResource::new();
        for (key, value) in resource_raw.iter(){
            match key.as_str() {
                "name" => {
                    let name = match value {
                        Value::String(name) => {name}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("name '{:?}' of resource is not a String.", value)));
                        }
                    };
                    transient_resource.add_name(name.to_owned());
                }
                "labels" => {
                    let labels_raw = match value {
                        Value::Object(labels) => {labels}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("labels '{:?}' of resource with name '{:?}' is not an Object.", value, transient_resource.name)));
                        }
                    };
                    for (key, value) in labels_raw.iter() {
                        let label = LabelWrapper((key.to_owned(), value.to_owned())).to_label()?;
                        transient_resource.add_label(label);
                    }
                }
                "super" => {
                    let supers = match_supers(value, &transient_resource.name, "resource".to_string())?;
                    for super_ in supers {
                        let super_field = SuperFieldResWrapper(super_.to_owned()).to_super_field_res()?;
                        transient_resource.add_super(super_field);
                    }
                }
                "cardinalities" => {
                    let res_props_raw = match value {
                        Value::Array(res_props_raw) => {res_props_raw}
                        _ => {
                            return Err(DataModelError::ParsingError(format!("cardinalities '{:?}' of resource with name '{:?}' is not a String.", value, transient_resource.name)));
                        }
                    };
                    for res_prop_raw in res_props_raw.iter() {
                        let res_prop = ResPropertyWrapper(res_prop_raw.to_owned()).to_res_prop()?;
                        transient_resource.add_res_prop(res_prop);
                    }
                }
                _ => {

                }
            }
        }
        transient_resource.is_complete()?;
        Ok(DMResource::new(transient_resource))

    }
}
