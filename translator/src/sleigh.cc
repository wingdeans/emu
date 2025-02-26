#include "loadimage.hh"
#include "sleigh.hh"

#include <cassert>

struct String;
struct PcodeVec;

struct Varnode {
    uint8_t addr;
    uint32_t size;
    uint64_t offset;
};

extern "C" String *rust_str(const char *buf, size_t len);
extern "C" void rust_push_pcode(PcodeVec *vec,
                                uint8_t opc,
                                Varnode *vars,
                                uint32_t size);

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

struct PcodeRaw : public ghidra::PcodeEmit {
    PcodeVec *vec;

    uint8_t spaces[8] = {0xFF};

    virtual void dump(const ghidra::Address &addr,
                      ghidra::OpCode opc,
                      ghidra::VarnodeData *outvar,
                      ghidra::VarnodeData *vars,
                      ghidra::int4 isize) {
        Varnode varnodes[isize + 1];
        if (outvar != nullptr) {
            Varnode &vn = varnodes[0];
            ghidra::int4 idx = outvar->space->getIndex();
            assert(idx < sizeof(spaces));
            vn.addr = spaces[idx];
            vn.size = outvar->size;
            vn.offset = outvar->offset;
        } else {
            varnodes[0].addr = 0;
        }
        for (int i = 0; i < isize; i ++) {
            ghidra::VarnodeData &data = vars[i];
            Varnode &vn = varnodes[i + 1];

            ghidra::int4 idx;
            if (i == 0 && (opc == ghidra::CPUI_STORE || opc == ghidra::CPUI_LOAD)) {
                idx = reinterpret_cast<ghidra::AddrSpace*>(data.offset)->getIndex();
                vn.offset = 0;
            } else {
                idx = data.space->getIndex();
                vn.offset = data.offset;
            }
            assert(idx < sizeof(spaces));

            vn.addr = spaces[idx];
            vn.size = data.size;
        }
        rust_push_pcode(vec, opc, varnodes, isize + 1);
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

        pcodeRaw.spaces[sleigh.getDefaultDataSpace()->getIndex()] = 1; // ram
        pcodeRaw.spaces[sleigh.getSpaceByName("register")->getIndex()] = 2; // reg
        pcodeRaw.spaces[sleigh.getConstantSpace()->getIndex()] = 3; // const
        pcodeRaw.spaces[sleigh.getUniqueSpace()->getIndex()] = 4; // uniq
    }
};

extern "C" {
    Sleigh *sleigh_new(char *str, size_t len) {
        return new Sleigh(std::string(str, len));
    }
    void sleigh_free(Sleigh *sleigh) {
        delete sleigh;
    }
    void sleigh_disassemble(Sleigh *s, uint8_t *buf, size_t len, uint64_t addr,
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
    uint32_t sleigh_pcode(Sleigh *s, uint8_t *buf, size_t len, uint64_t addr, PcodeVec *vec) {
        ghidra::Address address(s->sleigh.getDefaultCodeSpace(), addr);
        s->loader.len = len;
        s->loader.buf = buf;
        s->pcodeRaw.vec = vec;
        uint32_t insnLen = s->sleigh.oneInstruction(s->pcodeRaw, address);
        s->loader.buf = nullptr;
        s->pcodeRaw.vec = nullptr;
        return insnLen;
    }
}
