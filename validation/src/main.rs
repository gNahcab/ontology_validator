use std::env;
use datamodel::domain::data_model::DataModel;
use datamodel::domain::property::Property;
use datamodel::domain::resource::DMResource;
use datamodel::domain::super_field_property::SuperFieldProperty;
use datamodel::domain::super_field_resource::SuperFieldResource;
use reader::prefix_to_graph;
use ref_ontology::prefix_to_ref_ontology;
use validation::validate::validate;

fn main() {
    let args: Vec<_> = env::args().collect();
    let dm_path: &String = &args[1];
    let json_file = match reader::read_json(&dm_path){
        Ok(json_file) => {json_file}
        Err(err) => {
            panic!("Err: {}", err);
        }
    };
    let datamodel: DataModel = json_file.try_into().expect("REASON");

    let prefixes = fetch_prefixes(&datamodel.properties, &datamodel.resources);
    let prefix_to_graph = prefix_to_graph(prefixes).expect("REASON");
    println!("prefix_to_graph: {:?}", prefix_to_graph);
    //let prefix_to_ref_ontology = prefix_to_ref_ontology(prefix_to_graph).expect("REASON");
    //println!("prefix_to_ref_ontology: {:?}", prefix_to_ref_ontology);
    //validate(prefix_to_ref_ontology, &datamodel);


}
fn fetch_prefixes(properties: &Vec<Property>, resources: &Vec<DMResource>) -> Vec<String> {
    let mut prefixes = vec![];
    properties.iter().for_each(|property| {
        property.super_fields.iter().for_each(|super_field| {
            match super_field {
                SuperFieldProperty::ExternalOntology(prefix, _) => {
                    prefixes.push(prefix.clone());
                }
                _ => {
                    // continue
                }
            }
        })
    });
    resources.iter().for_each(|resource| {
        resource.super_fields.iter().for_each(|super_field| {
            match super_field {
                SuperFieldResource::ExternalOntology(prefix, _) => {
                    prefixes.push(prefix.clone());
                }
                _ => {
                    // continue
                }
            }
        })
    });
    prefixes
}
