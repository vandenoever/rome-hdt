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
