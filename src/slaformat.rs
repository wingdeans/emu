// see Ghidra file: slaformat.cc

macro_rules! create_enum {
    {
        $enum_name:ident
        $($x:ident $name:ident = $y:ident($str:literal, $num:literal, $z:ident);)*
    } => {
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug)]
        pub(crate) enum $enum_name {
            $($name = $num,)*
        }

        impl From<u16> for $enum_name {
            fn from(v: u16) -> Self {
                match v {
                    $($num => Self::$name,)*
                        _ => unreachable!()
                }
            }
        }
    }
}

create_enum! {
    AId
    AttributeId VAL = AttributeId("val", 2, FORMAT_SCOPE);
    AttributeId ID = AttributeId("id", 3, FORMAT_SCOPE);
    AttributeId SPACE = AttributeId("space", 4, FORMAT_SCOPE);
    AttributeId S = AttributeId("s", 5, FORMAT_SCOPE);
    AttributeId OFF = AttributeId("off", 6, FORMAT_SCOPE);
    AttributeId CODE = AttributeId("code", 7, FORMAT_SCOPE);
    AttributeId MASK = AttributeId("mask", 8, FORMAT_SCOPE);
    AttributeId INDEX = AttributeId("index", 9, FORMAT_SCOPE);
    AttributeId NONZERO = AttributeId("nonzero", 10, FORMAT_SCOPE);
    AttributeId PIECE = AttributeId("piece", 11, FORMAT_SCOPE);
    AttributeId NAME = AttributeId("name", 12, FORMAT_SCOPE);
    AttributeId SCOPE = AttributeId("scope", 13, FORMAT_SCOPE);
    AttributeId STARTBIT = AttributeId("startbit", 14, FORMAT_SCOPE);
    AttributeId SIZE = AttributeId("size", 15, FORMAT_SCOPE);
    AttributeId TABLE = AttributeId("table", 16, FORMAT_SCOPE);
    AttributeId CT = AttributeId("ct", 17, FORMAT_SCOPE);
    AttributeId MINLEN = AttributeId("minlen", 18, FORMAT_SCOPE);
    AttributeId BASE = AttributeId("base", 19, FORMAT_SCOPE);
    AttributeId NUMBER = AttributeId("number", 20, FORMAT_SCOPE);
    AttributeId CONTEXT = AttributeId("context", 21, FORMAT_SCOPE);
    AttributeId PARENT = AttributeId("parent", 22, FORMAT_SCOPE);
    AttributeId SUBSYM = AttributeId("subsym", 23, FORMAT_SCOPE);
    AttributeId LINE = AttributeId("line", 24, FORMAT_SCOPE);
    AttributeId SOURCE = AttributeId("source", 25, FORMAT_SCOPE);
    AttributeId LENGTH = AttributeId("length", 26, FORMAT_SCOPE);
    AttributeId FIRST = AttributeId("first", 27, FORMAT_SCOPE);
    AttributeId PLUS = AttributeId("plus", 28, FORMAT_SCOPE);
    AttributeId SHIFT = AttributeId("shift", 29, FORMAT_SCOPE);
    AttributeId ENDBIT = AttributeId("endbit", 30, FORMAT_SCOPE);
    AttributeId SIGNBIT = AttributeId("signbit", 31, FORMAT_SCOPE);
    AttributeId ENDBYTE = AttributeId("endbyte", 32, FORMAT_SCOPE);
    AttributeId STARTBYTE = AttributeId("startbyte", 33, FORMAT_SCOPE);

    AttributeId VERSION = AttributeId("version", 34, FORMAT_SCOPE);
    AttributeId BIGENDIAN = AttributeId("bigendian", 35, FORMAT_SCOPE);
    AttributeId ALIGN = AttributeId("align", 36, FORMAT_SCOPE);
    AttributeId UNIQBASE = AttributeId("uniqbase", 37, FORMAT_SCOPE);
    AttributeId MAXDELAY = AttributeId("maxdelay", 38, FORMAT_SCOPE);
    AttributeId UNIQMASK = AttributeId("uniqmask", 39, FORMAT_SCOPE);
    AttributeId NUMSECTIONS = AttributeId("numsections", 40, FORMAT_SCOPE);
    AttributeId DEFAULTSPACE = AttributeId("defaultspace", 41, FORMAT_SCOPE);
    AttributeId DELAY = AttributeId("delay", 42, FORMAT_SCOPE);
    AttributeId WORDSIZE = AttributeId("wordsize", 43, FORMAT_SCOPE);
    AttributeId PHYSICAL = AttributeId("physical", 44, FORMAT_SCOPE);
    AttributeId SCOPESIZE = AttributeId("scopesize", 45, FORMAT_SCOPE);
    AttributeId SYMBOLSIZE = AttributeId("symbolsize", 46, FORMAT_SCOPE);
    AttributeId VARNODE = AttributeId("varnode", 47, FORMAT_SCOPE);
    AttributeId LOW = AttributeId("low", 48, FORMAT_SCOPE);
    AttributeId HIGH = AttributeId("high", 49, FORMAT_SCOPE);
    AttributeId FLOW = AttributeId("flow", 50, FORMAT_SCOPE);
    AttributeId CONTAIN = AttributeId("contain", 51, FORMAT_SCOPE);
    AttributeId I = AttributeId("i", 52, FORMAT_SCOPE);
    AttributeId NUMCT = AttributeId("numct", 53, FORMAT_SCOPE);
    AttributeId SECTION = AttributeId("section", 54, FORMAT_SCOPE);
    AttributeId LABELS = AttributeId("labels", 55, FORMAT_SCOPE);
}

