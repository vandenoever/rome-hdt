extern crate libc;
extern crate rdfio;
extern crate rand;
use std::marker::PhantomData;

mod hdt;
mod get_resource_string;

#[derive (Clone)]
pub struct BlankNodePtr<'g> {
    str: String,
    graph_id: usize,
    phantom: PhantomData<&'g u8>,
}
impl<'g> PartialEq for BlankNodePtr<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.str == other.str && self.graph_id == other.graph_id
    }
}
impl<'g> Eq for BlankNodePtr<'g> {}
impl<'g> PartialOrd for BlankNodePtr<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g> Ord for BlankNodePtr<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut cmp = self.str.cmp(&other.str);
        if cmp == std::cmp::Ordering::Equal {
            cmp = self.graph_id.cmp(&other.graph_id)
        }
        cmp
    }
}
impl<'g> rdfio::graph::BlankNodePtr<'g> for BlankNodePtr<'g> {}
#[derive (Clone)]
pub struct IRIPtr<'g> {
    str: String,
    phantom: PhantomData<&'g u8>,
}
impl<'g> PartialEq for IRIPtr<'g> {
    fn eq(&self, other: &Self) -> bool {
        self.str.eq(&other.str)
    }
}
impl<'g> Eq for IRIPtr<'g> {}
impl<'g> PartialOrd for IRIPtr<'g> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g> Ord for IRIPtr<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.str.cmp(&other.str)
    }
}
impl<'g> rdfio::graph::IRIPtr<'g> for IRIPtr<'g> {
    fn as_str(&self) -> &str {
        &self.str
    }
}
#[derive (Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct LiteralPtr<'g> {
    str: String,
    datatype: String,
    language: Option<String>,
    phantom: PhantomData<&'g u8>,
}
impl<'g> rdfio::graph::LiteralPtr<'g> for LiteralPtr<'g> {
    fn as_str(&self) -> &str {
        &self.str
    }
    fn datatype(&self) -> &str {
        &self.datatype
    }
    fn language(&self) -> Option<&str> {
        if self.language.is_none() {
            None
        } else {
            Some(self.language.as_ref().unwrap())
        }
    }
}
#[derive (Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct Triple<'g> {
    subject: rdfio::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>>,
    predicate: IRIPtr<'g>,
    object: rdfio::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>>,
}
impl<'g> rdfio::graph::Triple<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> for Triple<'g> {
    fn subject(&self) -> rdfio::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>> {
        self.subject.clone()
    }
    fn predicate(&self) -> IRIPtr<'g> {
        self.predicate.clone()
    }
    fn object(&self) -> rdfio::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> {
        self.object.clone()
    }
}
pub struct Iter<'g> {
    it: hdt::IteratorTripleID<'g>,
    hdt: &'g HDT<'g>,
}
fn string_to_blank_node<'g>(string: String, graph_id: usize) -> BlankNodePtr<'g> {
    BlankNodePtr {
        str: string,
        graph_id: graph_id,
        phantom: PhantomData,
    }
}
fn string_to_iri<'g>(string: String) -> IRIPtr<'g> {
    IRIPtr {
        str: string,
        phantom: PhantomData,
    }
}
fn string_to_literal<'g>(mut string: String) -> LiteralPtr<'g> {
    if string.starts_with("\"") {
        if let Some(end) = string[1..].find("\"") {
            let datatype;
            let language;
            if end + 2 == string.len() {
                datatype = String::from("http://www.w3.org/2001/XMLSchema#string");
                language = None;
            } else if string.ends_with(">") {
                assert!(&string[end + 2..end + 5] == "^^<");
                datatype = String::from(&string[end + 5..string.len() - 1]);
                language = None;
            } else if &string[end + 2..end + 3] == "@" {
                datatype = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString");
                language = Some(String::from(&string[end + 3..]));
            } else {
                panic!(format!("Not a valid literal: '{}'", string));
            }
            string.truncate(end + 1);
            string.remove(0);
            return LiteralPtr {
                str: string,
                datatype: datatype,
                language: language,
                phantom: PhantomData,
            };
        }
    }
    panic!(format!("Not a valid literal: '{}'", string));
}
impl<'g> Iterator for Iter<'g> {
    type Item = Triple<'g>;
    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|triple| {
            let subject_str = triple.0;
            let object_str = triple.2;
            let subject;
            if subject_str.starts_with("_") {
                subject =
                    rdfio::graph::BlankNodeOrIRI::BlankNode(string_to_blank_node(subject_str,
                                                                                 self.hdt
                                                                                     .graph_id),
                                                            PhantomData);
            } else {
                subject = rdfio::graph::BlankNodeOrIRI::IRI(string_to_iri(subject_str));
            }
            let object;
            if object_str.starts_with("_") {
                object = rdfio::graph::Resource::BlankNode(string_to_blank_node(object_str,
                                                                                self.hdt
                                                                                    .graph_id),
                                                           PhantomData);
            } else if object_str.starts_with("\"") {
                object = rdfio::graph::Resource::Literal(string_to_literal(object_str));
            } else {
                object = rdfio::graph::Resource::IRI(string_to_iri(object_str));
            }
            Triple {
                subject: subject,
                predicate: string_to_iri(triple.1),
                object: object,
            }
        })
    }
}
impl<'g> rdfio::iter::SortedIterator for Iter<'g> {}
pub struct HDT<'g> {
    graph_id: usize,
    hdt: hdt::HDT,
    phantom: PhantomData<&'g u8>,
}

