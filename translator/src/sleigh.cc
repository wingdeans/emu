#include "loadimage.hh"
#include "sleigh.hh"

#include <iostream> // TODO
#include <cassert>

struct BufLoadImage : public ghidra::LoadImage {
    uint8_t *buf;
    size_t len;

    BufLoadImage() : LoadImage("") {}
    virtual void loadFill(ghidra::uint1* outbuf,
                          ghidra::int4 readlen,
                          const ghidra::Address& addr) {
        assert(buf);
        assert(readlen == 16);
        memcpy(outbuf, buf, len);
    }
    virtual std::string getArchType() const { return ""; }
    virtual void adjustVma(long adjust) {}
};

struct AssemblyRaw : public ghidra::AssemblyEmit {
    virtual void dump(const ghidra::Address &addr,
                      const std::string &mnem,
                      const std::string &body) {
        std::cout << mnem << "\t" << body << std::endl;
    }
};

struct PcodeRaw : public ghidra::PcodeEmit {
    virtual void dump(const ghidra::Address &addr,
                      ghidra::OpCode opc,
                      ghidra::VarnodeData *outvar,
                      ghidra::VarnodeData *vars,
                      ghidra::int4 isize) {
    }
};

struct Sleigh {
    ghidra::ContextInternal c_db;
    BufLoadImage loader;
    ghidra::Sleigh sleigh;
    AssemblyRaw asmRaw;
    PcodeRaw pcodeRaw;

    Sleigh(const std::string &s) : c_db(), loader(), sleigh(&loader, &c_db) {
        ghidra::DocumentStorage docStorage;

        std::stringstream doc;
        doc << "<sleigh>" << s << "</sleigh>";
        ghidra::Element *root = docStorage.parseDocument(doc)->getRoot();
        docStorage.registerTag(root);

        sleigh.initialize(docStorage);
    }
};

extern "C" {
    Sleigh *sleigh_new(char *str, size_t len) {
        return new Sleigh(std::string(str, len));
    }
    void sleigh_free(Sleigh *sleigh) {
        delete sleigh;
    }
    void sleigh_disassemble(Sleigh *s, uint8_t *buf, size_t len, size_t addr) {
        ghidra::Address address(s->sleigh.getDefaultCodeSpace(), addr);
        s->loader.len = len;
        s->loader.buf = buf;
        s->sleigh.printAssembly(s->asmRaw, address);
        s->loader.buf = nullptr;
    }
    void sleigh_pcode(Sleigh *s, uint8_t *buf, size_t len, size_t addr) {
        ghidra::Address address(s->sleigh.getDefaultCodeSpace(), addr);
        s->loader.len = len;
        s->loader.buf = buf;
        s->sleigh.oneInstruction(s->pcodeRaw, address);
        s->loader.buf = nullptr;
    }
}
