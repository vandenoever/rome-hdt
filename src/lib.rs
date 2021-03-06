extern crate rome;
extern crate rand;
use std::marker::PhantomData;
use std::fmt;

mod hdt;
mod get_resource_string;

const XSD_STRING: &'static str = "http://www.w3.org/2001/XMLSchema#string";
const RDF_LANG_STRING: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";

#[derive (Clone,Debug)]
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
impl<'g> fmt::Display for BlankNodePtr<'g> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}
impl<'g> rome::graph::BlankNodePtr<'g> for BlankNodePtr<'g> {}
impl<'g> Into<rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>>> for BlankNodePtr<'g> {
    fn into(self) -> rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>> {
        rome::graph::BlankNodeOrIRI::BlankNode(self, PhantomData)
    }
}
impl<'g> Into<rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>>> for BlankNodePtr<'g> {
    fn into(self) -> rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> {
        rome::graph::Resource::BlankNode(self, PhantomData)
    }
}
#[derive (Clone,Debug)]
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
impl<'g> rome::graph::IRIPtr<'g> for IRIPtr<'g> {
    fn as_str(&self) -> &str {
        &self.str
    }
}
impl<'g> Into<rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>>> for IRIPtr<'g> {
    fn into(self) -> rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>> {
        rome::graph::BlankNodeOrIRI::IRI(self)
    }
}
impl<'g> Into<rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>>> for IRIPtr<'g> {
    fn into(self) -> rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> {
        rome::graph::Resource::IRI(self)
    }
}
#[derive (PartialEq)]
pub enum DatatypePtr {
    None,
    Datatype(String),
    Language,
}
impl<'g> rome::graph::DatatypePtr<'g> for DatatypePtr {
    fn as_str(&self) -> &str {
        match self {
            &DatatypePtr::None => XSD_STRING,
            &DatatypePtr::Datatype(ref str) => str,
            &DatatypePtr::Language => RDF_LANG_STRING,
        }
    }
}
#[derive (Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
enum LiteralType {
    None,
    Datatype,
    Language,
}
#[derive (Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
pub struct LiteralPtr<'g> {
    str: String,
    value_end: usize,
    literal_type: LiteralType,
    phantom: PhantomData<&'g u8>,
}
impl<'g> LiteralPtr<'g> {
    fn new(value: &str, datatype: &str, language: Option<&str>) -> LiteralPtr<'g> {
        // calculate needed capacity
        let mut len = value.len() + 2;
        let literal_type;
        if let Some(ref lang) = language {
            len += 1 + lang.len();
            literal_type = LiteralType::Language;
        } else if datatype == XSD_STRING {
            literal_type = LiteralType::None;
        } else {
            len += 4 + datatype.len();
            literal_type = LiteralType::Datatype;
        }
        let mut str = String::with_capacity(len);
        str.push('\"');
        str.push_str(value);
        str.push('"');
        if let Some(ref lang) = language {
            str.push_str("@");
            str.push_str(lang);
        } else if literal_type == LiteralType::Datatype {
            str.push_str("^^<");
            str.push_str(datatype);
            str.push('>');
        }
        LiteralPtr {
            str: str,
            value_end: value.len() + 1,
            literal_type: literal_type,
            phantom: PhantomData,
        }
    }
}
impl<'g> rome::graph::LiteralPtr<'g> for LiteralPtr<'g> {
    type DatatypePtr = DatatypePtr;
    fn as_str(&self) -> &str {
        &self.str[1..self.value_end]
    }
    fn datatype(&self) -> DatatypePtr {
        match self.literal_type {
            LiteralType::None => DatatypePtr::None,
            LiteralType::Datatype => {
                DatatypePtr::Datatype(String::from(&self.str[self.value_end + 4..self.str.len() -
                                                                                 1]))
            }
            LiteralType::Language => DatatypePtr::Language,
        }
    }
    fn datatype_str(&self) -> &str {
        match self.literal_type {
            LiteralType::None => XSD_STRING,
            LiteralType::Datatype => &self.str[self.value_end + 4..self.str.len() - 1],
            LiteralType::Language => RDF_LANG_STRING,
        }
    }
    fn language(&self) -> Option<&str> {
        match self.literal_type {
            LiteralType::None => None,
            LiteralType::Datatype => None,
            LiteralType::Language => Some(&self.str[self.value_end + 2..]),
        }
    }
}
impl<'g> Into<rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>>> for LiteralPtr<'g> {
    fn into(self) -> rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> {
        rome::graph::Resource::Literal(self)
    }
}
#[derive (Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
pub struct Triple<'g> {
    subject: rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>>,
    predicate: IRIPtr<'g>,
    object: rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>>,
}
impl<'g> rome::graph::Triple<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> for Triple<'g> {
    fn subject(&self) -> rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>> {
        self.subject.clone()
    }
    fn predicate(&self) -> IRIPtr<'g> {
        self.predicate.clone()
    }
    fn object(&self) -> rome::graph::Resource<'g, BlankNodePtr<'g>, IRIPtr<'g>, LiteralPtr<'g>> {
        self.object.clone()
    }
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
fn string_to_literal<'g>(string: String) -> LiteralPtr<'g> {
    if string.starts_with("\"") {
        // we use 'rfind' because a literal may contain escaped " characters
        // TODO: figure out how to work with escapes and HDT
        if let Some(end) = string[1..].rfind("\"") {
            let literal_type;
            if end + 2 == string.len() {
                literal_type = LiteralType::None;
            } else if string.ends_with(">") {
                literal_type = LiteralType::Datatype;
                assert!(&string[end + 2..end + 5] == "^^<");
            } else if &string[end + 2..end + 3] == "@" {
                literal_type = LiteralType::Language;
            } else {
                panic!(format!("Not a valid literal: '{}'", string));
            }
            return LiteralPtr {
                str: string,
                value_end: end + 1,
                literal_type: literal_type,
                phantom: PhantomData,
            };
        }
    }
    panic!(format!("Not a valid literal: '{}'", string));
}
pub struct Iter<'g> {
    it: Option<hdt::IteratorTripleID<'g>>,
    hdt: &'g HDT<'g>,
}
impl<'g> Iterator for Iter<'g> {
    type Item = Triple<'g>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.it.is_none() {
            return None;
        }
        let it = &self.it.as_ref().unwrap();
        it.next().map(|triple| {
            let subject_str = triple.0;
            let object_str = triple.2;
            let subject;
            if subject_str.starts_with("_") {
                subject =
                    rome::graph::BlankNodeOrIRI::BlankNode(string_to_blank_node(subject_str,
                                                                                 self.hdt
                                                                                     .graph_id),
                                                            PhantomData);
            } else {
                subject = rome::graph::BlankNodeOrIRI::IRI(string_to_iri(subject_str));
            }
            let object;
            if object_str.starts_with("_") {
                object = rome::graph::Resource::BlankNode(string_to_blank_node(object_str,
                                                                                self.hdt
                                                                                    .graph_id),
                                                           PhantomData);
            } else if object_str.starts_with("\"") {
                object = rome::graph::Resource::Literal(string_to_literal(object_str));
            } else {
                object = rome::graph::Resource::IRI(string_to_iri(object_str));
            }
            Triple {
                subject: subject,
                predicate: string_to_iri(triple.1),
                object: object,
            }
        })
    }
}

