use std::cell::RefCell;
use std::rc::Rc;

use super::arm7tdmi::{Addr, Bus};
use super::gba::IoDevices;
use super::sysbus::BoxedMemory;
use super::keypad;

pub mod consts {
    use super::*;

    pub const IO_BASE: Addr = 0x0400_0000;

    // LCD I/O Registers
    pub const REG_DISPCNT: Addr = IO_BASE + 0x_0000; //  2    R/W     LCD Control
    pub const REG_DISPSTAT: Addr = IO_BASE + 0x_0004; //  2    R/W    General LCD Status (STAT,LYC)
    pub const REG_VCOUNT: Addr = IO_BASE + 0x_0006; //  2    R        Vertical Counter (LY)
    pub const REG_BG0CNT: Addr = IO_BASE + 0x_0008; //  2    R/W      BG0 Control
    pub const REG_BG1CNT: Addr = IO_BASE + 0x_000A; //  2    R/W      BG1 Control
    pub const REG_BG2CNT: Addr = IO_BASE + 0x_000C; //  2    R/W      BG2 Control
    pub const REG_BG3CNT: Addr = IO_BASE + 0x_000E; //  2    R/W      BG3 Control
    pub const REG_BG0HOFS: Addr = IO_BASE + 0x_0010; //  2    W       BG0 X-Offset
    pub const REG_BG0VOFS: Addr = IO_BASE + 0x_0012; //  2    W       BG0 Y-Offset
    pub const REG_BG1HOFS: Addr = IO_BASE + 0x_0014; //  2    W       BG1 X-Offset
    pub const REG_BG1VOFS: Addr = IO_BASE + 0x_0016; //  2    W       BG1 Y-Offset
    pub const REG_BG2HOFS: Addr = IO_BASE + 0x_0018; //  2    W       BG2 X-Offset
    pub const REG_BG2VOFS: Addr = IO_BASE + 0x_001A; //  2    W       BG2 Y-Offset
    pub const REG_BG3HOFS: Addr = IO_BASE + 0x_001C; //  2    W       BG3 X-Offset
    pub const REG_BG3VOFS: Addr = IO_BASE + 0x_001E; //  2    W       BG3 Y-Offset
    pub const REG_BG2PA: Addr = IO_BASE + 0x_0020; //  2    W         BG2 Rotation/Scaling Parameter A (dx)
    pub const REG_BG2PB: Addr = IO_BASE + 0x_0022; //  2    W         BG2 Rotation/Scaling Parameter B (dmx)
    pub const REG_BG2PC: Addr = IO_BASE + 0x_0024; //  2    W         BG2 Rotation/Scaling Parameter C (dy)
    pub const REG_BG2PD: Addr = IO_BASE + 0x_0026; //  2    W         BG2 Rotation/Scaling Parameter D (dmy)
    pub const REG_BG2X: Addr = IO_BASE + 0x_0028; //  4    W          BG2 Reference Point X-Coordinate
    pub const REG_BG2Y: Addr = IO_BASE + 0x_002C; //  4    W          BG2 Reference Point Y-Coordinate
    pub const REG_BG3PA: Addr = IO_BASE + 0x_0030; //  2    W         BG3 Rotation/Scaling Parameter A (dx)
    pub const REG_BG3PB: Addr = IO_BASE + 0x_0032; //  2    W         BG3 Rotation/Scaling Parameter B (dmx)
    pub const REG_BG3PC: Addr = IO_BASE + 0x_0034; //  2    W         BG3 Rotation/Scaling Parameter C (dy)
    pub const REG_BG3PD: Addr = IO_BASE + 0x_0036; //  2    W         BG3 Rotation/Scaling Parameter D (dmy)
    pub const REG_BG3X: Addr = IO_BASE + 0x_0038; //  4    W          BG3 Reference Point X-Coordinate
    pub const REG_BG3Y: Addr = IO_BASE + 0x_003C; //  4    W          BG3 Reference Point Y-Coordinate
    pub const REG_WIN0H: Addr = IO_BASE + 0x_0040; //  2    W         Window 0 Horizontal Dimensions
    pub const REG_WIN1H: Addr = IO_BASE + 0x_0042; //  2    W         Window 1 Horizontal Dimensions
    pub const REG_WIN0V: Addr = IO_BASE + 0x_0044; //  2    W         Window 0 Vertical Dimensions
    pub const REG_WIN1V: Addr = IO_BASE + 0x_0046; //  2    W         Window 1 Vertical Dimensions
    pub const REG_WININ: Addr = IO_BASE + 0x_0048; //  2    R/W       Inside of Window 0 and 1
    pub const REG_WINOUT: Addr = IO_BASE + 0x_004A; //  2    R/W      Inside of OBJ Window & Outside of Windows
    pub const REG_MOSAIC: Addr = IO_BASE + 0x_004C; //  2    W        Mosaic Size
    pub const REG_BLDCNT: Addr = IO_BASE + 0x_0050; //  2    R/W      Color Special Effects Selection
    pub const REG_BLDALPHA: Addr = IO_BASE + 0x_0052; //  2    R/W    Alpha Blending Coefficients
    pub const REG_BLDY: Addr = IO_BASE + 0x_0054; //  2    W          Brightness (Fade-In/Out) Coefficient
                                                  // Sound Registers
    pub const REG_SOUND1CNT_L: Addr = IO_BASE + 0x_0060; //  2  R/W     Channel 1 Sweep register       (NR10)
    pub const REG_SOUND1CNT_H: Addr = IO_BASE + 0x_0062; //  2  R/W     Channel 1 Duty/Length/Envelope (NR11, NR12)
    pub const REG_SOUND1CNT_X: Addr = IO_BASE + 0x_0064; //  2  R/W     Channel 1 Frequency/Control    (NR13, NR14)
    pub const REG_SOUND2CNT_L: Addr = IO_BASE + 0x_0068; //  2  R/W     Channel 2 Duty/Length/Envelope (NR21, NR22)
    pub const REG_SOUND2CNT_H: Addr = IO_BASE + 0x_006C; //  2  R/W     Channel 2 Frequency/Control    (NR23, NR24)
    pub const REG_SOUND3CNT_L: Addr = IO_BASE + 0x_0070; //  2  R/W     Channel 3 Stop/Wave RAM select (NR30)
    pub const REG_SOUND3CNT_H: Addr = IO_BASE + 0x_0072; //  2  R/W     Channel 3 Length/Volume        (NR31, NR32)
    pub const REG_SOUND3CNT_X: Addr = IO_BASE + 0x_0074; //  2  R/W     Channel 3 Frequency/Control    (NR33, NR34)
    pub const REG_SOUND4CNT_L: Addr = IO_BASE + 0x_0078; //  2  R/W     Channel 4 Length/Envelope      (NR41, NR42)
    pub const REG_SOUND4CNT_H: Addr = IO_BASE + 0x_007C; //  2  R/W     Channel 4 Frequency/Control    (NR43, NR44)
    pub const REG_SOUNDCNT_L: Addr = IO_BASE + 0x_0080; //  2  R/W      Control Stereo/Volume/Enable   (NR50, NR51)
    pub const REG_SOUNDCNT_H: Addr = IO_BASE + 0x_0082; //  2  R/W      Control Mixing/DMA Control
    pub const REG_SOUNDCNT_X: Addr = IO_BASE + 0x_0084; //  2  R/W      Control Sound on/off           (NR52)
    pub const REG_SOUNDBIAS: Addr = IO_BASE + 0x_0088; //  2  BIOS      Sound PWM Control
    pub const REG_WAVE_RAM: Addr = IO_BASE + 0x_0090; //              Channel 3 Wave Pattern RAM (2 banks!!)
    pub const REG_FIFO_A: Addr = IO_BASE + 0x_00A0; //  4    W        Channel A FIFO, Data 0-3
    pub const REG_FIFO_B: Addr = IO_BASE + 0x_00A4; //  4    W        Channel B FIFO, Data 0-3
                                                    // DMA Transfer Channels
    pub const REG_DMA0SAD: Addr = IO_BASE + 0x_00B0; //  4    W       DMA 0 Source Address
    pub const REG_DMA0DAD: Addr = IO_BASE + 0x_00B4; //  4    W       DMA 0 Destination Address
    pub const REG_DMA0CNT_L: Addr = IO_BASE + 0x_00B8; //  2    W     DMA 0 Word Count
    pub const REG_DMA0CNT_H: Addr = IO_BASE + 0x_00BA; //  2    R/W   DMA 0 Control
    pub const REG_DMA1SAD: Addr = IO_BASE + 0x_00BC; //  4    W       DMA 1 Source Address
    pub const REG_DMA1DAD: Addr = IO_BASE + 0x_00C0; //  4    W       DMA 1 Destination Address
    pub const REG_DMA1CNT_L: Addr = IO_BASE + 0x_00C4; //  2    W     DMA 1 Word Count
    pub const REG_DMA1CNT_H: Addr = IO_BASE + 0x_00C6; //  2    R/W   DMA 1 Control
    pub const REG_DMA2SAD: Addr = IO_BASE + 0x_00C8; //  4    W       DMA 2 Source Address
    pub const REG_DMA2DAD: Addr = IO_BASE + 0x_00CC; //  4    W       DMA 2 Destination Address
    pub const REG_DMA2CNT_L: Addr = IO_BASE + 0x_00D0; //  2    W     DMA 2 Word Count
    pub const REG_DMA2CNT_H: Addr = IO_BASE + 0x_00D2; //  2    R/W   DMA 2 Control
    pub const REG_DMA3SAD: Addr = IO_BASE + 0x_00D4; //  4    W       DMA 3 Source Address
    pub const REG_DMA3DAD: Addr = IO_BASE + 0x_00D8; //  4    W       DMA 3 Destination Address
    pub const REG_DMA3CNT_L: Addr = IO_BASE + 0x_00DC; //  2    W     DMA 3 Word Count
    pub const REG_DMA3CNT_H: Addr = IO_BASE + 0x_00DE; //  2    R/W   DMA 3 Control
                                                       // Timer Registers
    pub const REG_TM0CNT_L: Addr = IO_BASE + 0x_0100; //  2    R/W    Timer 0 Counter/Reload
    pub const REG_TM0CNT_H: Addr = IO_BASE + 0x_0102; //  2    R/W    Timer 0 Control
    pub const REG_TM1CNT_L: Addr = IO_BASE + 0x_0104; //  2    R/W    Timer 1 Counter/Reload
    pub const REG_TM1CNT_H: Addr = IO_BASE + 0x_0106; //  2    R/W    Timer 1 Control
    pub const REG_TM2CNT_L: Addr = IO_BASE + 0x_0108; //  2    R/W    Timer 2 Counter/Reload
    pub const REG_TM2CNT_H: Addr = IO_BASE + 0x_010A; //  2    R/W    Timer 2 Control
    pub const REG_TM3CNT_L: Addr = IO_BASE + 0x_010C; //  2    R/W    Timer 3 Counter/Reload
    pub const REG_TM3CNT_H: Addr = IO_BASE + 0x_010E; //  2    R/W    Timer 3 Control
                                                      // Serial Communication (1)
    pub const REG_SIODATA32: Addr = IO_BASE + 0x_0120; //  4    R/W   SIO Data (Normal-32bit Mode; shared with below)
    pub const REG_SIOMULTI0: Addr = IO_BASE + 0x_0120; //  2    R/W   SIO Data 0 (Parent)    (Multi-Player Mode)
    pub const REG_SIOMULTI1: Addr = IO_BASE + 0x_0122; //  2    R/W   SIO Data 1 (1st Child) (Multi-Player Mode)
    pub const REG_SIOMULTI2: Addr = IO_BASE + 0x_0124; //  2    R/W   SIO Data 2 (2nd Child) (Multi-Player Mode)
    pub const REG_SIOMULTI3: Addr = IO_BASE + 0x_0126; //  2    R/W   SIO Data 3 (3rd Child) (Multi-Player Mode)
    pub const REG_SIOCNT: Addr = IO_BASE + 0x_0128; //  2    R/W      SIO Control Register
    pub const REG_SIOMLT_SEND: Addr = IO_BASE + 0x_012A; //  2    R/W   SIO Data (Local of MultiPlayer; shared below)
    pub const REG_SIODATA8: Addr = IO_BASE + 0x_012A; //  2    R/W    SIO Data (Normal-8bit and UART Mode)
                                                      // Keypad Input
    pub const REG_KEYINPUT: Addr = IO_BASE + 0x_0130; //  2    R      Key Status
    pub const REG_KEYCNT: Addr = IO_BASE + 0x_0132; //  2    R/W      Key Interrupt Control
                                                    // Serial Communication (2)
    pub const REG_RCNT: Addr = IO_BASE + 0x_0134; //  2    R/W        SIO Mode Select/General Purpose Data
    pub const REG_IR: Addr = IO_BASE + 0x_0136; //  -    -            Ancient - Infrared Register (Prototypes only)
    pub const REG_JOYCNT: Addr = IO_BASE + 0x_0140; //  2    R/W      SIO JOY Bus Control
    pub const REG_JOY_RECV: Addr = IO_BASE + 0x_0150; //  4    R/W    SIO JOY Bus Receive Data
    pub const REG_JOY_TRANS: Addr = IO_BASE + 0x_0154; //  4    R/W   SIO JOY Bus Transmit Data
    pub const REG_JOYSTAT: Addr = IO_BASE + 0x_0158; //  2    R/?     SIO JOY Bus Receive Status
                                                     // Interrupt, Waitstate, and Power-Down Control
    pub const REG_IE: Addr = IO_BASE + 0x_0200; //  2    R/W          Interrupt Enable Register
    pub const REG_IF: Addr = IO_BASE + 0x_0202; //  2    R/W          Interrupt Request Flags / IRQ Acknowledge
    pub const REG_WAITCNT: Addr = IO_BASE + 0x_0204; //  2    R/W     Game Pak Waitstate Control
    pub const REG_IME: Addr = IO_BASE + 0x_0208; //  2    R/W         Interrupt Master Enable Register
    pub const REG_POSTFLG: Addr = IO_BASE + 0x_0300; //  1    R/W     Undocumented - Post Boot Flag
    pub const REG_HALTCNT: Addr = IO_BASE + 0x_0301; //  1    W       Undocumented - Power Down Control
}