impl<'g> HDT<'g> {
    pub fn new(file_path: &str) -> rdfio::Result<HDT> {
        Ok(HDT {
            graph_id: rand::random::<usize>(),
            hdt: hdt::HDT::new(file_path)?,
            phantom: PhantomData,
        })
    }
}

impl<'g> rdfio::graph::Graph<'g> for HDT<'g> {
    type BlankNodePtr = BlankNodePtr<'g>;
    type IRIPtr = IRIPtr<'g>;
    type LiteralPtr = LiteralPtr<'g>;
    type SPOTriple = Triple<'g>;
    type SPOIter = Iter<'g>;
    type SPORangeIter = Iter<'g>;
    type OPSTriple = Triple<'g>;
    type OPSRangeIter = Iter<'g>;
    fn iter(&'g self) -> Self::SPOIter {
        Iter {
            it: self.hdt.search_all().unwrap(),
            hdt: self,
        }
    }
    fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr> {
        None
    }
    fn find_literal<'a>(&'g self,
                        literal: &'a str,
                        datatype: &'a str,
                        language: Option<&'a str>)
                        -> Option<Self::LiteralPtr> {
        None
    }

    fn iter_s_p(&'g self,
                subject: rdfio::graph::BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                predicate: Self::IRIPtr)
                -> Self::SPORangeIter {
        self.empty_spo_range()
    }
    fn iter_o_p(&'g self,
                object: rdfio::graph::Resource<'g,
                                               Self::BlankNodePtr,
                                               Self::IRIPtr,
                                               Self::LiteralPtr>,
                predicate: Self::IRIPtr)
                -> Self::OPSRangeIter {
        self.empty_ops_range()
    }

    /// iterator that returns no results
    fn empty_spo_range(&'g self) -> Self::SPORangeIter {
        Iter {
            it: self.hdt.search_all().unwrap(),
            hdt: self,
        }
    }
    /// iterator that returns no results
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter {
        Iter {
            it: self.hdt.search_all().unwrap(),
            hdt: self,
        }
    }
}

#[test]
fn load_file() {
    assert!(HDT::new("data/literals.hdt").is_ok());
}

#[test]
fn load_inexistant_file() {
    assert!(HDT::new("hello").is_err());
}

#[test]
fn iter() {
    use rdfio::graph::Graph;
    let hdt = HDT::new("data/literals.hdt").unwrap();
    for _ in hdt.iter() {
    }
}
