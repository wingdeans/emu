use crate::cpu::{Cpu, State as StateEnum};
use library::{bus::Addressable, error::Error as LibError};
use std::{cell::RefCell, mem::MaybeUninit, os::raw::c_int, rc::Rc};

type CBool = bool;
type CSizeT = usize;

const MEMORY_ACCESS_MODE_WRITE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
struct MemoryAccess {
    mode: c_int,
    address: u16,
    value: u8,
}

#[repr(C)]
struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
}

#[repr(C)]
struct State {
    registers: Registers,
    sp: u16,
    pc: u16,
    halted: CBool,
    ime: CBool,
    num_mem_access: c_int,
    mem_accesses: [MemoryAccess; 16],
}

struct Global {
    cpu: Cpu,
    memory_size: usize,
    memory: *const u8,
    num_mem_access: usize,
    mem_accesses: Vec<MemoryAccess>,
}

#[derive(Default)]
struct GlobalBus {}

impl Addressable for GlobalBus {
    fn read(&mut self, addr: u16) -> Option<u8> {
        unsafe {
            let g = GLOBAL.assume_init_ref();

            if addr as usize >= g.memory_size {
                Some(0xaa)
            } else {
                Some(*g.memory.offset(addr as isize))
            }
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        unsafe {
            let g = GLOBAL.assume_init_mut();

            if g.num_mem_access == 16 {
                None
            } else {
                g.mem_accesses.push(MemoryAccess {
                    mode: MEMORY_ACCESS_MODE_WRITE,
                    address: addr,
                    value,
                });

                g.num_mem_access += 1;
                Some(())
            }
        }
    }
}

static mut GLOBAL: MaybeUninit<Global> = MaybeUninit::uninit();

#[no_mangle]
unsafe extern "C" fn init(memory_size: CSizeT, memory: *const u8) {
    GLOBAL.write(Global {
        cpu: Cpu::new(Rc::new(RefCell::<GlobalBus>::new(Default::default()))),
        memory_size: memory_size as usize,
        memory,
        num_mem_access: 0,
        mem_accesses: Vec::new(),
    });
}

#[no_mangle]
unsafe extern "C" fn set_state(state: *const State) {
    let g = GLOBAL.assume_init_mut();
    let s = &*state;

    g.cpu.af = s.registers.af;
    g.cpu.bc = s.registers.bc;
    g.cpu.de = s.registers.de;
    g.cpu.hl = s.registers.hl;
    g.cpu.sp = s.sp;
    g.cpu.pc = s.pc;
    g.cpu.ime = s.ime;
    g.cpu.state = if s.halted {
        StateEnum::Halted
    } else {
        StateEnum::Running
    };
    g.num_mem_access = 0;
    g.mem_accesses.clear();
}

#[no_mangle]
unsafe extern "C" fn get_state(state: *mut State) {
    let g = GLOBAL.assume_init_ref();
    let s = &mut *state;

    s.registers.af = g.cpu.af;
    s.registers.bc = g.cpu.bc;
    s.registers.de = g.cpu.de;
    s.registers.hl = g.cpu.hl;
    s.sp = g.cpu.sp;
    s.pc = g.cpu.pc;
    s.ime = g.cpu.ime;
    s.halted = g.cpu.state != StateEnum::Running;
    s.num_mem_access = g.num_mem_access as c_int;
    s.mem_accesses[..g.mem_accesses.len()].copy_from_slice(&g.mem_accesses);
}

#[no_mangle]
unsafe extern "C" fn step() -> c_int {
    match GLOBAL.assume_init_mut().cpu.execute() {
        Ok(x) => (x * 4) as c_int,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}
