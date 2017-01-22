#include <HDTManager.hpp>

extern "C" {
    void* map_indexed_hdt(char* file_path) {
        hdt::HDT* hdt = 0;
        try {
            hdt = hdt::HDTManager::mapIndexedHDT(file_path);
        } catch (...) {
        }
        return hdt;
    }
    void delete_hdt(void *hdt) {
        try {
            delete static_cast<hdt::HDT*>(hdt);
        } catch (...) {
        }
    }
    void* hdt_search_all(void *hdt) {
        hdt::IteratorTripleID* it = 0;
        try {
            it = static_cast<hdt::HDT*>(hdt)->getTriples()->searchAll();
        } catch (...) {
        }
        return it;
    }
    void* hdt_search_sp(void *hdt, const char* s, const char* p) {
        hdt::Dictionary * d = static_cast<hdt::HDT*>(hdt)->getDictionary();
        hdt::IteratorTripleID* it = 0;
        try {
            std::string subject_str(s);
            unsigned int subject = d->stringToId(subject_str, hdt::SUBJECT);
            std::string predicate_str(p);
            unsigned int predicate = d->stringToId(predicate_str, hdt::PREDICATE);
            hdt::TripleID pattern(subject, predicate, 0);
            it = static_cast<hdt::HDT*>(hdt)->getTriples()->search(pattern);
        } catch (...) {
        }
        return it;
    }
    void* hdt_search_op(void *hdt, const char* o, const char* p) {
        hdt::Dictionary * d = static_cast<hdt::HDT*>(hdt)->getDictionary();
        hdt::IteratorTripleID* it = 0;
        try {
            std::string object_str(o);
            unsigned int object = d->stringToId(object_str, hdt::OBJECT);
            std::string predicate_str(p);
            unsigned int predicate = d->stringToId(predicate_str, hdt::PREDICATE);
            hdt::TripleID pattern(0, predicate, object);
            it = static_cast<hdt::HDT*>(hdt)->getTriples()->search(pattern);
        } catch (...) {
        }
        return it;
    }
    void delete_iterator_triple_id(void *it) {
        delete static_cast<hdt::IteratorTripleID*>(it);
    }
    bool hdt_triple_id_has_next(void *it) {
        return static_cast<hdt::IteratorTripleID*>(it)->hasNext();
    }
    void* hdt_triple_id_next(void *it) {
        return static_cast<hdt::IteratorTripleID*>(it)->next();
    }
    uint64_t triple_id_subject(void* triple_id) {
        return static_cast<hdt::TripleID*>(triple_id)->getSubject();
    }
    uint64_t triple_id_predicate(void* triple_id) {
        return static_cast<hdt::TripleID*>(triple_id)->getPredicate();
    }
    uint64_t triple_id_object(void* triple_id) {
        return static_cast<hdt::TripleID*>(triple_id)->getObject();
    }
}
