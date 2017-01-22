extern crate rdfio;
extern crate rdfio_hdt;
use std::env::args;

use rdfio::namespaces::Namespaces;

fn main() {
    use rdfio::graph::{Graph, Triple, LiteralPtr};
    let mut args = args();
    let exe = args.next().unwrap();
    let hdt = args.next().unwrap();
    let hdt = rdfio_hdt::HDT::new(&hdt).unwrap();
    let namespaces = Namespaces::new();
    rdfio::io::write_turtle(&namespaces, hdt.iter(), &mut ::std::io::stdout());
}