create_enum! {
    EId
    ElementId CONST_REAL = ElementId("const_real", 1, FORMAT_SCOPE);
    ElementId VARNODE_TPL = ElementId("varnode_tpl", 2, FORMAT_SCOPE);
    ElementId CONST_SPACEID = ElementId("const_spaceid", 3, FORMAT_SCOPE);
    ElementId CONST_HANDLE = ElementId("const_handle", 4, FORMAT_SCOPE);
    ElementId OP_TPL = ElementId("op_tpl", 5, FORMAT_SCOPE);
    ElementId MASK_WORD = ElementId("mask_word", 6, FORMAT_SCOPE);
    ElementId PAT_BLOCK = ElementId("pat_block", 7, FORMAT_SCOPE);
    ElementId PRINT = ElementId("print", 8, FORMAT_SCOPE);
    ElementId PAIR = ElementId("pair", 9, FORMAT_SCOPE);
    ElementId CONTEXT_PAT = ElementId("context_pat", 10, FORMAT_SCOPE);
    ElementId NULL = ElementId("null", 11, FORMAT_SCOPE);
    ElementId OPERAND_EXP = ElementId("operand_exp", 12, FORMAT_SCOPE);
    ElementId OPERAND_SYM = ElementId("operand_sym", 13, FORMAT_SCOPE);
    ElementId OPERAND_SYM_HEAD = ElementId("operand_sym_head", 14, FORMAT_SCOPE);
    ElementId OPER = ElementId("oper", 15, FORMAT_SCOPE);
    ElementId DECISION = ElementId("decision", 16, FORMAT_SCOPE);
    ElementId OPPRINT = ElementId("opprint", 17, FORMAT_SCOPE);
    ElementId INSTRUCT_PAT = ElementId("instruct_pat", 18, FORMAT_SCOPE);
    ElementId COMBINE_PAT = ElementId("combine_pat", 19, FORMAT_SCOPE);
    ElementId CONSTRUCTOR = ElementId("constructor", 20, FORMAT_SCOPE);
    ElementId CONSTRUCT_TPL = ElementId("construct_tpl", 21, FORMAT_SCOPE);
    ElementId SCOPE = ElementId("scope", 22, FORMAT_SCOPE);
    ElementId VARNODE_SYM = ElementId("varnode_sym", 23, FORMAT_SCOPE);
    ElementId VARNODE_SYM_HEAD = ElementId("varnode_sym_head", 24, FORMAT_SCOPE);
    ElementId USEROP = ElementId("userop", 25, FORMAT_SCOPE);
    ElementId USEROP_HEAD = ElementId("userop_head", 26, FORMAT_SCOPE);
    ElementId TOKENFIELD = ElementId("tokenfield", 27, FORMAT_SCOPE);
    ElementId VAR = ElementId("var", 28, FORMAT_SCOPE);
    ElementId CONTEXTFIELD = ElementId("contextfield", 29, FORMAT_SCOPE);
    ElementId HANDLE_TPL = ElementId("handle_tpl", 30, FORMAT_SCOPE);
    ElementId CONST_RELATIVE = ElementId("const_relative", 31, FORMAT_SCOPE);
    ElementId CONTEXT_OP = ElementId("context_op", 32, FORMAT_SCOPE);

    ElementId SLEIGH = ElementId("sleigh", 33, FORMAT_SCOPE);
    ElementId SPACES = ElementId("spaces", 34, FORMAT_SCOPE);
    ElementId SOURCEFILES = ElementId("sourcefiles", 35, FORMAT_SCOPE);
    ElementId SOURCEFILE = ElementId("sourcefile", 36, FORMAT_SCOPE);
    ElementId SPACE = ElementId("space", 37, FORMAT_SCOPE);
    ElementId SYMBOL_TABLE = ElementId("symbol_table", 38, FORMAT_SCOPE);
    ElementId VALUE_SYM = ElementId("value_sym", 39, FORMAT_SCOPE);
    ElementId VALUE_SYM_HEAD = ElementId("value_sym_head", 40, FORMAT_SCOPE);
    ElementId CONTEXT_SYM = ElementId("context_sym", 41, FORMAT_SCOPE);
    ElementId CONTEXT_SYM_HEAD = ElementId("context_sym_head", 42, FORMAT_SCOPE);
    ElementId END_SYM = ElementId("end_sym", 43, FORMAT_SCOPE);
    ElementId END_SYM_HEAD = ElementId("end_sym_head", 44, FORMAT_SCOPE);
    ElementId SPACE_OTHER = ElementId("space_other", 45, FORMAT_SCOPE);
    ElementId SPACE_UNIQUE = ElementId("space_unique", 46, FORMAT_SCOPE);
    ElementId AND_EXP = ElementId("and_exp", 47, FORMAT_SCOPE);
    ElementId DIV_EXP = ElementId("div_exp", 48, FORMAT_SCOPE);
    ElementId LSHIFT_EXP = ElementId("lshift_exp", 49, FORMAT_SCOPE);
    ElementId MINUS_EXP = ElementId("minus_exp", 50, FORMAT_SCOPE);
    ElementId MULT_EXP = ElementId("mult_exp", 51, FORMAT_SCOPE);
    ElementId NOT_EXP = ElementId("not_exp", 52, FORMAT_SCOPE);
    ElementId OR_EXP = ElementId("or_exp", 53, FORMAT_SCOPE);
    ElementId PLUS_EXP = ElementId("plus_exp", 54, FORMAT_SCOPE);
    ElementId RSHIFT_EXP = ElementId("rshift_exp", 55, FORMAT_SCOPE);
    ElementId SUB_EXP = ElementId("sub_exp", 56, FORMAT_SCOPE);
    ElementId XOR_EXP = ElementId("xor_exp", 57, FORMAT_SCOPE);
    ElementId INTB = ElementId("intb", 58, FORMAT_SCOPE);
    ElementId END_EXP = ElementId("end_exp", 59, FORMAT_SCOPE);
    ElementId NEXT2_EXP = ElementId("next2_exp", 60, FORMAT_SCOPE);
    ElementId START_EXP = ElementId("start_exp", 61, FORMAT_SCOPE);
    ElementId EPSILON_SYM = ElementId("epsilon_sym", 62, FORMAT_SCOPE);
    ElementId EPSILON_SYM_HEAD = ElementId("epsilon_sym_head", 63, FORMAT_SCOPE);
    ElementId NAME_SYM = ElementId("name_sym", 64, FORMAT_SCOPE);
    ElementId NAME_SYM_HEAD = ElementId("name_sym_head", 65, FORMAT_SCOPE);
    ElementId NAMETAB = ElementId("nametab", 66, FORMAT_SCOPE);
    ElementId NEXT2_SYM = ElementId("next2_sym", 67, FORMAT_SCOPE);
    ElementId NEXT2_SYM_HEAD = ElementId("next2_sym_head", 68, FORMAT_SCOPE);
    ElementId START_SYM = ElementId("start_sym", 69, FORMAT_SCOPE);
    ElementId START_SYM_HEAD = ElementId("start_sym_head", 70, FORMAT_SCOPE);
    ElementId SUBTABLE_SYM = ElementId("subtable_sym", 71, FORMAT_SCOPE);
    ElementId SUBTABLE_SYM_HEAD = ElementId("subtable_sym_head", 72, FORMAT_SCOPE);
    ElementId VALUEMAP_SYM = ElementId("valuemap_sym", 73, FORMAT_SCOPE);
    ElementId VALUEMAP_SYM_HEAD = ElementId("valuemap_sym_head", 74, FORMAT_SCOPE);
    ElementId VALUETAB = ElementId("valuetab", 75, FORMAT_SCOPE);
    ElementId VARLIST_SYM = ElementId("varlist_sym", 76, FORMAT_SCOPE);
    ElementId VARLIST_SYM_HEAD = ElementId("varlist_sym_head", 77, FORMAT_SCOPE);
    ElementId OR_PAT = ElementId("or_pat", 78, FORMAT_SCOPE);
    ElementId COMMIT = ElementId("commit", 79, FORMAT_SCOPE);
    ElementId CONST_START = ElementId("const_start", 80, FORMAT_SCOPE);
    ElementId CONST_NEXT = ElementId("const_next", 81, FORMAT_SCOPE);
    ElementId CONST_NEXT2 = ElementId("const_next2", 82, FORMAT_SCOPE);
    ElementId CONST_CURSPACE = ElementId("const_curspace", 83, FORMAT_SCOPE);
    ElementId CONST_CURSPACE_SIZE = ElementId("const_curspace_size", 84, FORMAT_SCOPE);
    ElementId CONST_FLOWREF = ElementId("const_flowref", 85, FORMAT_SCOPE);
    ElementId CONST_FLOWREF_SIZE = ElementId("const_flowref_size", 86, FORMAT_SCOPE);
    ElementId CONST_FLOWDEST = ElementId("const_flowdest", 87, FORMAT_SCOPE);
    ElementId CONST_FLOWDEST_SIZE = ElementId("const_flowdest_size", 88, FORMAT_SCOPE);
}
