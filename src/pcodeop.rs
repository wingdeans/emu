// see Ghidra file: PcodeOp.java
//              or: opcodes.hh

macro_rules! create_pcode {
    {
        $($pub:ident $stat:ident $fin:ident $int:ident $name:ident = $num:literal;)*
    } => {
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug, Clone, Copy)]
        pub(crate) enum PcodeOp {
            $($name = $num,)*
        }

        impl From<u8> for PcodeOp {
            fn from(v: u8) -> Self {
                match v {
                    $($num => Self::$name,)*
                        _ => unreachable!()
                }
            }
        }
    }
}

create_pcode! {
    // The opcodes of the Pcode language

    // Each Pcode Op is given a unique identifying index here
    public static final int UNIMPLEMENTED = 0;		// Place holder for unimplemented instruction
    public static final int COPY = 1;		        // Copy one operand to another
    public static final int LOAD = 2;		        // Dereference a pointer into specified space
    public static final int STORE = 3;		        // Store at a pointer into specified space

    public static final int BRANCH = 4;		// Always branch
    public static final int CBRANCH = 5;		// Conditional branch
    public static final int BRANCHIND = 6;		// An indirect branch (jumptable)

    public static final int CALL = 7;		        // A call with absolute address
    public static final int CALLIND = 8;		// An indirect call
    public static final int CALLOTHER = 9;     // Other unusual subroutine calling conventions
    public static final int RETURN = 10;		// A return from subroutine

    public static final int INT_EQUAL = 11;	        // Return TRUE if operand1 == operand2
    public static final int INT_NOTEQUAL = 12;	        // Return TRUE if operand1 != operand2
    public static final int INT_SLESS = 13;         	// Return TRUE if signed op1 < signed op2
    public static final int INT_SLESSEQUAL = 14;	// Return TRUE if signed op1 <= signed op2
    public static final int INT_LESS = 15;		// Return TRUE if unsigned op1 < unsigned op2
    // Also indicates borrow on unsigned subtraction
    public static final int INT_LESSEQUAL = 16;	// Return TRUE if unsigned op1 <= unsigned op2
    public static final int INT_ZEXT = 17;		// Zero extend operand
    public static final int INT_SEXT = 18;		// Sign extend operand
    public static final int INT_ADD = 19;		// Unsigned addition of operands of same size
    public static final int INT_SUB = 20;		// Unsigned subtraction of operands of same size
    public static final int INT_CARRY = 21;        	// TRUE if adding two operands has overflow (carry)
    public static final int INT_SCARRY = 22;   	// TRUE if carry in signed addition of 2 ops
    public static final int INT_SBORROW = 23;  	// TRUE if borrow in signed subtraction of 2 ops
    public static final int INT_2COMP = 24;    	// Twos complement (for subtracting) of operand
    public static final int INT_NEGATE = 25;
    public static final int INT_XOR = 26;		// Exclusive OR of two operands of same size
    public static final int INT_AND = 27;
    public static final int INT_OR = 28;
    public static final int INT_LEFT = 29;		// Left shift
    public static final int INT_RIGHT = 30;	        // Right shift zero fill
    public static final int INT_SRIGHT = 31;        	// Signed right shift
    public static final int INT_MULT = 32;		// Integer multiplication
    public static final int INT_DIV = 33;		// Unsigned integer division
    public static final int INT_SDIV = 34;		// Signed integer division
    public static final int INT_REM = 35;		// Unsigned mod (remainder)
    public static final int INT_SREM = 36;		// Signed mod (remainder)

    public static final int BOOL_NEGATE = 37;  	// Boolean negate or not
    public static final int BOOL_XOR = 38;		// Boolean xor
    public static final int BOOL_AND = 39;		// Boolean and (&&)
    public static final int BOOL_OR = 40;		// Boolean or (||)

    // floating point instructions:  No floating point data format is specified here,
    // although the exact operation of these instructions obviously depends on the
    // format.  For simulation, a "mode" variable specifying the floating point format
    // will be necessary.
    public static final int FLOAT_EQUAL = 41;          // Return TRUE if operand1 == operand2
    public static final int FLOAT_NOTEQUAL = 42;	// Return TRUE if operand1 != operand2
    public static final int FLOAT_LESS = 43;   	// Return TRUE if op1 < op2
    public static final int FLOAT_LESSEQUAL = 44;	// Return TRUE if op1 <= op2
    // Slot 45 is unused
    public static final int FLOAT_NAN = 46;	// Return TRUE if neither op1 is NaN

    public static final int FLOAT_ADD = 47;            // float addition
    public static final int FLOAT_DIV = 48;            // float division
    public static final int FLOAT_MULT = 49;           // float multiplication
    public static final int FLOAT_SUB = 50;            // float subtraction
    public static final int FLOAT_NEG = 51;            // float negation
    public static final int FLOAT_ABS = 52;            // float absolute value
    public static final int FLOAT_SQRT = 53;           // float square root

    public static final int FLOAT_INT2FLOAT = 54;      // convert int type to float type
    public static final int FLOAT_FLOAT2FLOAT = 55;    // convert between float sizes
    public static final int FLOAT_TRUNC = 56;          // round towards zero
    public static final int FLOAT_CEIL = 57;           // round towards +infinity
    public static final int FLOAT_FLOOR = 58;          // round towards -infinity
    public static final int FLOAT_ROUND = 59;          // round towards nearest

    // Internal opcodes for simplification.  Not typically generated in direct
    // translation.
    public static final int MULTIEQUAL = 60;  // Output equal to one of inputs, depending on execution
    public static final int INDIRECT = 61;    // Output probably equals input, but may be indirectly affected
    public static final int PIECE = 62;       // Output is constructed from multiple pieces
    public static final int SUBPIECE = 63;    // Output is a subpiece of input0, input1=offset into input0

    public static final int CAST = 64;        // Cast from one type to another
    public static final int PTRADD = 65;      // outptr = ptrbase,offset, (size multiplier)
    public static final int PTRSUB = 66;      // outptr = &(ptr->subfield)
    public static final int SEGMENTOP = 67;
    public static final int CPOOLREF = 68;
    public static final int NEW = 69;
    public static final int INSERT = 70;
    public static final int EXTRACT = 71;
    public static final int POPCOUNT = 72;
    public static final int LZCOUNT = 73;

    public static final int PCODE_MAX = 74;
}
