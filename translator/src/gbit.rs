use core::ffi::c_int;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

use library::cpu::Cpu;

#[repr(C)]
#[derive(Copy, Clone)]
struct MemoryAccess {
    mode: c_int,
    address: u16,
    value: u8,
}

#[repr(C)]
struct Registers {
    // AF
    f: u8,
    a: u8,
    // BC
    c: u8,
    b: u8,
    // DE
    e: u8,
    d: u8,
    // HL
    l: u8,
    h: u8,
}

#[repr(C)]
struct State {
    registers: Registers,
    sp: u16,
    pc: u16,
    halted: bool,
    ime: bool,
    num_mem_access: c_int,
    mem_accesses: [MemoryAccess; 16],
}

struct MemoryAccessTracker<'a> {
    mem: &'a [u8],
    mem_accesses: Vec<MemoryAccess>,
}

impl library::bus::Addressable for MemoryAccessTracker<'_> {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match self.mem.get(addr as usize) {
            Some(v) => Some(*v),
            None => None,
        }
    }
    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        todo!()
    }
}

thread_local! {
    static CPU: RefCell<MaybeUninit<crate::Cpu<MemoryAccessTracker<'static>>>> =
        RefCell::new(MaybeUninit::uninit());
}

#[no_mangle]
unsafe extern "C" fn init(memory_size: usize, memory: *const u8) {
    let mem = unsafe { std::slice::from_raw_parts(memory, memory_size) };
    let tracker = MemoryAccessTracker {
        mem: mem.into(),
        mem_accesses: Vec::new(),
    };

    let cpu = crate::Cpu::new(Rc::new(RefCell::new(tracker)));
    CPU.with_borrow_mut(|uninit| {
        uninit.write(cpu);
    });
}

#[no_mangle]
unsafe extern "C" fn set_state(state: *const State) {
    let s = &*state;
    CPU.with_borrow_mut(|uninit| {
        let g = uninit.assume_init_mut();
        g.regs = crate::Regs {
            f: s.registers.f,
            a: s.registers.a,
            c: s.registers.c,
            b: s.registers.b,
            e: s.registers.e,
            d: s.registers.d,
            l: s.registers.l,
            h: s.registers.h,
            sp: s.sp,
        };

        g.pc = s.pc;
    })
}

#[no_mangle]
unsafe extern "C" fn get_state(state: *mut State) {
    let s = &mut *state;
    CPU.with_borrow(|uninit| {
        let g = uninit.assume_init_ref();

        let regs = &g.regs;
        s.registers = Registers {
            f: regs.f,
            a: regs.a,
            c: regs.c,
            b: regs.b,
            e: regs.e,
            d: regs.d,
            l: regs.l,
            h: regs.h,
        };
        s.sp = regs.sp;
        s.pc = g.pc;
    });

    s.halted = false;
    s.ime = false;
    s.num_mem_access = 0;
}

#[no_mangle]
unsafe extern "C" fn step() -> c_int {
    CPU.with_borrow_mut(|uninit| uninit.assume_init_mut().execute());
    -1
}
