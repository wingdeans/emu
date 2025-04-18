use crate::{
    apu::Apu,
    bus::{bank, map_to, Addressable, Bus},
    cartridge::{Cartridge, CGB_FLAG_ADDRESS},
    input::{Input, InputHandler},
    int::{Interrupt, InterruptHandler},
    io::IO,
    memory::{Access, Memory},
    palette::Palette,
    ppu::Ppu,
    serial::Serial,
    surface::Surface,
};
use std::{cell::RefCell, rc::Rc};

pub const VRAM_BEGIN: u16 = 0x8000;
pub const VRAM_END: u16 = 0xa000;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_BEGIN) as usize;
pub const WRAM_0_BEGIN: u16 = 0xc000;
pub const WRAM_0_END: u16 = 0xd000;
pub const WRAM_X_BEGIN: u16 = 0xd000;
pub const WRAM_X_END: u16 = 0xe000;
pub const WRAM_BANK_SIZE: usize = (WRAM_X_END - WRAM_X_BEGIN) as usize;
pub const WRAM_BANK_COUNT: u32 = 8;
pub const ECHO_RAM_BEGIN: u16 = 0xe000;
pub const ECHO_RAM_END: u16 = 0xfe00;
pub const HRAM_BEGIN: u16 = 0xff7f; // Should be 0xff80, used as workaround
pub const HRAM_END: u16 = 0xffff;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_BEGIN) as usize;
pub const OAM_BEGIN: u16 = 0xfe00;
pub const OAM_END: u16 = 0xfea0;
pub const OAM_SIZE: usize = (OAM_END - OAM_BEGIN) as usize;
pub const ILLEGAL_BEGIN: u16 = 0xfea0;
pub const ILLEGAL_END: u16 = 0xff00;
pub const ILLEGAL_SIZE: usize = (ILLEGAL_END - ILLEGAL_BEGIN) as usize;

pub struct System {
    bus: Rc<RefCell<Bus>>,
    ppu: Rc<RefCell<Ppu>>,
    apu: Rc<RefCell<Apu>>,
    io: Rc<RefCell<IO>>,
    palette: Rc<RefCell<Palette>>,
}

impl System {
    pub fn new(
        cartridge: Rc<RefCell<Cartridge>>,
        surface: Rc<RefCell<dyn Surface>>,
        input_handler: Rc<RefCell<dyn InputHandler>>,
        interrupt_handler: Rc<RefCell<dyn InterruptHandler>>,
    ) -> Self {
        let bus: Rc<RefCell<Bus>> = Default::default();

        let vram_bank = bank(
            Rc::new(RefCell::new(Memory::new(VRAM_SIZE * 2, Access::ReadWrite))),
            VRAM_SIZE,
            2,
        );

        let vram0 = map_to(
            Rc::clone(vram_bank.borrow().bank(0)),
            VRAM_BEGIN..VRAM_END,
            VRAM_SIZE as u16,
        );
        let vram1 = map_to(
            Rc::clone(vram_bank.borrow().bank(1)),
            VRAM_BEGIN..VRAM_END,
            VRAM_SIZE as u16,
        );

        let vram = map_to(
            Rc::clone(&vram_bank) as Rc<RefCell<dyn Addressable>>,
            VRAM_BEGIN..VRAM_END,
            VRAM_SIZE as u16,
        );

        let oam = map_to(
            Rc::new(RefCell::new(Memory::new(OAM_SIZE, Access::ReadWrite))),
            OAM_BEGIN..OAM_END,
            OAM_SIZE as u16,
        );

        let wram = bank(
            Rc::new(RefCell::new(Memory::new(
                WRAM_BANK_SIZE * WRAM_BANK_COUNT as usize,
                Access::ReadWrite,
            ))),
            WRAM_BANK_SIZE,
            WRAM_BANK_COUNT,
        );

        wram.borrow_mut().select(1);

        let wram_0 = map_to(
            Rc::clone(wram.borrow().bank(0)),
            WRAM_0_BEGIN..WRAM_X_BEGIN,
            WRAM_BANK_SIZE as u16,
        );

        let hram = map_to(
            Rc::new(RefCell::new(Memory::new(HRAM_SIZE, Access::ReadWrite))),
            HRAM_BEGIN..HRAM_END,
            HRAM_SIZE as u16,
        );

        let cgb_flag = cartridge.borrow_mut().read(CGB_FLAG_ADDRESS).unwrap();

        let int = Rc::new(RefCell::new(Interrupt::new(interrupt_handler)));
        let io = Rc::new(RefCell::new(IO::new(
            Rc::clone(&int),
            Rc::clone(&wram),
            vram_bank,
        )));
        let palette = Rc::new(RefCell::new(Palette::new(
            cgb_flag != 0x80 && cgb_flag != 0xc0,
        )));

        let ppu = Rc::new(RefCell::new(Ppu::new(
            surface,
            Rc::clone(&bus) as Rc<RefCell<dyn Addressable>>,
            vram0,
            vram1,
            Rc::clone(&oam),
            Rc::clone(&palette),
            Rc::clone(&int),
        )));

        let apu = Rc::new(RefCell::new(Apu::default()));

        let illegal = map_to(
            Rc::new(RefCell::new(Memory::new(ILLEGAL_SIZE, Access::ReadWrite))),
            ILLEGAL_BEGIN..ILLEGAL_END,
            ILLEGAL_SIZE as u16,
        );

        {
            let mut b = bus.borrow_mut();
            b.add(cartridge);
            b.add(vram);
            b.add(wram_0);
            b.add(map_to(
                wram,
                WRAM_X_BEGIN..WRAM_X_END,
                WRAM_BANK_SIZE as u16,
            ));
            b.add(hram);
            b.add(Rc::clone(&io) as Rc<RefCell<dyn Addressable>>);
            b.add(Rc::clone(&palette) as Rc<RefCell<dyn Addressable>>);
            b.add(Rc::clone(&ppu) as Rc<RefCell<dyn Addressable>>);
            b.add(Rc::new(RefCell::new(Input::new(input_handler))));
            b.add(int);
            b.add(oam);
            b.add(Rc::new(RefCell::new(Serial::default())));
            b.add(Rc::clone(&apu) as Rc<RefCell<dyn Addressable>>);
            b.add(illegal);
        }

        Self {
            bus,
            ppu,
            apu,
            io,
            palette,
        }
    }

    pub fn bus_ref(&mut self) -> &Rc<RefCell<Bus>> {
        &self.bus
    }

    pub fn ppu_ref(&self) -> &Rc<RefCell<Ppu>> {
        &self.ppu
    }

    pub fn apu_ref(&self) -> &Rc<RefCell<Apu>> {
        &self.apu
    }

    pub fn io_ref(&self) -> &Rc<RefCell<IO>> {
        &self.io
    }

    pub fn palette_ref(&self) -> &Rc<RefCell<Palette>> {
        &self.palette
    }
}

impl Addressable for System {
    fn read(&mut self, addr: u16) -> Option<u8> {
        match addr {
            ECHO_RAM_BEGIN..ECHO_RAM_END => self.read(addr - ECHO_RAM_BEGIN + 0xc000),
            _ => self.bus.borrow_mut().read(addr).or(Some(0)),
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> Option<()> {
        match addr {
            ECHO_RAM_BEGIN..ECHO_RAM_END => self.write(addr - ECHO_RAM_BEGIN + 0xc000, value),
            _ => self.bus.borrow_mut().write(addr, value).or(Some(())),
        }
    }
}
