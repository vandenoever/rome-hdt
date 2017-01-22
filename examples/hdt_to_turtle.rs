extern crate rome;
extern crate rome_hdt;
use std::env::args;

use rome::namespaces::Namespaces;

fn main() {
    use rome::graph::Graph;
    let mut args = args();
    args.next();
    let hdt = args.next().unwrap();
    let hdt = rome_hdt::HDT::new(&hdt).unwrap();
    let mut namespaces = Namespaces::new();
    namespaces.set(b"dc", "http://purl.org/dc/terms/");
    namespaces.set(b"foaf", "http://xmlns.com/foaf/0.1/");
    namespaces.set(b"geo", "http://www.w3.org/2003/01/geo/");
    namespaces.set(b"l", "http://www.monnet-project.eu/lemon#");
    namespaces.set(b"o", "http://dbpedia.org/ontology/");
    namespaces.set(b"owl", "http://www.w3.org/2002/07/owl#");
    namespaces.set(b"prov", "http://www.w3.org/ns/prov#");
    namespaces.set(b"r", "http://dbpedia.org/resource/");
    namespaces.set(b"rdfs", "http://www.w3.org/2000/01/rdf-schema#");
    namespaces.set(b"s", "http://schema.org/");
    namespaces.set(b"skos", "http://www.w3.org/2004/02/skos/core#");
    namespaces.set(b"wkr", "http://wiktionary.dbpedia.org/resource/");
    namespaces.set(b"wkt", "http://wiktionary.dbpedia.org/terms/");
    namespaces.set(b"xs", "http://www.w3.org/2001/XMLSchema#");
    rome::io::write_turtle(&namespaces, hdt.iter(), &hdt, &mut ::std::io::stdout())
        .expect("failed to write turtle");
}