use consts::*;

#[derive(Debug)]
pub struct IoRegs {
    mem: BoxedMemory,
    pub io: Rc<RefCell<IoDevices>>,
    pub keyinput: u16,
    pub post_boot_flag: bool,
    pub waitcnt: WaitControl, // TODO also implement 4000800
}

impl IoRegs {
    pub fn new(io: Rc<RefCell<IoDevices>>) -> IoRegs {
        IoRegs {
            mem: BoxedMemory::new(vec![0; 0x400].into_boxed_slice(), 0x3ff),
            io: io,
            post_boot_flag: false,
            keyinput: keypad::KEYINPUT_ALL_RELEASED,
            waitcnt: WaitControl(0),
        }
    }
}

impl Bus for IoRegs {
    fn read_32(&self, addr: Addr) -> u32 {
        (self.read_16(addr + 2) as u32) << 16 | (self.read_16(addr) as u32)
    }

    fn read_16(&self, addr: Addr) -> u16 {
        let io = self.io.borrow();
        match addr + IO_BASE {
            REG_DISPCNT => io.gpu.dispcnt.0,
            REG_DISPSTAT => io.gpu.dispstat.0,
            REG_VCOUNT => io.gpu.current_scanline as u16,
            REG_BG0CNT => io.gpu.bgcnt[0].0,
            REG_BG1CNT => io.gpu.bgcnt[1].0,
            REG_BG2CNT => io.gpu.bgcnt[2].0,
            REG_BG3CNT => io.gpu.bgcnt[3].0,
            REG_WIN0H => io.gpu.win0h,
            REG_WIN1H => io.gpu.win1h,
            REG_WIN0V => io.gpu.win0v,
            REG_WIN1V => io.gpu.win1v,
            REG_WININ => io.gpu.winin,
            REG_WINOUT => io.gpu.winout,
            REG_MOSAIC => io.gpu.mosaic,
            REG_BLDCNT => io.gpu.bldcnt,
            REG_BLDALPHA => io.gpu.bldalpha,
            REG_BLDY => io.gpu.bldy,

            REG_IME => io.intc.interrupt_master_enable as u16,
            REG_IE => io.intc.interrupt_enable.0 as u16,
            REG_IF => io.intc.interrupt_flags.0 as u16,

            REG_TM0CNT_L => io.timers[0].timer_data,
            REG_TM0CNT_H => io.timers[0].timer_ctl.0,
            REG_TM1CNT_L => io.timers[1].timer_data,
            REG_TM1CNT_H => io.timers[1].timer_ctl.0,
            REG_TM2CNT_L => io.timers[2].timer_data,
            REG_TM2CNT_H => io.timers[2].timer_ctl.0,
            REG_TM3CNT_L => io.timers[3].timer_data,
            REG_TM3CNT_H => io.timers[3].timer_ctl.0,

            REG_WAITCNT => self.waitcnt.0,

            REG_POSTFLG => self.post_boot_flag as u16,
            REG_HALTCNT => 0,
            REG_KEYINPUT => self.keyinput as u16,
            _ => {
                self.mem.read_16(addr)
            }
        }
    }