fn blank_node_or_iri_to_hdt_string<'g>(blank_node_or_iri: &'g rome::graph::BlankNodeOrIRI<'g, BlankNodePtr<'g>, IRIPtr<'g>>) -> &'g str {
    match blank_node_or_iri {
        &rome::graph::BlankNodeOrIRI::BlankNode(ref b, _) => &b.str,
        &rome::graph::BlankNodeOrIRI::IRI(ref i) => &i.str,
    }
}
fn resource_to_hdt_string<'g>(resource: &'g rome::graph::Resource<'g,
                                                                   BlankNodePtr<'g>,
                                                                   IRIPtr<'g>,
                                                                   LiteralPtr<'g>>)
                              -> &'g str {
    match resource {
        &rome::graph::Resource::BlankNode(ref b, _) => &b.str,
        &rome::graph::Resource::IRI(ref i) => &i.str,
        &rome::graph::Resource::Literal(ref l) => &l.str,
    }
}

impl<'g> rome::iter::SortedIterator for Iter<'g> {}
pub struct HDT<'g> {
    graph_id: usize,
    hdt: hdt::HDT,
    phantom: PhantomData<&'g u8>,
}

impl<'g> HDT<'g> {
    pub fn new(file_path: &str) -> rome::Result<HDT> {
        Ok(HDT {
            graph_id: rand::random::<usize>(),
            hdt: hdt::HDT::new(file_path)?,
            phantom: PhantomData,
        })
    }
}

