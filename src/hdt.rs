use rdfio;
use std::ffi::CString;
use std::os::raw::{c_void, c_char};

use get_resource_string::*;

extern "C" {
    fn map_indexed_hdt(file_path: *const u8) -> *mut c_void;
    fn delete_hdt(hdt: *mut c_void);

    fn hdt_search_all(hdt: *mut c_void) -> *mut c_void;
    fn hdt_search_sp(hdt: *mut c_void, s: *const c_char, p: *const c_char) -> *mut c_void;
    fn hdt_search_op(hdt: *mut c_void, o: *const c_char, p: *const c_char) -> *mut c_void;
    fn delete_iterator_triple_id(hdt: *mut c_void);
    fn hdt_triple_id_has_next(it: *mut c_void) -> bool;
    fn hdt_triple_id_next(it: *mut c_void) -> *mut c_void;
    fn triple_id_subject(triple: *mut c_void) -> u64;
    fn triple_id_predicate(triple: *mut c_void) -> u64;
    fn triple_id_object(triple: *mut c_void) -> u64;
}

pub struct HDT {
    hdt: *mut c_void,
}

impl HDT {
    pub fn new(file_path: &str) -> rdfio::Result<HDT> {
        let hdt;
        unsafe {
            hdt = map_indexed_hdt(file_path.as_ptr());
        }
        if hdt.is_null() {
            Err(rdfio::error::Error::Custom("could not open file"))
        } else {
            Ok(HDT { hdt: hdt })
        }
    }
    pub fn search_all(&self) -> rdfio::Result<IteratorTripleID> {
        let it;
        unsafe {
            it = hdt_search_all(self.hdt);
        }
        if it.is_null() {
            Err(rdfio::error::Error::Custom("could not create iterator"))
        } else {
            Ok(IteratorTripleID {
                it: it,
                hdt: self,
            })
        }
    }
    pub fn search_sp(&self, subject: &str, predicate: &str) -> Option<IteratorTripleID> {
        let it;
        let subject = CString::new(subject).unwrap();
        let predicate = CString::new(predicate).unwrap();
        unsafe {
            it = hdt_search_sp(self.hdt, subject.as_ptr(), predicate.as_ptr());
        }
        if it.is_null() {
            None
        } else {
            Some(IteratorTripleID {
                it: it,
                hdt: self,
            })
        }
    }
    pub fn search_op(&self, object: &str, predicate: &str) -> Option<IteratorTripleID> {
        let it;
        let object = CString::new(object).unwrap();
        let predicate = CString::new(predicate).unwrap();
        unsafe {
            it = hdt_search_op(self.hdt, object.as_ptr(), predicate.as_ptr());
        }
        if it.is_null() {
            None
        } else {
            Some(IteratorTripleID {
                it: it,
                hdt: self,
            })
        }
    }
}
impl<'g> Drop for HDT {
    fn drop(&mut self) {
        unsafe {
            delete_hdt(self.hdt);
        }
    }
}

pub struct IteratorTripleID<'g> {
    hdt: &'g HDT,
    it: *mut c_void,
}
impl<'g> IteratorTripleID<'g> {
    fn has_next(&self) -> bool {
        unsafe { hdt_triple_id_has_next(self.it) }
    }
    pub fn next(&self) -> Option<(String, String, String)> {
        if !self.has_next() {
            return None;
        }
        let triple;
        unsafe {
            let ptr = hdt_triple_id_next(self.it);
            triple = (get_resource_string(self.hdt.hdt,
                                          triple_id_subject(ptr),
                                          TripleComponentRole::Subject),
                      get_resource_string(self.hdt.hdt,
                                          triple_id_predicate(ptr),
                                          TripleComponentRole::Predicate),
                      get_resource_string(self.hdt.hdt,
                                          triple_id_object(ptr),
                                          TripleComponentRole::Object));
        }
        Some(triple)
    }
}
impl<'g> Drop for IteratorTripleID<'g> {
    fn drop(&mut self) {
        unsafe {
            delete_iterator_triple_id(self.it);
        }
    }
}
