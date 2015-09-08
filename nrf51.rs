// nRF51 reference description for radio MCU with ARM 32-bit Cortex-M0 Microcontroller at 16MHz CPU clock

use volatile_cell::VolatileCell;
use core::ops::Drop
ioregs! (POWER @ 0x40000000 = {
    0x078 => reg32 TASKS_CONSTLAT {}
    0x07C => reg32 TASKS_LOWPWR {}
    0x108 => reg32 EVENTS_POFWARN {}
    0x304 => reg32 INTENSET {
        2 => POFWARN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        2 => POFWARN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 RESETREAS {
        0 => RESETPIN: rw {
            0 => NotDetected,
            1 => Detected,
        }
        1 => DOG: rw {
            0 => NotDetected,
            1 => Detected,
        }
        2 => SREQ: rw {
            0 => NotDetected,
            1 => Detected,
        }
        3 => LOCKUP: rw {
            0 => NotDetected,
            1 => Detected,
        }
        16 => OFF: rw {
            0 => NotDetected,
            1 => Detected,
        }
        17 => LPCOMP: rw {
            0 => NotDetected,
            1 => Detected,
        }
        18 => DIF: rw {
            0 => NotDetected,
            1 => Detected,
        }
    }
    0x428 => reg32 RAMSTATUS {
        0 => RAMBLOCK0: ro {
            0 => Off,
            1 => On,
        }
        1 => RAMBLOCK1: ro {
            0 => Off,
            1 => On,
        }
        2 => RAMBLOCK2: ro {
            0 => Off,
            1 => On,
        }
        3 => RAMBLOCK3: ro {
            0 => Off,
            1 => On,
        }
    }
    0x500 => reg32 SYSTEMOFF {
        0 => SYSTEMOFF: wo {
            1 => Enter,
        }
    }
    0x510 => reg32 POFCON {
        0 => POF: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1..2 => THRESHOLD: rw {
            0x00 => V21,
            0x01 => V23,
            0x02 => V25,
            0x03 => V27,
        }
    }
    0x51C => reg32 GPREGRET {
        0..7 => GPREGRET: rw,
    }
    0x524 => reg32 RAMON {
        0 => ONRAM0: rw {
            0 => RAM0Off,
            1 => RAM0On,
        }
        1 => ONRAM1: rw {
            0 => RAM1Off,
            1 => RAM1On,
        }
        16 => OFFRAM0: rw {
            0 => RAM0Off,
            1 => RAM0On,
        }
        17 => OFFRAM1: rw {
            0 => RAM1Off,
            1 => RAM1On,
        }
    }
    0x544 => reg32 RESET {
        0 => RESET: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x554 => reg32 RAMONB {
        0 => ONRAM2: rw {
            0 => RAM2Off,
            1 => RAM2On,
        }
        1 => ONRAM3: rw {
            0 => RAM3Off,
            1 => RAM3On,
        }
        16 => OFFRAM2: rw {
            0 => RAM2Off,
            1 => RAM2On,
        }
        17 => OFFRAM3: rw {
            0 => RAM3Off,
            1 => RAM3On,
        }
    }
    0x578 => reg32 DCDCEN {
        0 => DCDCEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0xA08 => reg32 DCDCFORCE {
        0 => FORCEOFF: rw {
            0 => NoForce,
            1 => Force,
        }
        1 => FORCEON: rw {
            0 => NoForce,
            1 => Force,
        }
    }
});

ioregs! (CLOCK @ 0x40000000 = {
    0x000 => reg32 TASKS_HFCLKSTART {}
    0x004 => reg32 TASKS_HFCLKSTOP {}
    0x008 => reg32 TASKS_LFCLKSTART {}
    0x00C => reg32 TASKS_LFCLKSTOP {}
    0x010 => reg32 TASKS_CAL {}
    0x014 => reg32 TASKS_CTSTART {}
    0x018 => reg32 TASKS_CTSTOP {}
    0x100 => reg32 EVENTS_HFCLKSTARTED {}
    0x104 => reg32 EVENTS_LFCLKSTARTED {}
    0x10C => reg32 EVENTS_DONE {}
    0x110 => reg32 EVENTS_CTTO {}
    0x304 => reg32 INTENSET {
        0 => HFCLKSTARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => LFCLKSTARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => DONE: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CTTO: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => HFCLKSTARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => LFCLKSTARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => DONE: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CTTO: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x408 => reg32 HFCLKRUN {
        0 => STATUS: ro {
            0 => NotTriggered,
            1 => Triggered,
        }
    }
    0x40C => reg32 HFCLKSTAT {
        0 => SRC: ro {
            0 => RC,
            1 => Xtal,
        }
        16 => STATE: ro {
            0 => NotRunning,
            1 => Running,
        }
    }
    0x414 => reg32 LFCLKRUN {
        0 => STATUS: ro {
            0 => NotTriggered,
            1 => Triggered,
        }
    }
    0x418 => reg32 LFCLKSTAT {
        0..1 => SRC: ro {
            0 => RC,
            1 => Xtal,
            2 => Synth,
        }
        16 => STATE: ro {
            0 => NotRunning,
            1 => Running,
        }
    }
    0x41C => reg32 LFCLKSRCCOPY {
        0..1 => SRC: ro {
            0 => RC,
            1 => Xtal,
            2 => Synth,
        }
    }
    0x518 => reg32 LFCLKSRC {
        0..1 => SRC: rw {
            0 => RC,
            1 => Xtal,
            2 => Synth,
        }
    }
    0x538 => reg32 CTIV {
        0..6 => CTIV: rw,
    }
    0x550 => reg32 XTALFREQ {
        0..7 => XTALFREQ: rw {
            0xFF => 16MHz,
            0x00 => 32MHz,
        }
    }
});

ioregs! (MPU @ 0x40000000 = {
    0x528 => reg32 PERR0 {
        0 => POWER_CLOCK: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        1 => RADIO: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        2 => UART0: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        3 => SPI0_TWI0: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        4 => SPI1_TWI1: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        6 => GPIOTE: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        7 => ADC: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        8 => TIMER0: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        9 => TIMER1: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        10 => TIMER2: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        11 => RTC0: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        12 => TEMP: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        13 => RNG: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        14 => ECB: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        15 => CCM_AAR: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        16 => WDT: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        17 => RTC1: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        18 => QDEC: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        19 => LPCOMP: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        30 => NVMC: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
        31 => PPI: rw {
            1 => InRegion0,
            0 => InRegion1,
        }
    }
    0x52C => reg32 RLENR0 {}
    0x600 => reg32 PROTENSET0 {
        0 => PROTREG0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => PROTREG1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => PROTREG2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => PROTREG3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => PROTREG4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => PROTREG5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => PROTREG6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => PROTREG7: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => PROTREG8: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => PROTREG9: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => PROTREG10: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => PROTREG11: rw {
            0 => Disabled,
            1 => Enabled,
        }
        12 => PROTREG12: rw {
            0 => Disabled,
            1 => Enabled,
        }
        13 => PROTREG13: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => PROTREG14: rw {
            0 => Disabled,
            1 => Enabled,
        }
        15 => PROTREG15: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => PROTREG16: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => PROTREG17: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => PROTREG18: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => PROTREG19: rw {
            0 => Disabled,
            1 => Enabled,
        }
        20 => PROTREG20: rw {
            0 => Disabled,
            1 => Enabled,
        }
        21 => PROTREG21: rw {
            0 => Disabled,
            1 => Enabled,
        }
        22 => PROTREG22: rw {
            0 => Disabled,
            1 => Enabled,
        }
        23 => PROTREG23: rw {
            0 => Disabled,
            1 => Enabled,
        }
        24 => PROTREG24: rw {
            0 => Disabled,
            1 => Enabled,
        }
        25 => PROTREG25: rw {
            0 => Disabled,
            1 => Enabled,
        }
        26 => PROTREG26: rw {
            0 => Disabled,
            1 => Enabled,
        }
        27 => PROTREG27: rw {
            0 => Disabled,
            1 => Enabled,
        }
        28 => PROTREG28: rw {
            0 => Disabled,
            1 => Enabled,
        }
        29 => PROTREG29: rw {
            0 => Disabled,
            1 => Enabled,
        }
        30 => PROTREG30: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => PROTREG31: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x604 => reg32 PROTENSET1 {
        0 => PROTREG32: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => PROTREG33: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => PROTREG34: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => PROTREG35: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => PROTREG36: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => PROTREG37: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => PROTREG38: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => PROTREG39: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => PROTREG40: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => PROTREG41: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => PROTREG42: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => PROTREG43: rw {
            0 => Disabled,
            1 => Enabled,
        }
        12 => PROTREG44: rw {
            0 => Disabled,
            1 => Enabled,
        }
        13 => PROTREG45: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => PROTREG46: rw {
            0 => Disabled,
            1 => Enabled,
        }
        15 => PROTREG47: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => PROTREG48: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => PROTREG49: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => PROTREG50: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => PROTREG51: rw {
            0 => Disabled,
            1 => Enabled,
        }
        20 => PROTREG52: rw {
            0 => Disabled,
            1 => Enabled,
        }
        21 => PROTREG53: rw {
            0 => Disabled,
            1 => Enabled,
        }
        22 => PROTREG54: rw {
            0 => Disabled,
            1 => Enabled,
        }
        23 => PROTREG55: rw {
            0 => Disabled,
            1 => Enabled,
        }
        24 => PROTREG56: rw {
            0 => Disabled,
            1 => Enabled,
        }
        25 => PROTREG57: rw {
            0 => Disabled,
            1 => Enabled,
        }
        26 => PROTREG58: rw {
            0 => Disabled,
            1 => Enabled,
        }
        27 => PROTREG59: rw {
            0 => Disabled,
            1 => Enabled,
        }
        28 => PROTREG60: rw {
            0 => Disabled,
            1 => Enabled,
        }
        29 => PROTREG61: rw {
            0 => Disabled,
            1 => Enabled,
        }
        30 => PROTREG62: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => PROTREG63: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x608 => reg32 DISABLEINDEBUG {
        0 => DISABLEINDEBUG: rw {
            0 => Enabled,
            1 => Disabled,
        }
    }
    0x60C => reg32 PROTBLOCKSIZE {
        0..1 => PROTBLOCKSIZE: rw {
            0 => 4k,
        }
    }
});

ioregs! (AMLI @ 0x40000000 = {
    0xE00 => group RAMPRI[0] {
        0x000 => reg32 CPU0 {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
        0x004 => reg32 SPIS1 {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
        0x008 => reg32 RADIO {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
        0x00C => reg32 ECB {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
        0x010 => reg32 CCM {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
        0x014 => reg32 AAR {
            0..3 => RAM0: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            4..7 => RAM1: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            8..11 => RAM2: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            12..15 => RAM3: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            16..19 => RAM4: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            20..23 => RAM5: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            24..27 => RAM6: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
            28..31 => RAM7: rw {
                0x0 => Pri0,
                0x2 => Pri2,
                0x4 => Pri4,
                0x6 => Pri6,
                0x8 => Pri8,
                0xA => Pri10,
                0xC => Pri12,
                0xE => Pri14,
            }
        }
    }
});

ioregs! (RADIO @ 0x40001000 = {
    0x000 => reg32 TASKS_TXEN {}
    0x004 => reg32 TASKS_RXEN {}
    0x008 => reg32 TASKS_START {}
    0x00C => reg32 TASKS_STOP {}
    0x010 => reg32 TASKS_DISABLE {}
    0x014 => reg32 TASKS_RSSISTART {}
    0x018 => reg32 TASKS_RSSISTOP {}
    0x01C => reg32 TASKS_BCSTART {}
    0x020 => reg32 TASKS_BCSTOP {}
    0x100 => reg32 EVENTS_READY {}
    0x104 => reg32 EVENTS_ADDRESS {}
    0x108 => reg32 EVENTS_PAYLOAD {}
    0x10C => reg32 EVENTS_END {}
    0x110 => reg32 EVENTS_DISABLED {}
    0x114 => reg32 EVENTS_DEVMATCH {}
    0x118 => reg32 EVENTS_DEVMISS {}
    0x11C => reg32 EVENTS_RSSIEND {}
    0x128 => reg32 EVENTS_BCMATCH {}
    0x200 => reg32 SHORTS {
        0 => READY_START: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => END_DISABLE: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => DISABLED_TXEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => DISABLED_RXEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => ADDRESS_RSSISTART: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => END_START: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => ADDRESS_BCSTART: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => DISABLED_RSSISTOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ADDRESS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => PAYLOAD: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => DISABLED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => DEVMATCH: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => DEVMISS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => RSSIEND: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => BCMATCH: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ADDRESS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => PAYLOAD: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => DISABLED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => DEVMATCH: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => DEVMISS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => RSSIEND: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => BCMATCH: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 CRCSTATUS {
        0 => CRCSTATUS: ro {
            0 => CRCError,
            1 => CRCOk,
        }
    }
    0x408 => reg32 RXMATCH {
        0..2 => RXMATCH: ro,
    }
    0x40C => reg32 RXCRC {
        0..23 => RXCRC: ro,
    }
    0x410 => reg32 DAI {
        0..2 => DAI: ro,
    }
    0x504 => reg32 PACKETPTR {}
    0x508 => reg32 FREQUENCY {
        0..6 => FREQUENCY: rw,
    }
    0x50C => reg32 TXPOWER {
        0..7 => TXPOWER: rw {
            0x04 => Pos4dBm,
            0x00 => 0dBm,
            0xFC => Neg4dBm,
            0xF8 => Neg8dBm,
            0xF4 => Neg12dBm,
            0xF0 => Neg16dBm,
            0xEC => Neg20dBm,
            0xD8 => Neg30dBm,
        }
    }
    0x510 => reg32 MODE {
        0..1 => MODE: rw {
            0x00 => Nrf_1Mbit,
            0x01 => Nrf_2Mbit,
            0x02 => Nrf_250Kbit,
            0x03 => Ble_1Mbit,
        }
    }
    0x514 => reg32 PCNF0 {
        0..3 => LFLEN: rw,
        8 => S0LEN: rw,
        16..19 => S1LEN: rw,
    }
    0x518 => reg32 PCNF1 {
        0..7 => MAXLEN: rw,
        8..15 => STATLEN: rw,
        16..18 => BALEN: rw,
        24 => ENDIAN: rw {
            0 => Little,
            1 => Big,
        }
        25 => WHITEEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x51C => reg32 BASE0 {}
    0x520 => reg32 BASE1 {}
    0x524 => reg32 PREFIX0 {
        0..7 => AP0: rw,
        8..15 => AP1: rw,
        16..23 => AP2: rw,
        24..31 => AP3: rw,
    }
    0x528 => reg32 PREFIX1 {
        0..7 => AP4: rw,
        8..15 => AP5: rw,
        16..23 => AP6: rw,
        24..31 => AP7: rw,
    }
    0x52C => reg32 TXADDRESS {
        0..2 => TXADDRESS: rw,
    }
    0x530 => reg32 RXADDRESSES {
        0 => ADDR0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ADDR1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ADDR2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => ADDR3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => ADDR4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => ADDR5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => ADDR6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => ADDR7: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x534 => reg32 CRCCNF {
        0..1 => LEN: rw {
            0 => Disabled,
            1 => One,
            2 => Two,
            3 => Three,
        }
        8 => SKIPADDR: rw {
            0 => Include,
            1 => Skip,
        }
    }
    0x538 => reg32 CRCPOLY {
        0..23 => CRCPOLY: rw,
    }
    0x53C => reg32 CRCINIT {
        0..23 => CRCINIT: rw,
    }
    0x540 => reg32 TEST {
        0 => CONSTCARRIER: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => PLLLOCK: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x544 => reg32 TIFS {
        0..7 => TIFS: rw,
    }
    0x548 => reg32 RSSISAMPLE {
        0..6 => RSSISAMPLE: ro,
    }
    0x550 => reg32 STATE {
        0..3 => STATE: ro {
            0x00 => Disabled,
            0x01 => RxRu,
            0x02 => RxIdle,
            0x03 => Rx,
            0x04 => RxDisable,
            0x09 => TxRu,
            0x0A => TxIdle,
            0x0B => Tx,
            0x0C => TxDisable,
        }
    }
    0x554 => reg32 DATAWHITEIV {
        0..6 => DATAWHITEIV: rw,
    }
    0x560 => reg32 BCC {}
    0x600 => group DAB[8] {
        0x0 => reg32 DAB {}
    }
    0x620 => group DAP[8] {
        0x0 => reg32 DAP {
            0..15 => DAP: rw,
        }
    }
    0x640 => reg32 DACNF {
        0 => ENA0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ENA1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ENA2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => ENA3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => ENA4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => ENA5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => ENA6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => ENA7: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => TXADD0: rw,
        9 => TXADD1: rw,
        10 => TXADD2: rw,
        11 => TXADD3: rw,
        12 => TXADD4: rw,
        13 => TXADD5: rw,
        14 => TXADD6: rw,
        15 => TXADD7: rw,
    }
    0x724 => reg32 OVERRIDE0 {
        0..31 => OVERRIDE0: rw,
    }
    0x728 => reg32 OVERRIDE1 {
        0..31 => OVERRIDE1: rw,
    }
    0x72C => reg32 OVERRIDE2 {
        0..31 => OVERRIDE2: rw,
    }
    0x730 => reg32 OVERRIDE3 {
        0..31 => OVERRIDE3: rw,
    }
    0x734 => reg32 OVERRIDE4 {
        0..27 => OVERRIDE4: rw,
        31 => ENABLE: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (UART0 @ 0x40002000 = {
    0x000 => reg32 TASKS_STARTRX {}
    0x004 => reg32 TASKS_STOPRX {}
    0x008 => reg32 TASKS_STARTTX {}
    0x00C => reg32 TASKS_STOPTX {}
    0x01C => reg32 TASKS_SUSPEND {}
    0x100 => reg32 EVENTS_CTS {}
    0x104 => reg32 EVENTS_NCTS {}
    0x108 => reg32 EVENTS_RXDRDY {}
    0x11C => reg32 EVENTS_TXDRDY {}
    0x124 => reg32 EVENTS_ERROR {}
    0x144 => reg32 EVENTS_RXTO {}
    0x200 => reg32 SHORTS {
        3 => CTS_STARTRX: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => NCTS_STOPRX: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => CTS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => NCTS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => RXTO: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => CTS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => NCTS: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => RXTO: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x480 => reg32 ERRORSRC {
        0 => OVERRUN: rw {
            0 => NotPresent,
            1 => Present,
        }
        1 => PARITY: rw {
            0 => NotPresent,
            1 => Present,
        }
        2 => FRAMING: rw {
            0 => NotPresent,
            1 => Present,
        }
        3 => BREAK: rw {
            0 => NotPresent,
            1 => Present,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x04 => Enabled,
        }
    }
    0x508 => reg32 PSELRTS {}
    0x50C => reg32 PSELTXD {}
    0x510 => reg32 PSELCTS {}
    0x514 => reg32 PSELRXD {}
    0x518 => reg32 RXD {
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD {
        0..7 => TXD: wo,
    }
    0x524 => reg32 BAUDRATE {
        0..31 => BAUDRATE: rw {
            0x0004F000 => Baud1200,
            0x0009D000 => Baud2400,
            0x0013B000 => Baud4800,
            0x00275000 => Baud9600,
            0x003B0000 => Baud14400,
            0x004EA000 => Baud19200,
            0x0075F000 => Baud28800,
            0x009D5000 => Baud38400,
            0x00EBF000 => Baud57600,
            0x013A9000 => Baud76800,
            0x01D7E000 => Baud115200,
            0x03AFB000 => Baud230400,
            0x04000000 => Baud250000,
            0x075F7000 => Baud460800,
            0x0EBED000 => Baud921600,
            0x10000000 => Baud1M,
        }
    }
    0x56C => reg32 CONFIG {
        0 => HWFC: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1..3 => PARITY: rw {
            0 => Excluded,
            7 => Included,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (SPI0 @ 0x40003000 = {
    0x108 => reg32 EVENTS_READY {}
    0x304 => reg32 INTENSET {
        2 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        2 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x01 => Enabled,
        }
    }
    0x508 => reg32 PSELSCK {}
    0x50C => reg32 PSELMOSI {}
    0x510 => reg32 PSELMISO {}
    0x518 => reg32 RXD {
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD {
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY {
        0..31 => FREQUENCY: rw {
            0x02000000 => K125,
            0x04000000 => K250,
            0x08000000 => K500,
            0x10000000 => M1,
            0x20000000 => M2,
            0x40000000 => M4,
            0x80000000 => M8,
        }
    }
    0x554 => reg32 CONFIG {
        0 => ORDER: rw {
            0 => MsbFirst,
            1 => LsbFirst,
        }
        1 => CPHA: rw {
            0 => Leading,
            1 => Trailing,
        }
        2 => CPOL: rw {
            0 => ActiveHigh,
            1 => ActiveLow,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TWI0 @ 0x40003000 = {
    0x000 => reg32 TASKS_STARTRX {}
    0x008 => reg32 TASKS_STARTTX {}
    0x014 => reg32 TASKS_STOP {}
    0x01C => reg32 TASKS_SUSPEND {}
    0x020 => reg32 TASKS_RESUME {}
    0x104 => reg32 EVENTS_STOPPED {}
    0x108 => reg32 EVENTS_RXDREADY {}
    0x11C => reg32 EVENTS_TXDSENT {}
    0x124 => reg32 EVENTS_ERROR {}
    0x138 => reg32 EVENTS_BB {}
    0x148 => reg32 EVENTS_SUSPENDED {}
    0x200 => reg32 SHORTS {
        0 => BB_SUSPEND: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => BB_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDREADY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDSENT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => BB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => SUSPENDED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDREADY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDSENT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => BB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => SUSPENDED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x4C4 => reg32 ERRORSRC {
        0 => OVERRUN: rw {
            0 => NotPresent,
            1 => Present,
        }
        1 => ANACK: rw {
            0 => NotPresent,
            1 => Present,
        }
        2 => DNACK: rw {
            0 => NotPresent,
            1 => Present,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x05 => Enabled,
        }
    }
    0x508 => reg32 PSELSCL {}
    0x50C => reg32 PSELSDA {}
    0x518 => reg32 RXD {
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD {
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY {
        0..31 => FREQUENCY: rw {
            0x01980000 => K100,
            0x04000000 => K250,
            0x06680000 => K400,
        }
    }
    0x588 => reg32 ADDRESS {
        0..6 => ADDRESS: rw,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (SPI1 @ 0x40004000 = {
    0x108 => reg32 EVENTS_READY {}
    0x304 => reg32 INTENSET {
        2 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        2 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x01 => Enabled,
        }
    }
    0x508 => reg32 PSELSCK {}
    0x50C => reg32 PSELMOSI {}
    0x510 => reg32 PSELMISO {}
    0x518 => reg32 RXD {
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD {
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY {
        0..31 => FREQUENCY: rw {
            0x02000000 => K125,
            0x04000000 => K250,
            0x08000000 => K500,
            0x10000000 => M1,
            0x20000000 => M2,
            0x40000000 => M4,
            0x80000000 => M8,
        }
    }
    0x554 => reg32 CONFIG {
        0 => ORDER: rw {
            0 => MsbFirst,
            1 => LsbFirst,
        }
        1 => CPHA: rw {
            0 => Leading,
            1 => Trailing,
        }
        2 => CPOL: rw {
            0 => ActiveHigh,
            1 => ActiveLow,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TWI1 @ 0x40004000 = {
    0x000 => reg32 TASKS_STARTRX {}
    0x008 => reg32 TASKS_STARTTX {}
    0x014 => reg32 TASKS_STOP {}
    0x01C => reg32 TASKS_SUSPEND {}
    0x020 => reg32 TASKS_RESUME {}
    0x104 => reg32 EVENTS_STOPPED {}
    0x108 => reg32 EVENTS_RXDREADY {}
    0x11C => reg32 EVENTS_TXDSENT {}
    0x124 => reg32 EVENTS_ERROR {}
    0x138 => reg32 EVENTS_BB {}
    0x148 => reg32 EVENTS_SUSPENDED {}
    0x200 => reg32 SHORTS {
        0 => BB_SUSPEND: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => BB_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDREADY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDSENT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => BB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => SUSPENDED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RXDREADY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => TXDSENT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => BB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => SUSPENDED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x4C4 => reg32 ERRORSRC {
        0 => OVERRUN: rw {
            0 => NotPresent,
            1 => Present,
        }
        1 => ANACK: rw {
            0 => NotPresent,
            1 => Present,
        }
        2 => DNACK: rw {
            0 => NotPresent,
            1 => Present,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x05 => Enabled,
        }
    }
    0x508 => reg32 PSELSCL {}
    0x50C => reg32 PSELSDA {}
    0x518 => reg32 RXD {
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD {
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY {
        0..31 => FREQUENCY: rw {
            0x01980000 => K100,
            0x04000000 => K250,
            0x06680000 => K400,
        }
    }
    0x588 => reg32 ADDRESS {
        0..6 => ADDRESS: rw,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (SPIS1 @ 0x40004000 = {
    0x024 => reg32 TASKS_ACQUIRE {}
    0x028 => reg32 TASKS_RELEASE {}
    0x104 => reg32 EVENTS_END {}
    0x128 => reg32 EVENTS_ACQUIRED {}
    0x200 => reg32 SHORTS {
        2 => END_ACQUIRE: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        1 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => ACQUIRED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        1 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => ACQUIRED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 SEMSTAT {
        0..1 => SEMSTAT: ro {
            0x00 => Free,
            0x01 => CPU,
            0x02 => SPIS,
            0x03 => CPUPending,
        }
    }
    0x440 => reg32 STATUS {
        0 => OVERREAD: rw {
            0 => NotPresent,
            1 => Present,
        }
        1 => OVERFLOW: rw {
            0 => NotPresent,
            1 => Present,
        }
    }
    0x500 => reg32 ENABLE {
        0..2 => ENABLE: rw {
            0x00 => Disabled,
            0x02 => Enabled,
        }
    }
    0x508 => reg32 PSELSCK {}
    0x50C => reg32 PSELMISO {}
    0x510 => reg32 PSELMOSI {}
    0x514 => reg32 PSELCSN {}
    0x534 => reg32 RXDPTR {}
    0x538 => reg32 MAXRX {
        0..7 => MAXRX: rw,
    }
    0x53C => reg32 AMOUNTRX {
        0..7 => AMOUNTRX: ro,
    }
    0x544 => reg32 TXDPTR {}
    0x548 => reg32 MAXTX {
        0..7 => MAXTX: rw,
    }
    0x54C => reg32 AMOUNTTX {
        0..7 => AMOUNTTX: ro,
    }
    0x554 => reg32 CONFIG {
        0 => ORDER: rw {
            0 => MsbFirst,
            1 => LsbFirst,
        }
        1 => CPHA: rw {
            0 => Leading,
            1 => Trailing,
        }
        2 => CPOL: rw {
            0 => ActiveHigh,
            1 => ActiveLow,
        }
    }
    0x55C => reg32 DEF {
        0..7 => DEF: rw,
    }
    0x5C0 => reg32 ORC {
        0..7 => ORC: rw,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (SPIM1 @ 0x40004000 = {
    0x010 => reg32 TASKS_START {}
    0x014 => reg32 TASKS_STOP {}
    0x01C => reg32 TASKS_SUSPEND {}
    0x020 => reg32 TASKS_RESUME {}
    0x104 => reg32 EVENTS_STOPPED {}
    0x110 => reg32 EVENTS_ENDRX {}
    0x120 => reg32 EVENTS_ENDTX {}
    0x14C => reg32 EVENTS_STARTED {}
    0x304 => reg32 INTENSET {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => ENDRX: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => ENDTX: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => STARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        1 => STOPPED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => ENDRX: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => ENDTX: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => STARTED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x500 => reg32 ENABLE {
        0..3 => ENABLE: rw {
            0x00 => Disabled,
            0x07 => Enabled,
        }
    }
    0x508 => group PSEL[0] {
        0x0 => reg32 SCK {}
        0x4 => reg32 MOSI {}
        0x8 => reg32 MISO {}
    }
    0x524 => reg32 FREQUENCY {
        0..31 => FREQUENCY: rw {
            0x02000000 => K125,
            0x04000000 => K250,
            0x08000000 => K500,
            0x10000000 => M1,
            0x20000000 => M2,
            0x40000000 => M4,
            0x80000000 => M8,
        }
    }
    0x534 => group RXD[0] {
        0x0 => reg32 PTR {
            0..31 => PTR: rw,
        }
        0x4 => reg32 MAXCNT {
            0..7 => MAXCNT: rw,
        }
        0x8 => reg32 AMOUNT {
            0..7 => AMOUNT: ro,
        }
    }
    0x544 => group TXD[0] {
        0x0 => reg32 PTR {
            0..31 => PTR: rw,
        }
        0x4 => reg32 MAXCNT {
            0..7 => MAXCNT: rw,
        }
        0x8 => reg32 AMOUNT {
            0..7 => AMOUNT: ro,
        }
    }
    0x554 => reg32 CONFIG {
        0 => ORDER: rw {
            0 => MsbFirst,
            1 => LsbFirst,
        }
        1 => CPHA: rw {
            0 => Leading,
            1 => Trailing,
        }
        2 => CPOL: rw {
            0 => ActiveHigh,
            1 => ActiveLow,
        }
    }
    0x5C0 => reg32 ORC {
        0..7 => ORC: rw,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (GPIOTE @ 0x40006000 = {
    0x000 => group TASKS_OUT[4] {
        0x0 => reg32 TASKS_OUT {}
    }
    0x100 => group EVENTS_IN[4] {
        0x0 => reg32 EVENTS_IN {}
    }
    0x17C => reg32 EVENTS_PORT {}
    0x304 => reg32 INTENSET {
        0 => IN0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => IN1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => IN2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => IN3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => PORT: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => IN0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => IN1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => IN2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => IN3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => PORT: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x510 => group CONFIG[4] {
        0x0 => reg32 CONFIG {
            0..1 => MODE: rw {
                0x00 => Disabled,
                0x01 => Event,
                0x03 => Task,
            }
            8..12 => PSEL: rw,
            16..17 => POLARITY: rw {
                0x00 => None,
                0x01 => LoToHi,
                0x02 => HiToLo,
                0x03 => Toggle,
            }
            20 => OUTINIT: rw {
                0 => Low,
                1 => High,
            }
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (ADC @ 0x40007000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x100 => reg32 EVENTS_END {}
    0x304 => reg32 INTENSET {
        0 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 BUSY {
        0 => BUSY: ro {
            0 => Ready,
            1 => Busy,
        }
    }
    0x500 => reg32 ENABLE {
        0..1 => ENABLE: rw {
            0x00 => Disabled,
            0x01 => Enabled,
        }
    }
    0x504 => reg32 CONFIG {
        0..1 => RES: rw {
            0x00 => 8bit,
            0x01 => 9bit,
            0x02 => 10bit,
        }
        2..4 => INPSEL: rw {
            0x00 => AnalogInputNoPrescaling,
            0x01 => AnalogInputTwoThirdsPrescaling,
            0x02 => AnalogInputOneThirdPrescaling,
            0x05 => SupplyTwoThirdsPrescaling,
            0x06 => SupplyOneThirdPrescaling,
        }
        5..6 => REFSEL: rw {
            0x00 => VBG,
            0x01 => External,
            0x02 => SupplyOneHalfPrescaling,
            0x03 => SupplyOneThirdPrescaling,
        }
        8..15 => PSEL: rw {
            0 => Disabled,
            1 => AnalogInput0,
            2 => AnalogInput1,
            4 => AnalogInput2,
            8 => AnalogInput3,
            16 => AnalogInput4,
            32 => AnalogInput5,
            64 => AnalogInput6,
            128 => AnalogInput7,
        }
        16..17 => EXTREFSEL: rw {
            0 => None,
            1 => AnalogReference0,
            2 => AnalogReference1,
        }
    }
    0x508 => reg32 RESULT {
        0..9 => RESULT: ro,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TIMER0 @ 0x40008000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_COUNT {}
    0x00C => reg32 TASKS_CLEAR {}
    0x010 => reg32 TASKS_SHUTDOWN {}
    0x040 => group TASKS_CAPTURE[4] {
        0x0 => reg32 TASKS_CAPTURE {}
    }
    0x140 => group EVENTS_COMPARE[4] {
        0x0 => reg32 EVENTS_COMPARE {}
    }
    0x200 => reg32 SHORTS {
        0 => COMPARE0_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => COMPARE1_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => COMPARE2_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => COMPARE3_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => COMPARE0_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => COMPARE1_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => COMPARE2_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => COMPARE3_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 MODE {
        0 => MODE: rw {
            1 => Counter,
            0 => Timer,
        }
    }
    0x508 => reg32 BITMODE {
        0..1 => BITMODE: rw {
            0x00 => 16Bit,
            0x01 => 08Bit,
            0x02 => 24Bit,
            0x03 => 32Bit,
        }
    }
    0x510 => reg32 PRESCALER {
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] {
        0x0 => reg32 CC {}
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TIMER1 @ 0x40009000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_COUNT {}
    0x00C => reg32 TASKS_CLEAR {}
    0x010 => reg32 TASKS_SHUTDOWN {}
    0x040 => group TASKS_CAPTURE[4] {
        0x0 => reg32 TASKS_CAPTURE {}
    }
    0x140 => group EVENTS_COMPARE[4] {
        0x0 => reg32 EVENTS_COMPARE {}
    }
    0x200 => reg32 SHORTS {
        0 => COMPARE0_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => COMPARE1_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => COMPARE2_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => COMPARE3_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => COMPARE0_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => COMPARE1_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => COMPARE2_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => COMPARE3_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 MODE {
        0 => MODE: rw {
            1 => Counter,
            0 => Timer,
        }
    }
    0x508 => reg32 BITMODE {
        0..1 => BITMODE: rw {
            0x00 => 16Bit,
            0x01 => 08Bit,
            0x02 => 24Bit,
            0x03 => 32Bit,
        }
    }
    0x510 => reg32 PRESCALER {
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] {
        0x0 => reg32 CC {}
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TIMER2 @ 0x4000A000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_COUNT {}
    0x00C => reg32 TASKS_CLEAR {}
    0x010 => reg32 TASKS_SHUTDOWN {}
    0x040 => group TASKS_CAPTURE[4] {
        0x0 => reg32 TASKS_CAPTURE {}
    }
    0x140 => group EVENTS_COMPARE[4] {
        0x0 => reg32 EVENTS_COMPARE {}
    }
    0x200 => reg32 SHORTS {
        0 => COMPARE0_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => COMPARE1_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => COMPARE2_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => COMPARE3_CLEAR: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => COMPARE0_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => COMPARE1_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => COMPARE2_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => COMPARE3_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 MODE {
        0 => MODE: rw {
            1 => Counter,
            0 => Timer,
        }
    }
    0x508 => reg32 BITMODE {
        0..1 => BITMODE: rw {
            0x00 => 16Bit,
            0x01 => 08Bit,
            0x02 => 24Bit,
            0x03 => 32Bit,
        }
    }
    0x510 => reg32 PRESCALER {
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] {
        0x0 => reg32 CC {}
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (RTC0 @ 0x4000B000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_CLEAR {}
    0x00C => reg32 TASKS_TRIGOVRFLW {}
    0x100 => reg32 EVENTS_TICK {}
    0x104 => reg32 EVENTS_OVRFLW {}
    0x140 => group EVENTS_COMPARE[4] {
        0x0 => reg32 EVENTS_COMPARE {}
    }
    0x304 => reg32 INTENSET {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x340 => reg32 EVTEN {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x344 => reg32 EVTENSET {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x348 => reg32 EVTENCLR {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 COUNTER {
        0..23 => COUNTER: ro,
    }
    0x508 => reg32 PRESCALER {
        0..11 => PRESCALER: rw,
    }
    0x540 => group CC[4] {
        0x0 => reg32 CC {
            0..23 => COMPARE: rw,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (TEMP @ 0x4000C000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x100 => reg32 EVENTS_DATARDY {}
    0x304 => reg32 INTENSET {
        0 => DATARDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => DATARDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x508 => reg32 TEMP {}
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (RNG @ 0x4000D000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x100 => reg32 EVENTS_VALRDY {}
    0x200 => reg32 SHORTS {
        0 => VALRDY_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => VALRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => VALRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 CONFIG {
        0 => DERCEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x508 => reg32 VALUE {
        0..7 => VALUE: ro,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (ECB @ 0x4000E000 = {
    0x000 => reg32 TASKS_STARTECB {}
    0x004 => reg32 TASKS_STOPECB {}
    0x100 => reg32 EVENTS_ENDECB {}
    0x104 => reg32 EVENTS_ERRORECB {}
    0x304 => reg32 INTENSET {
        0 => ENDECB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ERRORECB: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => ENDECB: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ERRORECB: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 ECBDATAPTR {}
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (AAR @ 0x4000F000 = {
    0x000 => reg32 TASKS_START {}
    0x008 => reg32 TASKS_STOP {}
    0x100 => reg32 EVENTS_END {}
    0x104 => reg32 EVENTS_RESOLVED {}
    0x108 => reg32 EVENTS_NOTRESOLVED {}
    0x304 => reg32 INTENSET {
        0 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => RESOLVED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => NOTRESOLVED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => END: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => RESOLVED: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => NOTRESOLVED: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 STATUS {
        0..3 => STATUS: ro,
    }
    0x500 => reg32 ENABLE {
        0..1 => ENABLE: rw {
            0x00 => Disabled,
            0x03 => Enabled,
        }
    }
    0x504 => reg32 NIRK {
        0..4 => NIRK: rw,
    }
    0x508 => reg32 IRKPTR {}
    0x510 => reg32 ADDRPTR {}
    0x514 => reg32 SCRATCHPTR {}
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (CCM @ 0x4000F000 = {
    0x000 => reg32 TASKS_KSGEN {}
    0x004 => reg32 TASKS_CRYPT {}
    0x008 => reg32 TASKS_STOP {}
    0x100 => reg32 EVENTS_ENDKSGEN {}
    0x104 => reg32 EVENTS_ENDCRYPT {}
    0x108 => reg32 EVENTS_ERROR {}
    0x200 => reg32 SHORTS {
        0 => ENDKSGEN_CRYPT: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => ENDKSGEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ENDCRYPT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => ENDKSGEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => ENDCRYPT: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ERROR: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 MICSTATUS {
        0 => MICSTATUS: ro {
            0 => CheckFailed,
            1 => CheckPassed,
        }
    }
    0x500 => reg32 ENABLE {
        0..1 => ENABLE: rw {
            0x00 => Disabled,
            0x02 => Enabled,
        }
    }
    0x504 => reg32 MODE {
        0 => MODE: rw {
            0 => Encryption,
            1 => Decryption,
        }
    }
    0x508 => reg32 CNFPTR {}
    0x50C => reg32 INPTR {}
    0x510 => reg32 OUTPTR {}
    0x514 => reg32 SCRATCHPTR {}
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (WDT @ 0x40010000 = {
    0x000 => reg32 TASKS_START {}
    0x100 => reg32 EVENTS_TIMEOUT {}
    0x304 => reg32 INTENSET {
        0 => TIMEOUT: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => TIMEOUT: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 RUNSTATUS {
        0 => RUNSTATUS: ro {
            0 => NotRunning,
            1 => Running,
        }
    }
    0x404 => reg32 REQSTATUS {
        0 => RR0: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        1 => RR1: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        2 => RR2: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        3 => RR3: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        4 => RR4: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        5 => RR5: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        6 => RR6: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
        7 => RR7: ro {
            0 => DisabledOrRequested,
            1 => EnabledAndUnrequested,
        }
    }
    0x504 => reg32 CRV {}
    0x508 => reg32 RREN {
        0 => RR0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => RR1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => RR2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => RR3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => RR4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => RR5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => RR6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => RR7: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x50C => reg32 CONFIG {
        0 => SLEEP: rw {
            0 => Pause,
            1 => Run,
        }
        3 => HALT: rw {
            0 => Pause,
            1 => Run,
        }
    }
    0x600 => group RR[8] {
        0x0 => reg32 RR {
            0..31 => RR: wo {
                0x6E524635 => Reload,
            }
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (RTC1 @ 0x40011000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_CLEAR {}
    0x00C => reg32 TASKS_TRIGOVRFLW {}
    0x100 => reg32 EVENTS_TICK {}
    0x104 => reg32 EVENTS_OVRFLW {}
    0x140 => group EVENTS_COMPARE[4] {
        0x0 => reg32 EVENTS_COMPARE {}
    }
    0x304 => reg32 INTENSET {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x340 => reg32 EVTEN {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x344 => reg32 EVTENSET {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x348 => reg32 EVTENCLR {
        0 => TICK: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => OVRFLW: rw {
            0 => Disabled,
            1 => Enabled,
        }
        16 => COMPARE0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        17 => COMPARE1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        18 => COMPARE2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        19 => COMPARE3: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 COUNTER {
        0..23 => COUNTER: ro,
    }
    0x508 => reg32 PRESCALER {
        0..11 => PRESCALER: rw,
    }
    0x540 => group CC[4] {
        0x0 => reg32 CC {
            0..23 => COMPARE: rw,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (QDEC @ 0x40012000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_READCLRACC {}
    0x100 => reg32 EVENTS_SAMPLERDY {}
    0x104 => reg32 EVENTS_REPORTRDY {}
    0x108 => reg32 EVENTS_ACCOF {}
    0x200 => reg32 SHORTS {
        0 => REPORTRDY_READCLRACC: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => SAMPLERDY_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => SAMPLERDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => REPORTRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ACCOF: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => SAMPLERDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => REPORTRDY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => ACCOF: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x500 => reg32 ENABLE {
        0 => ENABLE: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 LEDPOL {
        0 => LEDPOL: rw {
            0 => ActiveLow,
            1 => ActiveHigh,
        }
    }
    0x508 => reg32 SAMPLEPER {
        0..2 => SAMPLEPER: rw {
            0x00 => 128us,
            0x01 => 256us,
            0x02 => 512us,
            0x03 => 1024us,
            0x04 => 2048us,
            0x05 => 4096us,
            0x06 => 8192us,
            0x07 => 16384us,
        }
    }
    0x50C => reg32 SAMPLE {
        0..31 => SAMPLE: ro,
    }
    0x510 => reg32 REPORTPER {
        0..2 => REPORTPER: rw {
            0x00 => 10Smpl,
            0x01 => 40Smpl,
            0x02 => 80Smpl,
            0x03 => 120Smpl,
            0x04 => 160Smpl,
            0x05 => 200Smpl,
            0x06 => 240Smpl,
            0x07 => 280Smpl,
        }
    }
    0x514 => reg32 ACC {}
    0x518 => reg32 ACCREAD {}
    0x51C => reg32 PSELLED {}
    0x520 => reg32 PSELA {}
    0x524 => reg32 PSELB {}
    0x528 => reg32 DBFEN {
        0 => DBFEN: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x540 => reg32 LEDPRE {
        0..8 => LEDPRE: rw,
    }
    0x544 => reg32 ACCDBL {
        0..3 => ACCDBL: ro,
    }
    0x548 => reg32 ACCDBLREAD {
        0..3 => ACCDBLREAD: ro,
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (LPCOMP @ 0x40013000 = {
    0x000 => reg32 TASKS_START {}
    0x004 => reg32 TASKS_STOP {}
    0x008 => reg32 TASKS_SAMPLE {}
    0x100 => reg32 EVENTS_READY {}
    0x104 => reg32 EVENTS_DOWN {}
    0x108 => reg32 EVENTS_UP {}
    0x10C => reg32 EVENTS_CROSS {}
    0x200 => reg32 SHORTS {
        0 => READY_SAMPLE: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => READY_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => DOWN_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => UP_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CROSS_STOP: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x304 => reg32 INTENSET {
        0 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => DOWN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => UP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => CROSS: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x308 => reg32 INTENCLR {
        0 => READY: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => DOWN: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => UP: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => CROSS: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x400 => reg32 RESULT {
        0 => RESULT: ro {
            0 => Bellow,
            1 => Above,
        }
    }
    0x500 => reg32 ENABLE {
        0..1 => ENABLE: rw {
            0x00 => Disabled,
            0x01 => Enabled,
        }
    }
    0x504 => reg32 PSEL {
        0..2 => PSEL: rw {
            0 => AnalogInput0,
            1 => AnalogInput1,
            2 => AnalogInput2,
            3 => AnalogInput3,
            4 => AnalogInput4,
            5 => AnalogInput5,
            6 => AnalogInput6,
            7 => AnalogInput7,
        }
    }
    0x508 => reg32 REFSEL {
        0..2 => REFSEL: rw {
            0 => SupplyOneEighthPrescaling,
            1 => SupplyTwoEighthsPrescaling,
            2 => SupplyThreeEighthsPrescaling,
            3 => SupplyFourEighthsPrescaling,
            4 => SupplyFiveEighthsPrescaling,
            5 => SupplySixEighthsPrescaling,
            6 => SupplySevenEighthsPrescaling,
            7 => ARef,
        }
    }
    0x50C => reg32 EXTREFSEL {
        0 => EXTREFSEL: rw {
            0 => AnalogReference0,
            1 => AnalogReference1,
        }
    }
    0x520 => reg32 ANADETECT {
        0..1 => ANADETECT: rw {
            0 => Cross,
            1 => Up,
            2 => Down,
        }
    }
    0xFFC => reg32 POWER {
        0 => POWER: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
});

ioregs! (SWI @ 0x40014000 = {
    0x000 => reg32 UNUSED {}
});

ioregs! (NVMC @ 0x4001E000 = {
    0x400 => reg32 READY {
        0 => READY: ro {
            0 => Busy,
            1 => Ready,
        }
    }
    0x504 => reg32 CONFIG {
        0..1 => WEN: rw {
            0x00 => Ren,
            0x01 => Wen,
            0x02 => Een,
        }
    }
    0x508 => reg32 ERASEPAGE {}
    0x508 => reg32 ERASEPCR1 {}
    0x50C => reg32 ERASEALL {
        0 => ERASEALL: rw {
            0 => NoOperation,
            1 => Erase,
        }
    }
    0x510 => reg32 ERASEPCR0 {}
    0x514 => reg32 ERASEUICR {
        0 => ERASEUICR: rw {
            0 => NoOperation,
            1 => Erase,
        }
    }
});

ioregs! (PPI @ 0x4001F000 = {
    0x000 => group TASKS_CHG[4] {
        0x000 => reg32 EN {}
        0x004 => reg32 DIS {}
    }
    0x500 => reg32 CHEN {
        0 => CH0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => CH1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => CH2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => CH3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CH4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => CH5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => CH6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => CH7: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => CH8: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => CH9: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => CH10: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => CH11: rw {
            0 => Disabled,
            1 => Enabled,
        }
        12 => CH12: rw {
            0 => Disabled,
            1 => Enabled,
        }
        13 => CH13: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => CH14: rw {
            0 => Disabled,
            1 => Enabled,
        }
        15 => CH15: rw {
            0 => Disabled,
            1 => Enabled,
        }
        20 => CH20: rw {
            0 => Disabled,
            1 => Enabled,
        }
        21 => CH21: rw {
            0 => Disabled,
            1 => Enabled,
        }
        22 => CH22: rw {
            0 => Disabled,
            1 => Enabled,
        }
        23 => CH23: rw {
            0 => Disabled,
            1 => Enabled,
        }
        24 => CH24: rw {
            0 => Disabled,
            1 => Enabled,
        }
        25 => CH25: rw {
            0 => Disabled,
            1 => Enabled,
        }
        26 => CH26: rw {
            0 => Disabled,
            1 => Enabled,
        }
        27 => CH27: rw {
            0 => Disabled,
            1 => Enabled,
        }
        28 => CH28: rw {
            0 => Disabled,
            1 => Enabled,
        }
        29 => CH29: rw {
            0 => Disabled,
            1 => Enabled,
        }
        30 => CH30: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => CH31: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x504 => reg32 CHENSET {
        0 => CH0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => CH1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => CH2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => CH3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CH4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => CH5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => CH6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => CH7: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => CH8: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => CH9: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => CH10: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => CH11: rw {
            0 => Disabled,
            1 => Enabled,
        }
        12 => CH12: rw {
            0 => Disabled,
            1 => Enabled,
        }
        13 => CH13: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => CH14: rw {
            0 => Disabled,
            1 => Enabled,
        }
        15 => CH15: rw {
            0 => Disabled,
            1 => Enabled,
        }
        20 => CH20: rw {
            0 => Disabled,
            1 => Enabled,
        }
        21 => CH21: rw {
            0 => Disabled,
            1 => Enabled,
        }
        22 => CH22: rw {
            0 => Disabled,
            1 => Enabled,
        }
        23 => CH23: rw {
            0 => Disabled,
            1 => Enabled,
        }
        24 => CH24: rw {
            0 => Disabled,
            1 => Enabled,
        }
        25 => CH25: rw {
            0 => Disabled,
            1 => Enabled,
        }
        26 => CH26: rw {
            0 => Disabled,
            1 => Enabled,
        }
        27 => CH27: rw {
            0 => Disabled,
            1 => Enabled,
        }
        28 => CH28: rw {
            0 => Disabled,
            1 => Enabled,
        }
        29 => CH29: rw {
            0 => Disabled,
            1 => Enabled,
        }
        30 => CH30: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => CH31: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x508 => reg32 CHENCLR {
        0 => CH0: rw {
            0 => Disabled,
            1 => Enabled,
        }
        1 => CH1: rw {
            0 => Disabled,
            1 => Enabled,
        }
        2 => CH2: rw {
            0 => Disabled,
            1 => Enabled,
        }
        3 => CH3: rw {
            0 => Disabled,
            1 => Enabled,
        }
        4 => CH4: rw {
            0 => Disabled,
            1 => Enabled,
        }
        5 => CH5: rw {
            0 => Disabled,
            1 => Enabled,
        }
        6 => CH6: rw {
            0 => Disabled,
            1 => Enabled,
        }
        7 => CH7: rw {
            0 => Disabled,
            1 => Enabled,
        }
        8 => CH8: rw {
            0 => Disabled,
            1 => Enabled,
        }
        9 => CH9: rw {
            0 => Disabled,
            1 => Enabled,
        }
        10 => CH10: rw {
            0 => Disabled,
            1 => Enabled,
        }
        11 => CH11: rw {
            0 => Disabled,
            1 => Enabled,
        }
        12 => CH12: rw {
            0 => Disabled,
            1 => Enabled,
        }
        13 => CH13: rw {
            0 => Disabled,
            1 => Enabled,
        }
        14 => CH14: rw {
            0 => Disabled,
            1 => Enabled,
        }
        15 => CH15: rw {
            0 => Disabled,
            1 => Enabled,
        }
        20 => CH20: rw {
            0 => Disabled,
            1 => Enabled,
        }
        21 => CH21: rw {
            0 => Disabled,
            1 => Enabled,
        }
        22 => CH22: rw {
            0 => Disabled,
            1 => Enabled,
        }
        23 => CH23: rw {
            0 => Disabled,
            1 => Enabled,
        }
        24 => CH24: rw {
            0 => Disabled,
            1 => Enabled,
        }
        25 => CH25: rw {
            0 => Disabled,
            1 => Enabled,
        }
        26 => CH26: rw {
            0 => Disabled,
            1 => Enabled,
        }
        27 => CH27: rw {
            0 => Disabled,
            1 => Enabled,
        }
        28 => CH28: rw {
            0 => Disabled,
            1 => Enabled,
        }
        29 => CH29: rw {
            0 => Disabled,
            1 => Enabled,
        }
        30 => CH30: rw {
            0 => Disabled,
            1 => Enabled,
        }
        31 => CH31: rw {
            0 => Disabled,
            1 => Enabled,
        }
    }
    0x510 => group CH[16] {
        0x000 => reg32 EEP {}
        0x004 => reg32 TEP {}
    }
    0x800 => group CHG[4] {
        0x0 => reg32 CHG {
            0 => CH0: rw {
                0 => Excluded,
                1 => Included,
            }
            1 => CH1: rw {
                0 => Excluded,
                1 => Included,
            }
            2 => CH2: rw {
                0 => Excluded,
                1 => Included,
            }
            3 => CH3: rw {
                0 => Excluded,
                1 => Included,
            }
            4 => CH4: rw {
                0 => Excluded,
                1 => Included,
            }
            5 => CH5: rw {
                0 => Excluded,
                1 => Included,
            }
            6 => CH6: rw {
                0 => Excluded,
                1 => Included,
            }
            7 => CH7: rw {
                0 => Excluded,
                1 => Included,
            }
            8 => CH8: rw {
                0 => Excluded,
                1 => Included,
            }
            9 => CH9: rw {
                0 => Excluded,
                1 => Included,
            }
            10 => CH10: rw {
                0 => Excluded,
                1 => Included,
            }
            11 => CH11: rw {
                0 => Excluded,
                1 => Included,
            }
            12 => CH12: rw {
                0 => Excluded,
                1 => Included,
            }
            13 => CH13: rw {
                0 => Excluded,
                1 => Included,
            }
            14 => CH14: rw {
                0 => Excluded,
                1 => Included,
            }
            15 => CH15: rw {
                0 => Excluded,
                1 => Included,
            }
            20 => CH20: rw {
                0 => Excluded,
                1 => Included,
            }
            21 => CH21: rw {
                0 => Excluded,
                1 => Included,
            }
            22 => CH22: rw {
                0 => Excluded,
                1 => Included,
            }
            23 => CH23: rw {
                0 => Excluded,
                1 => Included,
            }
            24 => CH24: rw {
                0 => Excluded,
                1 => Included,
            }
            25 => CH25: rw {
                0 => Excluded,
                1 => Included,
            }
            26 => CH26: rw {
                0 => Excluded,
                1 => Included,
            }
            27 => CH27: rw {
                0 => Excluded,
                1 => Included,
            }
            28 => CH28: rw {
                0 => Excluded,
                1 => Included,
            }
            29 => CH29: rw {
                0 => Excluded,
                1 => Included,
            }
            30 => CH30: rw {
                0 => Excluded,
                1 => Included,
            }
            31 => CH31: rw {
                0 => Excluded,
                1 => Included,
            }
        }
    }
});

ioregs! (FICR @ 0x10000000 = {
    0x010 => reg32 CODEPAGESIZE {}
    0x014 => reg32 CODESIZE {}
    0x028 => reg32 CLENR0 {}
    0x02C => reg32 PPFC {
        0..7 => PPFC: ro {
            0xFF => NotPresent,
            0x00 => Present,
        }
    }
    0x034 => reg32 NUMRAMBLOCK {}
    0x038 => reg32 SIZERAMBLOCKS {}
    0x038 => group SIZERAMBLOCK[4] {
        0x0 => reg32 SIZERAMBLOCK {}
    }
    0x05C => reg32 CONFIGID {
        0..15 => HWID: ro,
        16..31 => FWID: ro,
    }
    0x060 => group DEVICEID[2] {
        0x0 => reg32 DEVICEID {}
    }
    0x080 => group ER[4] {
        0x0 => reg32 ER {}
    }
    0x090 => group IR[4] {
        0x0 => reg32 IR {}
    }
    0x0A0 => reg32 DEVICEADDRTYPE {
        0 => DEVICEADDRTYPE: ro {
            0 => Public,
            1 => Random,
        }
    }
    0x0A4 => group DEVICEADDR[2] {
        0x0 => reg32 DEVICEADDR {}
    }
    0x0AC => reg32 OVERRIDEEN {
        0 => NRF_1MBIT: ro {
            0 => Override,
            1 => NotOverride,
        }
        3 => BLE_1MBIT: ro {
            0 => Override,
            1 => NotOverride,
        }
    }
    0x0B0 => group NRF_1MBIT[5] {
        0x0 => reg32 NRF_1MBIT {}
    }
    0x0EC => group BLE_1MBIT[5] {
        0x0 => reg32 BLE_1MBIT {}
    }
});

ioregs! (UICR @ 0x10001000 = {
    0x000 => reg32 CLENR0 {}
    0x004 => reg32 RBPCONF {
        0..7 => PR0: rw {
            0xFF => Disabled,
            0x00 => Enabled,
        }
        8..15 => PALL: rw {
            0xFF => Disabled,
            0x00 => Enabled,
        }
    }
    0x008 => reg32 XTALFREQ {
        0..7 => XTALFREQ: rw {
            0xFF => 16MHz,
            0x00 => 32MHz,
        }
    }
    0x010 => reg32 FWID {
        0..15 => FWID: ro,
    }
    0x014 => reg32 BOOTLOADERADDR {}
    0x014 => group NRFFW[15] {
        0x0 => reg32 NRFFW {}
    }
    0x050 => group NRFHW[12] {
        0x0 => reg32 NRFHW {}
    }
    0x080 => group CUSTOMER[32] {
        0x0 => reg32 CUSTOMER {}
    }
});

ioregs! (GPIO @ 0x50000000 = {
    0x504 => reg32 OUT {
        0 => PIN0: rw {
            0 => Low,
            1 => High,
        }
        1 => PIN1: rw {
            0 => Low,
            1 => High,
        }
        2 => PIN2: rw {
            0 => Low,
            1 => High,
        }
        3 => PIN3: rw {
            0 => Low,
            1 => High,
        }
        4 => PIN4: rw {
            0 => Low,
            1 => High,
        }
        5 => PIN5: rw {
            0 => Low,
            1 => High,
        }
        6 => PIN6: rw {
            0 => Low,
            1 => High,
        }
        7 => PIN7: rw {
            0 => Low,
            1 => High,
        }
        8 => PIN8: rw {
            0 => Low,
            1 => High,
        }
        9 => PIN9: rw {
            0 => Low,
            1 => High,
        }
        10 => PIN10: rw {
            0 => Low,
            1 => High,
        }
        11 => PIN11: rw {
            0 => Low,
            1 => High,
        }
        12 => PIN12: rw {
            0 => Low,
            1 => High,
        }
        13 => PIN13: rw {
            0 => Low,
            1 => High,
        }
        14 => PIN14: rw {
            0 => Low,
            1 => High,
        }
        15 => PIN15: rw {
            0 => Low,
            1 => High,
        }
        16 => PIN16: rw {
            0 => Low,
            1 => High,
        }
        17 => PIN17: rw {
            0 => Low,
            1 => High,
        }
        18 => PIN18: rw {
            0 => Low,
            1 => High,
        }
        19 => PIN19: rw {
            0 => Low,
            1 => High,
        }
        20 => PIN20: rw {
            0 => Low,
            1 => High,
        }
        21 => PIN21: rw {
            0 => Low,
            1 => High,
        }
        22 => PIN22: rw {
            0 => Low,
            1 => High,
        }
        23 => PIN23: rw {
            0 => Low,
            1 => High,
        }
        24 => PIN24: rw {
            0 => Low,
            1 => High,
        }
        25 => PIN25: rw {
            0 => Low,
            1 => High,
        }
        26 => PIN26: rw {
            0 => Low,
            1 => High,
        }
        27 => PIN27: rw {
            0 => Low,
            1 => High,
        }
        28 => PIN28: rw {
            0 => Low,
            1 => High,
        }
        29 => PIN29: rw {
            0 => Low,
            1 => High,
        }
        30 => PIN30: rw {
            0 => Low,
            1 => High,
        }
        31 => PIN31: rw {
            0 => Low,
            1 => High,
        }
    }
    0x508 => reg32 OUTSET {
        0 => PIN0: rw {
            0 => Low,
            1 => High,
        }
        1 => PIN1: rw {
            0 => Low,
            1 => High,
        }
        2 => PIN2: rw {
            0 => Low,
            1 => High,
        }
        3 => PIN3: rw {
            0 => Low,
            1 => High,
        }
        4 => PIN4: rw {
            0 => Low,
            1 => High,
        }
        5 => PIN5: rw {
            0 => Low,
            1 => High,
        }
        6 => PIN6: rw {
            0 => Low,
            1 => High,
        }
        7 => PIN7: rw {
            0 => Low,
            1 => High,
        }
        8 => PIN8: rw {
            0 => Low,
            1 => High,
        }
        9 => PIN9: rw {
            0 => Low,
            1 => High,
        }
        10 => PIN10: rw {
            0 => Low,
            1 => High,
        }
        11 => PIN11: rw {
            0 => Low,
            1 => High,
        }
        12 => PIN12: rw {
            0 => Low,
            1 => High,
        }
        13 => PIN13: rw {
            0 => Low,
            1 => High,
        }
        14 => PIN14: rw {
            0 => Low,
            1 => High,
        }
        15 => PIN15: rw {
            0 => Low,
            1 => High,
        }
        16 => PIN16: rw {
            0 => Low,
            1 => High,
        }
        17 => PIN17: rw {
            0 => Low,
            1 => High,
        }
        18 => PIN18: rw {
            0 => Low,
            1 => High,
        }
        19 => PIN19: rw {
            0 => Low,
            1 => High,
        }
        20 => PIN20: rw {
            0 => Low,
            1 => High,
        }
        21 => PIN21: rw {
            0 => Low,
            1 => High,
        }
        22 => PIN22: rw {
            0 => Low,
            1 => High,
        }
        23 => PIN23: rw {
            0 => Low,
            1 => High,
        }
        24 => PIN24: rw {
            0 => Low,
            1 => High,
        }
        25 => PIN25: rw {
            0 => Low,
            1 => High,
        }
        26 => PIN26: rw {
            0 => Low,
            1 => High,
        }
        27 => PIN27: rw {
            0 => Low,
            1 => High,
        }
        28 => PIN28: rw {
            0 => Low,
            1 => High,
        }
        29 => PIN29: rw {
            0 => Low,
            1 => High,
        }
        30 => PIN30: rw {
            0 => Low,
            1 => High,
        }
        31 => PIN31: rw {
            0 => Low,
            1 => High,
        }
    }
    0x50C => reg32 OUTCLR {
        0 => PIN0: rw {
            0 => Low,
            1 => High,
        }
        1 => PIN1: rw {
            0 => Low,
            1 => High,
        }
        2 => PIN2: rw {
            0 => Low,
            1 => High,
        }
        3 => PIN3: rw {
            0 => Low,
            1 => High,
        }
        4 => PIN4: rw {
            0 => Low,
            1 => High,
        }
        5 => PIN5: rw {
            0 => Low,
            1 => High,
        }
        6 => PIN6: rw {
            0 => Low,
            1 => High,
        }
        7 => PIN7: rw {
            0 => Low,
            1 => High,
        }
        8 => PIN8: rw {
            0 => Low,
            1 => High,
        }
        9 => PIN9: rw {
            0 => Low,
            1 => High,
        }
        10 => PIN10: rw {
            0 => Low,
            1 => High,
        }
        11 => PIN11: rw {
            0 => Low,
            1 => High,
        }
        12 => PIN12: rw {
            0 => Low,
            1 => High,
        }
        13 => PIN13: rw {
            0 => Low,
            1 => High,
        }
        14 => PIN14: rw {
            0 => Low,
            1 => High,
        }
        15 => PIN15: rw {
            0 => Low,
            1 => High,
        }
        16 => PIN16: rw {
            0 => Low,
            1 => High,
        }
        17 => PIN17: rw {
            0 => Low,
            1 => High,
        }
        18 => PIN18: rw {
            0 => Low,
            1 => High,
        }
        19 => PIN19: rw {
            0 => Low,
            1 => High,
        }
        20 => PIN20: rw {
            0 => Low,
            1 => High,
        }
        21 => PIN21: rw {
            0 => Low,
            1 => High,
        }
        22 => PIN22: rw {
            0 => Low,
            1 => High,
        }
        23 => PIN23: rw {
            0 => Low,
            1 => High,
        }
        24 => PIN24: rw {
            0 => Low,
            1 => High,
        }
        25 => PIN25: rw {
            0 => Low,
            1 => High,
        }
        26 => PIN26: rw {
            0 => Low,
            1 => High,
        }
        27 => PIN27: rw {
            0 => Low,
            1 => High,
        }
        28 => PIN28: rw {
            0 => Low,
            1 => High,
        }
        29 => PIN29: rw {
            0 => Low,
            1 => High,
        }
        30 => PIN30: rw {
            0 => Low,
            1 => High,
        }
        31 => PIN31: rw {
            0 => Low,
            1 => High,
        }
    }
    0x510 => reg32 IN {
        0 => PIN0: ro {
            0 => Low,
            1 => High,
        }
        1 => PIN1: ro {
            0 => Low,
            1 => High,
        }
        2 => PIN2: ro {
            0 => Low,
            1 => High,
        }
        3 => PIN3: ro {
            0 => Low,
            1 => High,
        }
        4 => PIN4: ro {
            0 => Low,
            1 => High,
        }
        5 => PIN5: ro {
            0 => Low,
            1 => High,
        }
        6 => PIN6: ro {
            0 => Low,
            1 => High,
        }
        7 => PIN7: ro {
            0 => Low,
            1 => High,
        }
        8 => PIN8: ro {
            0 => Low,
            1 => High,
        }
        9 => PIN9: ro {
            0 => Low,
            1 => High,
        }
        10 => PIN10: ro {
            0 => Low,
            1 => High,
        }
        11 => PIN11: ro {
            0 => Low,
            1 => High,
        }
        12 => PIN12: ro {
            0 => Low,
            1 => High,
        }
        13 => PIN13: ro {
            0 => Low,
            1 => High,
        }
        14 => PIN14: ro {
            0 => Low,
            1 => High,
        }
        15 => PIN15: ro {
            0 => Low,
            1 => High,
        }
        16 => PIN16: ro {
            0 => Low,
            1 => High,
        }
        17 => PIN17: ro {
            0 => Low,
            1 => High,
        }
        18 => PIN18: ro {
            0 => Low,
            1 => High,
        }
        19 => PIN19: ro {
            0 => Low,
            1 => High,
        }
        20 => PIN20: ro {
            0 => Low,
            1 => High,
        }
        21 => PIN21: ro {
            0 => Low,
            1 => High,
        }
        22 => PIN22: ro {
            0 => Low,
            1 => High,
        }
        23 => PIN23: ro {
            0 => Low,
            1 => High,
        }
        24 => PIN24: ro {
            0 => Low,
            1 => High,
        }
        25 => PIN25: ro {
            0 => Low,
            1 => High,
        }
        26 => PIN26: ro {
            0 => Low,
            1 => High,
        }
        27 => PIN27: ro {
            0 => Low,
            1 => High,
        }
        28 => PIN28: ro {
            0 => Low,
            1 => High,
        }
        29 => PIN29: ro {
            0 => Low,
            1 => High,
        }
        30 => PIN30: ro {
            0 => Low,
            1 => High,
        }
        31 => PIN31: ro {
            0 => Low,
            1 => High,
        }
    }
    0x514 => reg32 DIR {
        0 => PIN0: rw {
            0 => Input,
            1 => Output,
        }
        1 => PIN1: rw {
            0 => Input,
            1 => Output,
        }
        2 => PIN2: rw {
            0 => Input,
            1 => Output,
        }
        3 => PIN3: rw {
            0 => Input,
            1 => Output,
        }
        4 => PIN4: rw {
            0 => Input,
            1 => Output,
        }
        5 => PIN5: rw {
            0 => Input,
            1 => Output,
        }
        6 => PIN6: rw {
            0 => Input,
            1 => Output,
        }
        7 => PIN7: rw {
            0 => Input,
            1 => Output,
        }
        8 => PIN8: rw {
            0 => Input,
            1 => Output,
        }
        9 => PIN9: rw {
            0 => Input,
            1 => Output,
        }
        10 => PIN10: rw {
            0 => Input,
            1 => Output,
        }
        11 => PIN11: rw {
            0 => Input,
            1 => Output,
        }
        12 => PIN12: rw {
            0 => Input,
            1 => Output,
        }
        13 => PIN13: rw {
            0 => Input,
            1 => Output,
        }
        14 => PIN14: rw {
            0 => Input,
            1 => Output,
        }
        15 => PIN15: rw {
            0 => Input,
            1 => Output,
        }
        16 => PIN16: rw {
            0 => Input,
            1 => Output,
        }
        17 => PIN17: rw {
            0 => Input,
            1 => Output,
        }
        18 => PIN18: rw {
            0 => Input,
            1 => Output,
        }
        19 => PIN19: rw {
            0 => Input,
            1 => Output,
        }
        20 => PIN20: rw {
            0 => Input,
            1 => Output,
        }
        21 => PIN21: rw {
            0 => Input,
            1 => Output,
        }
        22 => PIN22: rw {
            0 => Input,
            1 => Output,
        }
        23 => PIN23: rw {
            0 => Input,
            1 => Output,
        }
        24 => PIN24: rw {
            0 => Input,
            1 => Output,
        }
        25 => PIN25: rw {
            0 => Input,
            1 => Output,
        }
        26 => PIN26: rw {
            0 => Input,
            1 => Output,
        }
        27 => PIN27: rw {
            0 => Input,
            1 => Output,
        }
        28 => PIN28: rw {
            0 => Input,
            1 => Output,
        }
        29 => PIN29: rw {
            0 => Input,
            1 => Output,
        }
        30 => PIN30: rw {
            0 => Input,
            1 => Output,
        }
        31 => PIN31: rw {
            0 => Input,
            1 => Output,
        }
    }
    0x518 => reg32 DIRSET {
        0 => PIN0: rw {
            0 => Input,
            1 => Output,
        }
        1 => PIN1: rw {
            0 => Input,
            1 => Output,
        }
        2 => PIN2: rw {
            0 => Input,
            1 => Output,
        }
        3 => PIN3: rw {
            0 => Input,
            1 => Output,
        }
        4 => PIN4: rw {
            0 => Input,
            1 => Output,
        }
        5 => PIN5: rw {
            0 => Input,
            1 => Output,
        }
        6 => PIN6: rw {
            0 => Input,
            1 => Output,
        }
        7 => PIN7: rw {
            0 => Input,
            1 => Output,
        }
        8 => PIN8: rw {
            0 => Input,
            1 => Output,
        }
        9 => PIN9: rw {
            0 => Input,
            1 => Output,
        }
        10 => PIN10: rw {
            0 => Input,
            1 => Output,
        }
        11 => PIN11: rw {
            0 => Input,
            1 => Output,
        }
        12 => PIN12: rw {
            0 => Input,
            1 => Output,
        }
        13 => PIN13: rw {
            0 => Input,
            1 => Output,
        }
        14 => PIN14: rw {
            0 => Input,
            1 => Output,
        }
        15 => PIN15: rw {
            0 => Input,
            1 => Output,
        }
        16 => PIN16: rw {
            0 => Input,
            1 => Output,
        }
        17 => PIN17: rw {
            0 => Input,
            1 => Output,
        }
        18 => PIN18: rw {
            0 => Input,
            1 => Output,
        }
        19 => PIN19: rw {
            0 => Input,
            1 => Output,
        }
        20 => PIN20: rw {
            0 => Input,
            1 => Output,
        }
        21 => PIN21: rw {
            0 => Input,
            1 => Output,
        }
        22 => PIN22: rw {
            0 => Input,
            1 => Output,
        }
        23 => PIN23: rw {
            0 => Input,
            1 => Output,
        }
        24 => PIN24: rw {
            0 => Input,
            1 => Output,
        }
        25 => PIN25: rw {
            0 => Input,
            1 => Output,
        }
        26 => PIN26: rw {
            0 => Input,
            1 => Output,
        }
        27 => PIN27: rw {
            0 => Input,
            1 => Output,
        }
        28 => PIN28: rw {
            0 => Input,
            1 => Output,
        }
        29 => PIN29: rw {
            0 => Input,
            1 => Output,
        }
        30 => PIN30: rw {
            0 => Input,
            1 => Output,
        }
        31 => PIN31: rw {
            0 => Input,
            1 => Output,
        }
    }
    0x51C => reg32 DIRCLR {
        0 => PIN0: rw {
            0 => Input,
            1 => Output,
        }
        1 => PIN1: rw {
            0 => Input,
            1 => Output,
        }
        2 => PIN2: rw {
            0 => Input,
            1 => Output,
        }
        3 => PIN3: rw {
            0 => Input,
            1 => Output,
        }
        4 => PIN4: rw {
            0 => Input,
            1 => Output,
        }
        5 => PIN5: rw {
            0 => Input,
            1 => Output,
        }
        6 => PIN6: rw {
            0 => Input,
            1 => Output,
        }
        7 => PIN7: rw {
            0 => Input,
            1 => Output,
        }
        8 => PIN8: rw {
            0 => Input,
            1 => Output,
        }
        9 => PIN9: rw {
            0 => Input,
            1 => Output,
        }
        10 => PIN10: rw {
            0 => Input,
            1 => Output,
        }
        11 => PIN11: rw {
            0 => Input,
            1 => Output,
        }
        12 => PIN12: rw {
            0 => Input,
            1 => Output,
        }
        13 => PIN13: rw {
            0 => Input,
            1 => Output,
        }
        14 => PIN14: rw {
            0 => Input,
            1 => Output,
        }
        15 => PIN15: rw {
            0 => Input,
            1 => Output,
        }
        16 => PIN16: rw {
            0 => Input,
            1 => Output,
        }
        17 => PIN17: rw {
            0 => Input,
            1 => Output,
        }
        18 => PIN18: rw {
            0 => Input,
            1 => Output,
        }
        19 => PIN19: rw {
            0 => Input,
            1 => Output,
        }
        20 => PIN20: rw {
            0 => Input,
            1 => Output,
        }
        21 => PIN21: rw {
            0 => Input,
            1 => Output,
        }
        22 => PIN22: rw {
            0 => Input,
            1 => Output,
        }
        23 => PIN23: rw {
            0 => Input,
            1 => Output,
        }
        24 => PIN24: rw {
            0 => Input,
            1 => Output,
        }
        25 => PIN25: rw {
            0 => Input,
            1 => Output,
        }
        26 => PIN26: rw {
            0 => Input,
            1 => Output,
        }
        27 => PIN27: rw {
            0 => Input,
            1 => Output,
        }
        28 => PIN28: rw {
            0 => Input,
            1 => Output,
        }
        29 => PIN29: rw {
            0 => Input,
            1 => Output,
        }
        30 => PIN30: rw {
            0 => Input,
            1 => Output,
        }
        31 => PIN31: rw {
            0 => Input,
            1 => Output,
        }
    }
    0x700 => group PIN_CNF[32] {
        0x0 => reg32 PIN_CNF {
            0 => DIR: rw {
                0 => Input,
                1 => Output,
            }
            1 => INPUT: rw {
                0 => Connect,
                1 => Disconnect,
            }
            2..3 => PULL: rw {
                0x00 => Disabled,
                0x01 => Pulldown,
                0x03 => Pullup,
            }
            8..10 => DRIVE: rw {
                0x00 => S0S1,
                0x01 => H0S1,
                0x02 => S0H1,
                0x03 => H0H1,
                0x04 => D0S1,
                0x05 => D0H1,
                0x06 => S0D1,
                0x07 => H0D1,
            }
            16..17 => SENSE: rw {
                0x00 => Disabled,
                0x02 => High,
                0x03 => Low,
            }
        }
    }
});
