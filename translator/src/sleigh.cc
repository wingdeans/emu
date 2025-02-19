#include "loadimage.hh"
#include "sleigh.hh"

#include <iostream> // TODO
#include <cassert>

struct String;

extern "C" String *rust_str(const char *buf, size_t len);

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
    String *mnem;
    String *body;
    virtual void dump(const ghidra::Address &addr,
                      const std::string &mnem,
                      const std::string &body) {
        this->mnem = rust_str(mnem.data(), mnem.size());
        this->body = rust_str(body.data(), body.size());
    }
};

// TODO
void print_vardata(std::ostream &s, ghidra::VarnodeData &data) {
    s << '(' << data.space->getName() << ',';
    data.space->printOffset(s,data.offset);
    s << ',' << std::dec << data.size << ')' << " ";
}

struct PcodeRaw : public ghidra::PcodeEmit {
    uint8_t opc;
    virtual void dump(const ghidra::Address &addr,
                      ghidra::OpCode opc,
                      ghidra::VarnodeData *outvar,
                      ghidra::VarnodeData *vars,
                      ghidra::int4 isize) {
        this->opc = opc;
        std::cout << ghidra::get_opname(opc) << " ";
        if (outvar != nullptr) {
            print_vardata(std::cout, *outvar);
        }
        for (int i = 0; i < isize; ++i) {
            print_vardata(std::cout, vars[i]);
        }
        std::cout << std::endl;
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
    void sleigh_disassemble(Sleigh *s, uint8_t *buf, size_t len, size_t addr,
                            String **out_mnem, String **out_body) {
        ghidra::Address address(s->sleigh.getDefaultCodeSpace(), addr);
        s->loader.len = len;
        s->loader.buf = buf;
        s->sleigh.printAssembly(s->asmRaw, address);
        s->loader.buf = nullptr;

        *out_mnem = s->asmRaw.mnem;
        *out_body = s->asmRaw.body;
        s->asmRaw.mnem = s->asmRaw.body = nullptr;
    }
    uint32_t sleigh_pcode(Sleigh *s, uint8_t *buf, size_t len, size_t addr) {
        ghidra::Address address(s->sleigh.getDefaultCodeSpace(), addr);
        s->loader.len = len;
        s->loader.buf = buf;
        uint32_t insnLen = s->sleigh.oneInstruction(s->pcodeRaw, address);
        s->loader.buf = nullptr;

        return insnLen;
    }
}
