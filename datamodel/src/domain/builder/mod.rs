use crate::domain::data_model::DataModel;
use crate::domain::dasch_list::DaSCHList;
use crate::domain::ontology::Ontology;
use crate::domain::property::Property;
use crate::domain::resource::DMResource;
use crate::error::DataModelError;

pub mod data_model_builder;

pub trait Builder {
    type OutputType;
    fn new(/* ... */) -> Self;
    fn add_to_ontology(&mut self, ontology: Ontology);
    fn add_to_properties(&mut self, properties: Vec<Property>);
    fn add_to_resources(&mut self, resources: Vec<DMResource>);

    fn add_list(&mut self, name: String, list:DaSCHList);
    fn is_complete(&self) -> Result<(), DataModelError>;

    fn build(self) -> DataModel;
    fn add_shortcode(&mut self, shortcode: String);
    fn add_shortname(&mut self, shortname: String);
}
