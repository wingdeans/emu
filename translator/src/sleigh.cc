#include "globalcontext.hh"
#include "loadimage.hh"
#include "sleigh.hh"

struct Sleigh {
    ghidra::ContextInternal c_db;
    ghidra::RawLoadImage loader;
    ghidra::Sleigh sleigh;

    Sleigh(const std::string &s) : c_db(), loader(s), sleigh(&loader, &c_db) {
    }
};

extern "C" {
    Sleigh *sleigh_new() {
        return new Sleigh("test");
    }
    void sleigh_free(Sleigh *sleigh) {
        delete sleigh;
    }
}
