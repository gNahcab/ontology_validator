use std::fs::File;
use oxrdfxml::RdfXmlParser;
use oxrdf::Triple;
use std::io::{BufReader, Cursor, Read};
use crate::error::ReadError;

pub(crate) fn read_rdf_xml_to_triples(path: &String) -> Result<Vec<Triple>, ReadError>{
    println!("Reading graph from {}", path);
    let file = File::open(path)?;
    read_triples(file)
}

fn read_triples(file: File) -> Result<Vec<Triple>, ReadError> {
    let mut triples = Vec::new();
    for triple in RdfXmlParser::new().for_reader(file) {
        triples.push(triple?);
    }
    Ok(triples)
}