    fn read_8(&self, addr: Addr) -> u8 {
        self.read_16(addr) as u8
    }

    fn write_32(&mut self, addr: Addr, value: u32) {
        self.write_16(addr, (value & 0xffff) as u16);
        self.write_16(addr, (value >> 16) as u16);
    }

    fn write_16(&mut self, addr: Addr, value: u16) {
        let mut io = self.io.borrow_mut();
        match addr + IO_BASE {
            REG_DISPCNT => io.gpu.dispcnt.0 |= value,
            REG_DISPSTAT => io.gpu.dispstat.0 |= value,
            REG_BG0CNT => io.gpu.bgcnt[0].0 |= value,
            REG_BG1CNT => io.gpu.bgcnt[1].0 |= value,
            REG_BG2CNT => io.gpu.bgcnt[2].0 |= value,
            REG_BG3CNT => io.gpu.bgcnt[3].0 |= value,
            REG_BG0HOFS => io.gpu.bghofs[0] = value,
            REG_BG0VOFS => io.gpu.bgvofs[0] = value,
            REG_BG1HOFS => io.gpu.bghofs[1] = value,
            REG_BG1VOFS => io.gpu.bgvofs[1] = value,
            REG_BG2HOFS => io.gpu.bghofs[2] = value,
            REG_BG2VOFS => io.gpu.bgvofs[2] = value,
            REG_BG3HOFS => io.gpu.bghofs[3] = value,
            REG_BG3VOFS => io.gpu.bgvofs[3] = value,
            REG_WIN0H => io.gpu.win0h = value,
            REG_WIN1H => io.gpu.win1h = value,
            REG_WIN0V => io.gpu.win0v = value,
            REG_WIN1V => io.gpu.win1v = value,
            REG_WININ => io.gpu.winin = value,
            REG_WINOUT => io.gpu.winout = value,
            REG_MOSAIC => io.gpu.mosaic = value,
            REG_BLDCNT => io.gpu.bldcnt = value,
            REG_BLDALPHA => io.gpu.bldalpha = value,
            REG_BLDY => io.gpu.bldy = value,

            REG_IME => io.intc.interrupt_master_enable = value != 0,
            REG_IE => io.intc.interrupt_enable.0 = value,
            REG_IF => io.intc.interrupt_flags.0 &= !value,

            REG_TM0CNT_L => {
                io.timers[0].timer_data = value;
                io.timers[0].initial_data = value;
            }
            REG_TM0CNT_H => io.timers[0].timer_ctl.0 = value,

            REG_TM1CNT_L => {
                io.timers[1].timer_data = value;
                io.timers[1].initial_data = value;
            }
            REG_TM1CNT_H => io.timers[1].timer_ctl.0 = value,

            REG_TM2CNT_L => {
                io.timers[2].timer_data = value;
                io.timers[2].initial_data = value;
            }
            REG_TM2CNT_H => io.timers[2].timer_ctl.0 = value,

            REG_TM3CNT_L => {
                io.timers[3].timer_data = value;
                io.timers[3].initial_data = value;
            }
            REG_TM3CNT_H => io.timers[3].timer_ctl.0 = value,

            REG_WAITCNT => self.waitcnt.0 = value,

            REG_POSTFLG => self.post_boot_flag = value != 0,
            REG_HALTCNT => {}
            _ => {
                self.mem.write_16(addr, value);
            }
        }
    }

    fn write_8(&mut self, addr: Addr, value: u8) {
        let t = self.read_16(addr);
        self.write_16(addr, (t & 0xff) | ((value as u16) << 8));
    }
}

bitfield! {
    #[derive(Default, Copy, Clone, PartialEq)]
    pub struct WaitControl(u16);
    impl Debug;
    u16;
    sram_wait_control, _:      1, 0;
    pub ws0_first_access, _:       3, 2;
    pub ws0_second_access, _:      4, 4;
    pub ws1_first_access, _:       6, 5;
    pub ws1_second_access, _:      7, 7;
    pub ws2_first_access, _:       9, 8;
    pub ws2_second_access, _:      10, 10;
    #[allow(non_snake_case)]
    PHI_terminal_output, _:    12, 11;
    prefetch, _:               14;
}