impl<'g> rome::graph::Graph<'g> for HDT<'g> {
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
            it: Some(self.hdt.search_all().unwrap()),
            hdt: self,
        }
    }
    fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr> {
        Some(IRIPtr {
            str: String::from(iri),
            phantom: PhantomData,
        })
    }
    fn find_literal<'a>(&'g self,
                        literal: &'a str,
                        datatype: &'a str,
                        language: Option<&'a str>)
                        -> Option<Self::LiteralPtr> {
        Some(LiteralPtr::new(literal, datatype, language))
    }
    fn find_datatype<'a>(&'g self, datatype: &'a str) -> Option<DatatypePtr> {
        Some(if datatype == XSD_STRING {
            DatatypePtr::None
        } else if datatype == RDF_LANG_STRING {
            DatatypePtr::Language
        } else {
            DatatypePtr::Datatype(String::from(datatype))
        })
    }
    fn iter_s_p(&'g self,
                subject: rome::graph::BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                predicate: Self::IRIPtr)
                -> Self::SPORangeIter {
        let subject = blank_node_or_iri_to_hdt_string(&subject);
        Iter {
            it: self.hdt.search_sp(subject, &predicate.str),
            hdt: self,
        }
    }
    fn iter_o_p(&'g self,
                object: rome::graph::Resource<'g,
                                               Self::BlankNodePtr,
                                               Self::IRIPtr,
                                               Self::LiteralPtr>,
                predicate: Self::IRIPtr)
                -> Self::OPSRangeIter {
        let object = resource_to_hdt_string(&object);
        Iter {
            it: self.hdt.search_op(object, &predicate.str),
            hdt: self,
        }
    }
    /// iterator that returns no results
    fn empty_spo_range(&'g self) -> Self::SPORangeIter {
        Iter {
            it: None,
            hdt: self,
        }
    }
    /// iterator that returns no results
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter {
        Iter {
            it: None,
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
fn iter_literals() {
    use rome::graph::{Graph, Triple, LiteralPtr};
    let hdt = HDT::new("data/literals.hdt").unwrap();
    assert_eq!(hdt.iter().count(), 9);
    for t in hdt.iter() {
        let o = t.object();
        let l1 = o.as_literal().unwrap();
        let l2 = self::LiteralPtr::new(l1.as_str(), l1.datatype_str(), l1.language());
        assert_eq!(l1, &l2);
    }
}

#[test]
fn iter_spo() {
    use rome::graph::Graph;
    let hdt = HDT::new("data/literals.hdt").unwrap();
    let s = hdt.find_iri("s").unwrap();
    let p = hdt.find_iri("p").unwrap();
    assert_eq!(hdt.iter_s_p(s.into(), p).count(), 9);
}
#[test]
fn iter_ops() {
    use rome::graph::Graph;
    let hdt = HDT::new("data/literals.hdt").unwrap();
    let o = hdt.find_literal("abc", "bcd", None).unwrap();
    let p = hdt.find_iri("p").unwrap();
    assert_eq!(hdt.iter_o_p(o.into(), p).count(), 1);
}
