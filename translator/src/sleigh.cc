#include "loadimage.hh"
#include "sleigh.hh"

#include <iostream>

struct Sla {
    ghidra::DocumentStorage docStorage;

    Sla(const std::string &s) {
        std::stringstream doc;
        doc << "<sleigh>" << s << "</sleigh>";
        ghidra::Element *root = docStorage.parseDocument(doc)->getRoot();
        docStorage.registerTag(root);
    }
};

struct Sleigh {
    ghidra::ContextInternal c_db;
    ghidra::RawLoadImage loader;
    ghidra::Sleigh sleigh;

    Sleigh(const std::string &s) : c_db(), loader(s), sleigh(&loader, &c_db) {
    }
};

extern "C" {
    Sla *sla_new(char *str, size_t len) {
        return new Sla(std::string(str, len));
    }
    void sla_free(Sla *sla) {
        delete sla;
    }

    Sleigh *sleigh_new(char *str, size_t len) {
        return new Sleigh(std::string(str, len));
    }
    void sleigh_free(Sleigh *sleigh) {
        delete sleigh;
    }
}
