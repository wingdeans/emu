use core::ffi::c_int;
use std::cell::RefCell;
use std::rc::Rc;

use library::cpu::Cpu;

const MEMORY_ACCESS_MODE_WRITE: c_int = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
pub(crate) struct MemoryAccessTracker<'a> {
    mem: &'a [u8],
    mem_accesses: Vec<MemoryAccess>,
}

impl library::bus::Addressable for MemoryAccessTracker<'_> {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match self.mem.get(addr as usize) {
            Some(v) => Some(*v),
            None => Some(0xaa),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        self.mem_accesses.push(MemoryAccess {
            mode: MEMORY_ACCESS_MODE_WRITE,
            address: addr,
            value,
        });

        Some(())
    }
}

// thread_local! {
// static CPU: RefCell<MaybeUninit<crate::Cpu<MemoryAccessTracker<'static>>>> =
// RefCell::new(MaybeUninit::uninit());
// }

#[no_mangle]
unsafe extern "C" fn init(memory_size: usize, memory: *const u8) {
    let mem = unsafe { std::slice::from_raw_parts(memory, memory_size) };
    let tracker = MemoryAccessTracker {
        mem: mem.into(),
        mem_accesses: Vec::new(),
    };

    let cpu = crate::Cpu::new(Rc::new(RefCell::new(tracker)));
    crate::CPU.with_borrow_mut(|cell| {
        cell.set(Rc::new(RefCell::new(cpu))).unwrap();
    });
}

#[no_mangle]
unsafe extern "C" fn set_state(state: *const State) {
    let s = &*state;
    crate::CPU.with_borrow_mut(|cell| {
        let mut g = Rc::get_mut(cell.get_mut().unwrap()).unwrap().borrow_mut();
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
        g.state = if s.halted {
            crate::State::Halted
        } else {
            crate::State::Running
        };
        g.ime = s.ime;

        let mut adrb = Rc::get_mut(&mut g.adrb).unwrap().borrow_mut();
        adrb.mem_accesses.clear();
    })
}

#[no_mangle]
unsafe extern "C" fn get_state(state: *mut State) {
    let s = &mut *state;
    crate::CPU.with_borrow_mut(|cell| {
        let mut g = Rc::get_mut(cell.get_mut().unwrap()).unwrap().borrow_mut();

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
        s.halted = g.state != crate::State::Running;
        s.ime = g.ime;

        let mut adrb = Rc::get_mut(&mut g.adrb).unwrap().borrow_mut();
        s.num_mem_access = adrb.mem_accesses.len() as c_int;
        s.mem_accesses[..adrb.mem_accesses.len()].copy_from_slice(&adrb.mem_accesses);
    });
}

#[no_mangle]
unsafe extern "C" fn step() -> c_int {
    crate::CPU.with_borrow_mut(|cell| {
        let mut g = Rc::get_mut(cell.get_mut().unwrap()).unwrap().borrow_mut();
        g.execute()
    });
    -1
}
