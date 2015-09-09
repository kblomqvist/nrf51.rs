/// nRF51 reference description for radio MCU with ARM 32-bit Cortex-M0
/// Microcontroller at 16MHz CPU clock

use volatile_cell::VolatileCell;
use core::ops::Drop;


ioregs! (POWER @ 0x40000000 = {
    /// Power Control.
    0x078 => reg32 TASKS_CONSTLAT {} //! Enable constant latency mode.
    0x07C => reg32 TASKS_LOWPWR {} //! Enable low power mode (variable latency).
    0x108 => reg32 EVENTS_POFWARN {} //! Power failure warning.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        2 => POFWARN: rw { //! Enable interrupt on POFWARN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        2 => POFWARN: rw { //! Disable interrupt on POFWARN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 RESETREAS { //! Reset reason.
        0 => RESETPIN: rw { //! Reset from pin-reset detected.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        1 => DOG: rw { //! Reset from watchdog detected.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        2 => SREQ: rw { //! Reset from AIRCR.SYSRESETREQ detected.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        3 => LOCKUP: rw { //! Reset from CPU lock-up detected.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        16 => OFF: rw { //! Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        17 => LPCOMP: rw { //! Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
        18 => DIF: rw { //! Reset from wake-up from OFF mode detected by entering into debug interface mode.
            0 => NotDetected, //= Reset not detected.
            1 => Detected, //= Reset detected.
        }
    }
    0x428 => reg32 RAMSTATUS { //! Ram status register.
        0 => RAMBLOCK0: ro { //! RAM block 0 status.
            0 => Off, //= RAM block 0 is off or powering up.
            1 => On, //= RAM block 0 is on.
        }
        1 => RAMBLOCK1: ro { //! RAM block 1 status.
            0 => Off, //= RAM block 1 is off or powering up.
            1 => On, //= RAM block 1 is on.
        }
        2 => RAMBLOCK2: ro { //! RAM block 2 status.
            0 => Off, //= RAM block 2 is off or powering up.
            1 => On, //= RAM block 2 is on.
        }
        3 => RAMBLOCK3: ro { //! RAM block 3 status.
            0 => Off, //= RAM block 3 is off or powering up.
            1 => On, //= RAM block 3 is on.
        }
    }
    0x500 => reg32 SYSTEMOFF { //! System off register.
        0 => SYSTEMOFF: wo { //! Enter system off mode.
            1 => Enter, //= Enter system off mode.
        }
    }
    0x510 => reg32 POFCON { //! Power failure configuration.
        0 => POF: rw { //! Power failure comparator enable.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        1..2 => THRESHOLD: rw { //! Set threshold level.
            0x00 => V21, //= Set threshold to 2.1Volts.
            0x01 => V23, //= Set threshold to 2.3Volts.
            0x02 => V25, //= Set threshold to 2.5Volts.
            0x03 => V27, //= Set threshold to 2.7Volts.
        }
    }
    0x51C => reg32 GPREGRET { //! General purpose retention register. This register is a retained register.
        0..7 => GPREGRET: rw,
    }
    0x524 => reg32 RAMON { //! Ram on/off.
        0 => ONRAM0: rw { //! RAM block 0 behaviour in ON mode.
            0 => RAM0Off, //= RAM block 0 OFF in ON mode.
            1 => RAM0On, //= RAM block 0 ON in ON mode.
        }
        1 => ONRAM1: rw { //! RAM block 1 behaviour in ON mode.
            0 => RAM1Off, //= RAM block 1 OFF in ON mode.
            1 => RAM1On, //= RAM block 1 ON in ON mode.
        }
        16 => OFFRAM0: rw { //! RAM block 0 behaviour in OFF mode.
            0 => RAM0Off, //= RAM block 0 OFF in OFF mode.
            1 => RAM0On, //= RAM block 0 ON in OFF mode.
        }
        17 => OFFRAM1: rw { //! RAM block 1 behaviour in OFF mode.
            0 => RAM1Off, //= RAM block 1 OFF in OFF mode.
            1 => RAM1On, //= RAM block 1 ON in OFF mode.
        }
    }
    0x544 => reg32 RESET { //! Pin reset functionality configuration register. This register is a retained register.
        0 => RESET: rw { //! Enable or disable pin reset in debug interface mode.
            0 => Disabled, //= Pin reset in debug interface mode disabled.
            1 => Enabled, //= Pin reset in debug interface mode enabled.
        }
    }
    0x554 => reg32 RAMONB { //! Ram on/off.
        0 => ONRAM2: rw { //! RAM block 2 behaviour in ON mode.
            0 => RAM2Off, //= RAM block 2 OFF in ON mode.
            1 => RAM2On, //= RAM block 2 ON in ON mode.
        }
        1 => ONRAM3: rw { //! RAM block 3 behaviour in ON mode.
            0 => RAM3Off, //= RAM block 33 OFF in ON mode.
            1 => RAM3On, //= RAM block 3 ON in ON mode.
        }
        16 => OFFRAM2: rw { //! RAM block 2 behaviour in OFF mode.
            0 => RAM2Off, //= RAM block 2 OFF in OFF mode.
            1 => RAM2On, //= RAM block 2 ON in OFF mode.
        }
        17 => OFFRAM3: rw { //! RAM block 3 behaviour in OFF mode.
            0 => RAM3Off, //= RAM block 3 OFF in OFF mode.
            1 => RAM3On, //= RAM block 3 ON in OFF mode.
        }
    }
    0x578 => reg32 DCDCEN { //! DCDC converter enable configuration register.
        0 => DCDCEN: rw { //! Enable DCDC converter.
            0 => Disabled, //= DCDC converter disabled.
            1 => Enabled, //= DCDC converter enabled.
        }
    }
    0xA08 => reg32 DCDCFORCE { //! DCDC power-up force register.
        0 => FORCEOFF: rw { //! DCDC power-up force off.
            0 => NoForce, //= No force.
            1 => Force, //= Force.
        }
        1 => FORCEON: rw { //! DCDC power-up force on.
            0 => NoForce, //= No force.
            1 => Force, //= Force.
        }
    }
});

ioregs! (CLOCK @ 0x40000000 = {
    /// Clock control.
    0x000 => reg32 TASKS_HFCLKSTART {} //! Start HFCLK clock source.
    0x004 => reg32 TASKS_HFCLKSTOP {} //! Stop HFCLK clock source.
    0x008 => reg32 TASKS_LFCLKSTART {} //! Start LFCLK clock source.
    0x00C => reg32 TASKS_LFCLKSTOP {} //! Stop LFCLK clock source.
    0x010 => reg32 TASKS_CAL {} //! Start calibration of LFCLK RC oscillator.
    0x014 => reg32 TASKS_CTSTART {} //! Start calibration timer.
    0x018 => reg32 TASKS_CTSTOP {} //! Stop calibration timer.
    0x100 => reg32 EVENTS_HFCLKSTARTED {} //! HFCLK oscillator started.
    0x104 => reg32 EVENTS_LFCLKSTARTED {} //! LFCLK oscillator started.
    0x10C => reg32 EVENTS_DONE {} //! Calibration of LFCLK RC oscillator completed.
    0x110 => reg32 EVENTS_CTTO {} //! Calibration timer timeout.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => HFCLKSTARTED: rw { //! Enable interrupt on HFCLKSTARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => LFCLKSTARTED: rw { //! Enable interrupt on LFCLKSTARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => DONE: rw { //! Enable interrupt on DONE event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => CTTO: rw { //! Enable interrupt on CTTO event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => HFCLKSTARTED: rw { //! Disable interrupt on HFCLKSTARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => LFCLKSTARTED: rw { //! Disable interrupt on LFCLKSTARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => DONE: rw { //! Disable interrupt on DONE event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => CTTO: rw { //! Disable interrupt on CTTO event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x408 => reg32 HFCLKRUN { //! Task HFCLKSTART trigger status.
        0 => STATUS: ro { //! Task HFCLKSTART trigger status.
            0 => NotTriggered, //= Task HFCLKSTART has not been triggered.
            1 => Triggered, //= Task HFCLKSTART has been triggered.
        }
    }
    0x40C => reg32 HFCLKSTAT { //! High frequency clock status.
        0 => SRC: ro { //! Active clock source for the HF clock.
            0 => RC, //= Internal 16MHz RC oscillator running and generating the HFCLK clock.
            1 => Xtal, //= External 16MHz/32MHz crystal oscillator running and generating the HFCLK clock.
        }
        16 => STATE: ro { //! State for the HFCLK.
            0 => NotRunning, //= HFCLK clock not running.
            1 => Running, //= HFCLK clock running.
        }
    }
    0x414 => reg32 LFCLKRUN { //! Task LFCLKSTART triggered status.
        0 => STATUS: ro { //! Task LFCLKSTART triggered status.
            0 => NotTriggered, //= Task LFCLKSTART has not been triggered.
            1 => Triggered, //= Task LFCLKSTART has been triggered.
        }
    }
    0x418 => reg32 LFCLKSTAT { //! Low frequency clock status.
        0..1 => SRC: ro { //! Active clock source for the LF clock.
            0 => RC, //= Internal 32KiHz RC oscillator running and generating the LFCLK clock.
            1 => Xtal, //= External 32KiHz crystal oscillator running and generating the LFCLK clock.
            2 => Synth, //= Internal 32KiHz synthesizer from the HFCLK running and generating the LFCLK clock.
        }
        16 => STATE: ro { //! State for the LF clock.
            0 => NotRunning, //= LFCLK clock not running.
            1 => Running, //= LFCLK clock running.
        }
    }
    0x41C => reg32 LFCLKSRCCOPY { //! Clock source for the LFCLK clock, set when task LKCLKSTART is triggered.
        0..1 => SRC: ro { //! Clock source for the LFCLK clock, set when task LKCLKSTART is triggered.
            0 => RC, //= Internal 32KiHz RC oscillator.
            1 => Xtal, //= External 32KiHz crystal.
            2 => Synth, //= Internal 32KiHz synthesizer from HFCLK system clock.
        }
    }
    0x518 => reg32 LFCLKSRC { //! Clock source for the LFCLK clock.
        0..1 => SRC: rw { //! Clock source.
            0 => RC, //= Internal 32KiHz RC oscillator.
            1 => Xtal, //= External 32KiHz crystal.
            2 => Synth, //= Internal 32KiHz synthesizer from HFCLK system clock.
        }
    }
    0x538 => reg32 CTIV { //! Calibration timer interval.
        0..6 => CTIV: rw,
    }
    0x550 => reg32 XTALFREQ { //! Crystal frequency.
        0..7 => XTALFREQ: rw { //! External Xtal frequency selection.
            0xFF => 16MHz, //= 16MHz xtal is used as source for the HFCLK oscillator.
            0x00 => 32MHz, //= 32MHz xtal is used as source for the HFCLK oscillator.
        }
    }
});

ioregs! (MPU @ 0x40000000 = {
    /// Memory Protection Unit.
    0x528 => reg32 PERR0 { //! Configuration of peripherals in mpu regions.
        0 => POWER_CLOCK: rw { //! POWER_CLOCK region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        1 => RADIO: rw { //! RADIO region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        2 => UART0: rw { //! UART0 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        3 => SPI0_TWI0: rw { //! SPI0 and TWI0 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        4 => SPI1_TWI1: rw { //! SPI1 and TWI1 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        6 => GPIOTE: rw { //! GPIOTE region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        7 => ADC: rw { //! ADC region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        8 => TIMER0: rw { //! TIMER0 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        9 => TIMER1: rw { //! TIMER1 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        10 => TIMER2: rw { //! TIMER2 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        11 => RTC0: rw { //! RTC0 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        12 => TEMP: rw { //! TEMP region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        13 => RNG: rw { //! RNG region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        14 => ECB: rw { //! ECB region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        15 => CCM_AAR: rw { //! CCM and AAR region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        16 => WDT: rw { //! WDT region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        17 => RTC1: rw { //! RTC1 region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        18 => QDEC: rw { //! QDEC region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        19 => LPCOMP: rw { //! LPCOMP region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        30 => NVMC: rw { //! NVMC region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
        31 => PPI: rw { //! PPI region configuration.
            1 => InRegion0, //= Peripheral configured in region 0.
            0 => InRegion1, //= Peripheral configured in region 1.
        }
    }
    0x52C => reg32 RLENR0 {} //! Length of RAM region 0.
    0x600 => reg32 PROTENSET0 { //! Erase and write protection bit enable set register.
        0 => PROTREG0: rw { //! Protection enable for region 0.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        1 => PROTREG1: rw { //! Protection enable for region 1.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        2 => PROTREG2: rw { //! Protection enable for region 2.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        3 => PROTREG3: rw { //! Protection enable for region 3.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        4 => PROTREG4: rw { //! Protection enable for region 4.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        5 => PROTREG5: rw { //! Protection enable for region 5.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        6 => PROTREG6: rw { //! Protection enable for region 6.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        7 => PROTREG7: rw { //! Protection enable for region 7.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        8 => PROTREG8: rw { //! Protection enable for region 8.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        9 => PROTREG9: rw { //! Protection enable for region 9.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        10 => PROTREG10: rw { //! Protection enable for region 10.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        11 => PROTREG11: rw { //! Protection enable for region 11.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        12 => PROTREG12: rw { //! Protection enable for region 12.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        13 => PROTREG13: rw { //! Protection enable for region 13.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        14 => PROTREG14: rw { //! Protection enable for region 14.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        15 => PROTREG15: rw { //! Protection enable for region 15.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        16 => PROTREG16: rw { //! Protection enable for region 16.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        17 => PROTREG17: rw { //! Protection enable for region 17.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        18 => PROTREG18: rw { //! Protection enable for region 18.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        19 => PROTREG19: rw { //! Protection enable for region 19.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        20 => PROTREG20: rw { //! Protection enable for region 20.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        21 => PROTREG21: rw { //! Protection enable for region 21.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        22 => PROTREG22: rw { //! Protection enable for region 22.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        23 => PROTREG23: rw { //! Protection enable for region 23.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        24 => PROTREG24: rw { //! Protection enable for region 24.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        25 => PROTREG25: rw { //! Protection enable for region 25.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        26 => PROTREG26: rw { //! Protection enable for region 26.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        27 => PROTREG27: rw { //! Protection enable for region 27.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        28 => PROTREG28: rw { //! Protection enable for region 28.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        29 => PROTREG29: rw { //! Protection enable for region 29.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        30 => PROTREG30: rw { //! Protection enable for region 30.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        31 => PROTREG31: rw { //! Protection enable for region 31.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
    }
    0x604 => reg32 PROTENSET1 { //! Erase and write protection bit enable set register.
        0 => PROTREG32: rw { //! Protection enable for region 32.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        1 => PROTREG33: rw { //! Protection enable for region 33.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        2 => PROTREG34: rw { //! Protection enable for region 34.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        3 => PROTREG35: rw { //! Protection enable for region 35.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        4 => PROTREG36: rw { //! Protection enable for region 36.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        5 => PROTREG37: rw { //! Protection enable for region 37.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        6 => PROTREG38: rw { //! Protection enable for region 38.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        7 => PROTREG39: rw { //! Protection enable for region 39.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        8 => PROTREG40: rw { //! Protection enable for region 40.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        9 => PROTREG41: rw { //! Protection enable for region 41.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        10 => PROTREG42: rw { //! Protection enable for region 42.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        11 => PROTREG43: rw { //! Protection enable for region 43.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        12 => PROTREG44: rw { //! Protection enable for region 44.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        13 => PROTREG45: rw { //! Protection enable for region 45.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        14 => PROTREG46: rw { //! Protection enable for region 46.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        15 => PROTREG47: rw { //! Protection enable for region 47.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        16 => PROTREG48: rw { //! Protection enable for region 48.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        17 => PROTREG49: rw { //! Protection enable for region 49.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        18 => PROTREG50: rw { //! Protection enable for region 50.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        19 => PROTREG51: rw { //! Protection enable for region 51.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        20 => PROTREG52: rw { //! Protection enable for region 52.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        21 => PROTREG53: rw { //! Protection enable for region 53.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        22 => PROTREG54: rw { //! Protection enable for region 54.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        23 => PROTREG55: rw { //! Protection enable for region 55.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        24 => PROTREG56: rw { //! Protection enable for region 56.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        25 => PROTREG57: rw { //! Protection enable for region 57.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        26 => PROTREG58: rw { //! Protection enable for region 58.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        27 => PROTREG59: rw { //! Protection enable for region 59.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        28 => PROTREG60: rw { //! Protection enable for region 60.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        29 => PROTREG61: rw { //! Protection enable for region 61.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        30 => PROTREG62: rw { //! Protection enable for region 62.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
        31 => PROTREG63: rw { //! Protection enable for region 63.
            0 => Disabled, //= Protection disabled.
            1 => Enabled, //= Protection enabled.
        }
    }
    0x608 => reg32 DISABLEINDEBUG { //! Disable erase and write protection mechanism in debug mode.
        0 => DISABLEINDEBUG: rw { //! Disable protection mechanism in debug mode.
            0 => Enabled, //= Protection enabled.
            1 => Disabled, //= Protection disabled.
        }
    }
    0x60C => reg32 PROTBLOCKSIZE { //! Erase and write protection block size.
        0..1 => PROTBLOCKSIZE: rw { //! Erase and write protection block size.
            0 => 4k, //= Erase and write protection block size is 4k.
        }
    }
});

ioregs! (AMLI @ 0x40000000 = {
    /// AHB Multi-Layer Interface.
    0xE00 => group RAMPRI[1] { //! RAM configurable priority configuration structure.
        0x000 => reg32 CPU0 { //! Configurable priority configuration register for CPU0.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
        0x004 => reg32 SPIS1 { //! Configurable priority configuration register for SPIS1.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
        0x008 => reg32 RADIO { //! Configurable priority configuration register for RADIO.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
        0x00C => reg32 ECB { //! Configurable priority configuration register for ECB.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
        0x010 => reg32 CCM { //! Configurable priority configuration register for CCM.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
        0x014 => reg32 AAR { //! Configurable priority configuration register for AAR.
            0..3 => RAM0: rw { //! Configuration field for RAM block 0.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            4..7 => RAM1: rw { //! Configuration field for RAM block 1.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            8..11 => RAM2: rw { //! Configuration field for RAM block 2.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            12..15 => RAM3: rw { //! Configuration field for RAM block 3.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            16..19 => RAM4: rw { //! Configuration field for RAM block 4.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            20..23 => RAM5: rw { //! Configuration field for RAM block 5.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            24..27 => RAM6: rw { //! Configuration field for RAM block 6.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
            28..31 => RAM7: rw { //! Configuration field for RAM block 7.
                0x0 => Pri0, //= Priority 0.
                0x2 => Pri2, //= Priority 2.
                0x4 => Pri4, //= Priority 4.
                0x6 => Pri6, //= Priority 6.
                0x8 => Pri8, //= Priority 8.
                0xA => Pri10, //= Priority 10.
                0xC => Pri12, //= Priority 12.
                0xE => Pri14, //= Priority 14.
            }
        }
    }
});

ioregs! (RADIO @ 0x40001000 = {
    /// The radio.
    0x000 => reg32 TASKS_TXEN {} //! Enable radio in TX mode.
    0x004 => reg32 TASKS_RXEN {} //! Enable radio in RX mode.
    0x008 => reg32 TASKS_START {} //! Start radio.
    0x00C => reg32 TASKS_STOP {} //! Stop radio.
    0x010 => reg32 TASKS_DISABLE {} //! Disable radio.
    0x014 => reg32 TASKS_RSSISTART {} //! Start the RSSI and take one sample of the receive signal strength.
    0x018 => reg32 TASKS_RSSISTOP {} //! Stop the RSSI measurement.
    0x01C => reg32 TASKS_BCSTART {} //! Start the bit counter.
    0x020 => reg32 TASKS_BCSTOP {} //! Stop the bit counter.
    0x100 => reg32 EVENTS_READY {} //! Ready event.
    0x104 => reg32 EVENTS_ADDRESS {} //! Address event.
    0x108 => reg32 EVENTS_PAYLOAD {} //! Payload event.
    0x10C => reg32 EVENTS_END {} //! End event.
    0x110 => reg32 EVENTS_DISABLED {} //! Disable event.
    0x114 => reg32 EVENTS_DEVMATCH {} //! A device address match occurred on the last received packet.
    0x118 => reg32 EVENTS_DEVMISS {} //! No device address match occurred on the last received packet.
    0x11C => reg32 EVENTS_RSSIEND {} //! Sampling of the receive signal strength complete. A new RSSI sample is ready for readout at the RSSISAMPLE register.
    0x128 => reg32 EVENTS_BCMATCH {} //! Bit counter reached bit count value specified in BCC register.
    0x200 => reg32 SHORTS { //! Shortcuts for the radio.
        0 => READY_START: rw { //! Shortcut between READY event and START task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => END_DISABLE: rw { //! Shortcut between END event and DISABLE task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        2 => DISABLED_TXEN: rw { //! Shortcut between DISABLED event and TXEN task. 
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        3 => DISABLED_RXEN: rw { //! Shortcut between DISABLED event and RXEN task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        4 => ADDRESS_RSSISTART: rw { //! Shortcut between ADDRESS event and RSSISTART task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        5 => END_START: rw { //! Shortcut between END event and START task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        6 => ADDRESS_BCSTART: rw { //! Shortcut between ADDRESS event and BCSTART task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        8 => DISABLED_RSSISTOP: rw { //! Shortcut between DISABLED event and RSSISTOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => READY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ADDRESS: rw { //! Enable interrupt on ADDRESS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => PAYLOAD: rw { //! Enable interrupt on PAYLOAD event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => END: rw { //! Enable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => DISABLED: rw { //! Enable interrupt on DISABLED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        5 => DEVMATCH: rw { //! Enable interrupt on DEVMATCH event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        6 => DEVMISS: rw { //! Enable interrupt on DEVMISS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => RSSIEND: rw { //! Enable interrupt on RSSIEND event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        10 => BCMATCH: rw { //! Enable interrupt on BCMATCH event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => READY: rw { //! Disable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ADDRESS: rw { //! Disable interrupt on ADDRESS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => PAYLOAD: rw { //! Disable interrupt on PAYLOAD event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => END: rw { //! Disable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => DISABLED: rw { //! Disable interrupt on DISABLED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        5 => DEVMATCH: rw { //! Disable interrupt on DEVMATCH event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        6 => DEVMISS: rw { //! Disable interrupt on DEVMISS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => RSSIEND: rw { //! Disable interrupt on RSSIEND event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        10 => BCMATCH: rw { //! Disable interrupt on BCMATCH event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 CRCSTATUS { //! CRC status of received packet.
        0 => CRCSTATUS: ro { //! CRC status of received packet.
            0 => CRCError, //= Packet received with CRC error.
            1 => CRCOk, //= Packet received with CRC ok.
        }
    }
    0x408 => reg32 RXMATCH { //! Received address.
        0..2 => RXMATCH: ro,
    }
    0x40C => reg32 RXCRC { //! Received CRC.
        0..23 => RXCRC: ro,
    }
    0x410 => reg32 DAI { //! Device address match index.
        0..2 => DAI: ro,
    }
    0x504 => reg32 PACKETPTR {} //! Packet pointer. Decision point: START task.
    0x508 => reg32 FREQUENCY { //! Frequency.
        0..6 => FREQUENCY: rw,
    }
    0x50C => reg32 TXPOWER { //! Output power.
        0..7 => TXPOWER: rw { //! Radio output power. Decision point: TXEN task.
            0x04 => Pos4dBm, //= +4dBm.
            0x00 => 0dBm, //= 0dBm.
            0xFC => Neg4dBm, //= -4dBm.
            0xF8 => Neg8dBm, //= -8dBm.
            0xF4 => Neg12dBm, //= -12dBm.
            0xF0 => Neg16dBm, //= -16dBm.
            0xEC => Neg20dBm, //= -20dBm.
            0xD8 => Neg30dBm, //= -30dBm.
        }
    }
    0x510 => reg32 MODE { //! Data rate and modulation.
        0..1 => MODE: rw { //! Radio data rate and modulation setting. Decision point: TXEN or RXEN task.
            0x00 => Nrf_1Mbit, //= 1Mbit/s Nordic propietary radio mode.
            0x01 => Nrf_2Mbit, //= 2Mbit/s Nordic propietary radio mode.
            0x02 => Nrf_250Kbit, //= 250kbit/s Nordic propietary radio mode.
            0x03 => Ble_1Mbit, //= 1Mbit/s Bluetooth Low Energy
        }
    }
    0x514 => reg32 PCNF0 { //! Packet configuration 0.
        0..3 => LFLEN: rw,
        8 => S0LEN: rw,
        16..19 => S1LEN: rw,
    }
    0x518 => reg32 PCNF1 { //! Packet configuration 1.
        0..7 => MAXLEN: rw,
        8..15 => STATLEN: rw,
        16..18 => BALEN: rw,
        24 => ENDIAN: rw { //! On air endianness of packet length field. Decision point: START task.
            0 => Little, //= Least significant bit on air first
            1 => Big, //= Most significant bit on air first
        }
        25 => WHITEEN: rw { //! Packet whitening enable.
            0 => Disabled, //= Whitening disabled.
            1 => Enabled, //= Whitening enabled.
        }
    }
    0x51C => reg32 BASE0 {} //! Radio base address 0. Decision point: START task.
    0x520 => reg32 BASE1 {} //! Radio base address 1. Decision point: START task.
    0x524 => reg32 PREFIX0 { //! Prefixes bytes for logical addresses 0 to 3.
        0..7 => AP0: rw,
        8..15 => AP1: rw,
        16..23 => AP2: rw,
        24..31 => AP3: rw,
    }
    0x528 => reg32 PREFIX1 { //! Prefixes bytes for logical addresses 4 to 7.
        0..7 => AP4: rw,
        8..15 => AP5: rw,
        16..23 => AP6: rw,
        24..31 => AP7: rw,
    }
    0x52C => reg32 TXADDRESS { //! Transmit address select.
        0..2 => TXADDRESS: rw,
    }
    0x530 => reg32 RXADDRESSES { //! Receive address select.
        0 => ADDR0: rw { //! Enable reception on logical address 0. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        1 => ADDR1: rw { //! Enable reception on logical address 1. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        2 => ADDR2: rw { //! Enable reception on logical address 2. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        3 => ADDR3: rw { //! Enable reception on logical address 3. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        4 => ADDR4: rw { //! Enable reception on logical address 4. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        5 => ADDR5: rw { //! Enable reception on logical address 5. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        6 => ADDR6: rw { //! Enable reception on logical address 6. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
        7 => ADDR7: rw { //! Enable reception on logical address 7. Decision point: START task.
            0 => Disabled, //= Reception disabled.
            1 => Enabled, //= Reception enabled.
        }
    }
    0x534 => reg32 CRCCNF { //! CRC configuration.
        0..1 => LEN: rw { //! CRC length. Decision point: START task.
            0 => Disabled, //= CRC calculation disabled.
            1 => One, //= One byte long CRC.
            2 => Two, //= Two bytes long CRC.
            3 => Three, //= Three bytes long CRC.
        }
        8 => SKIPADDR: rw { //! Leave packet address field out of the CRC calculation. Decision point: START task.
            0 => Include, //= Include packet address in CRC calculation.
            1 => Skip, //= Packet address is skipped in CRC calculation. The CRC calculation will start at the first byte after the address.
        }
    }
    0x538 => reg32 CRCPOLY { //! CRC polynomial.
        0..23 => CRCPOLY: rw,
    }
    0x53C => reg32 CRCINIT { //! CRC initial value.
        0..23 => CRCINIT: rw,
    }
    0x540 => reg32 TEST { //! Test features enable register.
        0 => CONSTCARRIER: rw { //! Constant carrier. Decision point: TXEN task.
            0 => Disabled, //= Constant carrier disabled.
            1 => Enabled, //= Constant carrier enabled.
        }
        1 => PLLLOCK: rw { //! PLL lock. Decision point: TXEN or RXEN task.
            0 => Disabled, //= PLL lock disabled.
            1 => Enabled, //= PLL lock enabled.
        }
    }
    0x544 => reg32 TIFS { //! Inter Frame Spacing in microseconds.
        0..7 => TIFS: rw,
    }
    0x548 => reg32 RSSISAMPLE { //! RSSI sample.
        0..6 => RSSISAMPLE: ro,
    }
    0x550 => reg32 STATE { //! Current radio state.
        0..3 => STATE: ro { //! Current radio state.
            0x00 => Disabled, //= Radio is in the Disabled state.
            0x01 => RxRu, //= Radio is in the Rx Ramp Up state.
            0x02 => RxIdle, //= Radio is in the Rx Idle state.
            0x03 => Rx, //= Radio is in the Rx state.
            0x04 => RxDisable, //= Radio is in the Rx Disable state.
            0x09 => TxRu, //= Radio is in the Tx Ramp Up state.
            0x0A => TxIdle, //= Radio is in the Tx Idle state.
            0x0B => Tx, //= Radio is in the Tx state.
            0x0C => TxDisable, //= Radio is in the Tx Disable state.
        }
    }
    0x554 => reg32 DATAWHITEIV { //! Data whitening initial value.
        0..6 => DATAWHITEIV: rw,
    }
    0x560 => reg32 BCC {} //! Bit counter compare.
    0x600 => group DAB[8] { //! Device address base segment.
        0x0 => reg32 DAB {} //! Device address base segment.
    }
    0x620 => group DAP[8] { //! Device address prefix.
        0x0 => reg32 DAP { //! Device address prefix.
            0..15 => DAP: rw,
        }
    }
    0x640 => reg32 DACNF { //! Device address match configuration.
        0 => ENA0: rw { //! Enable or disable device address matching using device address 0.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        1 => ENA1: rw { //! Enable or disable device address matching using device address 1.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        2 => ENA2: rw { //! Enable or disable device address matching using device address 2.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        3 => ENA3: rw { //! Enable or disable device address matching using device address 3.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        4 => ENA4: rw { //! Enable or disable device address matching using device address 4.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        5 => ENA5: rw { //! Enable or disable device address matching using device address 5.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        6 => ENA6: rw { //! Enable or disable device address matching using device address 6.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
        }
        7 => ENA7: rw { //! Enable or disable device address matching using device address 7.
            0 => Disabled, //= Disabled.
            1 => Enabled, //= Enabled.
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
    0x724 => reg32 OVERRIDE0 { //! Trim value override register 0.
        0..31 => OVERRIDE0: rw,
    }
    0x728 => reg32 OVERRIDE1 { //! Trim value override register 1.
        0..31 => OVERRIDE1: rw,
    }
    0x72C => reg32 OVERRIDE2 { //! Trim value override register 2.
        0..31 => OVERRIDE2: rw,
    }
    0x730 => reg32 OVERRIDE3 { //! Trim value override register 3.
        0..31 => OVERRIDE3: rw,
    }
    0x734 => reg32 OVERRIDE4 { //! Trim value override register 4.
        0..27 => OVERRIDE4: rw,
        31 => ENABLE: rw { //! Enable or disable override of default trim values.
            0 => Disabled, //= Override trim values disabled.
            1 => Enabled, //= Override trim values enabled.
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (UART0 @ 0x40002000 = {
    /// Universal Asynchronous Receiver/Transmitter.
    0x000 => reg32 TASKS_STARTRX {} //! Start UART receiver.
    0x004 => reg32 TASKS_STOPRX {} //! Stop UART receiver.
    0x008 => reg32 TASKS_STARTTX {} //! Start UART transmitter.
    0x00C => reg32 TASKS_STOPTX {} //! Stop UART transmitter.
    0x01C => reg32 TASKS_SUSPEND {} //! Suspend UART.
    0x100 => reg32 EVENTS_CTS {} //! CTS activated.
    0x104 => reg32 EVENTS_NCTS {} //! CTS deactivated.
    0x108 => reg32 EVENTS_RXDRDY {} //! Data received in RXD.
    0x11C => reg32 EVENTS_TXDRDY {} //! Data sent from TXD.
    0x124 => reg32 EVENTS_ERROR {} //! Error detected.
    0x144 => reg32 EVENTS_RXTO {} //! Receiver timeout.
    0x200 => reg32 SHORTS { //! Shortcuts for UART.
        3 => CTS_STARTRX: rw { //! Shortcut between CTS event and STARTRX task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        4 => NCTS_STOPRX: rw { //! Shortcut between NCTS event and STOPRX task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => CTS: rw { //! Enable interrupt on CTS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => NCTS: rw { //! Enable interrupt on NCTS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDRDY: rw { //! Enable interrupt on RXRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDRDY: rw { //! Enable interrupt on TXRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Enable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => RXTO: rw { //! Enable interrupt on RXTO event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => CTS: rw { //! Disable interrupt on CTS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => NCTS: rw { //! Disable interrupt on NCTS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDRDY: rw { //! Disable interrupt on RXRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDRDY: rw { //! Disable interrupt on TXRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Disable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => RXTO: rw { //! Disable interrupt on RXTO event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x480 => reg32 ERRORSRC { //! Error source. Write error field to 1 to clear error.
        0 => OVERRUN: rw { //! A start bit is received while the previous data still lies in RXD. (Data loss).
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        1 => PARITY: rw { //! A character with bad parity is received. Only checked if HW parity control is enabled.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        2 => FRAMING: rw { //! A valid stop bit is not detected on the serial data input after all bits in a character have been received.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        3 => BREAK: rw { //! The serial data input is '0' for longer than the length of a data frame.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
    }
    0x500 => reg32 ENABLE { //! Enable UART and acquire IOs.
        0..2 => ENABLE: rw { //! Enable or disable UART and acquire IOs.
            0x00 => Disabled, //= UART disabled.
            0x04 => Enabled, //= UART enabled.
        }
    }
    0x508 => reg32 PSELRTS {} //! Pin select for RTS.
    0x50C => reg32 PSELTXD {} //! Pin select for TXD.
    0x510 => reg32 PSELCTS {} //! Pin select for CTS.
    0x514 => reg32 PSELRXD {} //! Pin select for RXD.
    0x518 => reg32 RXD { //! RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working.
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD { //! TXD register.
        0..7 => TXD: wo,
    }
    0x524 => reg32 BAUDRATE { //! UART Baudrate.
        0..31 => BAUDRATE: rw { //! UART baudrate.
            0x0004F000 => Baud1200, //= 1200 baud.
            0x0009D000 => Baud2400, //= 2400 baud.
            0x0013B000 => Baud4800, //= 4800 baud.
            0x00275000 => Baud9600, //= 9600 baud.
            0x003B0000 => Baud14400, //= 14400 baud.
            0x004EA000 => Baud19200, //= 19200 baud.
            0x0075F000 => Baud28800, //= 28800 baud.
            0x009D5000 => Baud38400, //= 38400 baud.
            0x00EBF000 => Baud57600, //= 57600 baud.
            0x013A9000 => Baud76800, //= 76800 baud.
            0x01D7E000 => Baud115200, //= 115200 baud.
            0x03AFB000 => Baud230400, //= 230400 baud.
            0x04000000 => Baud250000, //= 250000 baud.
            0x075F7000 => Baud460800, //= 460800 baud.
            0x0EBED000 => Baud921600, //= 921600 baud.
            0x10000000 => Baud1M, //= 1M baud.
        }
    }
    0x56C => reg32 CONFIG { //! Configuration of parity and hardware flow control register.
        0 => HWFC: rw { //! Hardware flow control.
            0 => Disabled, //= Hardware flow control disabled.
            1 => Enabled, //= Hardware flow control enabled.
        }
        1..3 => PARITY: rw { //! Include parity bit.
            0 => Excluded, //= Parity bit excluded.
            7 => Included, //= Parity bit included.
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (SPI0 @ 0x40003000 = {
    /// SPI master 0.
    0x108 => reg32 EVENTS_READY {} //! TXD byte sent and RXD byte received.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        2 => READY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        2 => READY: rw { //! Disable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x500 => reg32 ENABLE { //! Enable SPI.
        0..2 => ENABLE: rw { //! Enable or disable SPI.
            0x00 => Disabled, //= Disabled SPI.
            0x01 => Enabled, //= Enable SPI.
        }
    }
    0x508 => reg32 PSELSCK {} //! Pin select for SCK.
    0x50C => reg32 PSELMOSI {} //! Pin select for MOSI.
    0x510 => reg32 PSELMISO {} //! Pin select for MISO.
    0x518 => reg32 RXD { //! RX data.
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD { //! TX data.
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY { //! SPI frequency
        0..31 => FREQUENCY: rw { //! SPI data rate.
            0x02000000 => K125, //= 125kbps.
            0x04000000 => K250, //= 250kbps.
            0x08000000 => K500, //= 500kbps.
            0x10000000 => M1, //= 1Mbps.
            0x20000000 => M2, //= 2Mbps.
            0x40000000 => M4, //= 4Mbps.
            0x80000000 => M8, //= 8Mbps.
        }
    }
    0x554 => reg32 CONFIG { //! Configuration register.
        0 => ORDER: rw { //! Bit order.
            0 => MsbFirst, //= Most significant bit transmitted out first.
            1 => LsbFirst, //= Least significant bit transmitted out first.
        }
        1 => CPHA: rw { //! Serial clock (SCK) phase.
            0 => Leading, //= Sample on leading edge of the clock. Shift serial data on trailing edge.
            1 => Trailing, //= Sample on trailing edge of the clock. Shift serial data on leading edge.
        }
        2 => CPOL: rw { //! Serial clock (SCK) polarity.
            0 => ActiveHigh, //= Active high.
            1 => ActiveLow, //= Active low.
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TWI0 @ 0x40003000 = {
    /// Two-wire interface master 0.
    0x000 => reg32 TASKS_STARTRX {} //! Start 2-Wire master receive sequence.
    0x008 => reg32 TASKS_STARTTX {} //! Start 2-Wire master transmit sequence.
    0x014 => reg32 TASKS_STOP {} //! Stop 2-Wire transaction.
    0x01C => reg32 TASKS_SUSPEND {} //! Suspend 2-Wire transaction.
    0x020 => reg32 TASKS_RESUME {} //! Resume 2-Wire transaction.
    0x104 => reg32 EVENTS_STOPPED {} //! Two-wire stopped.
    0x108 => reg32 EVENTS_RXDREADY {} //! Two-wire ready to deliver new RXD byte received.
    0x11C => reg32 EVENTS_TXDSENT {} //! Two-wire finished sending last TXD byte.
    0x124 => reg32 EVENTS_ERROR {} //! Two-wire error detected.
    0x138 => reg32 EVENTS_BB {} //! Two-wire byte boundary.
    0x148 => reg32 EVENTS_SUSPENDED {} //! Two-wire suspended.
    0x200 => reg32 SHORTS { //! Shortcuts for TWI.
        0 => BB_SUSPEND: rw { //! Shortcut between BB event and the SUSPEND task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => BB_STOP: rw { //! Shortcut between BB event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        1 => STOPPED: rw { //! Enable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDREADY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDSENT: rw { //! Enable interrupt on TXDSENT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Enable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        14 => BB: rw { //! Enable interrupt on BB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => SUSPENDED: rw { //! Enable interrupt on SUSPENDED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        1 => STOPPED: rw { //! Disable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDREADY: rw { //! Disable interrupt on RXDREADY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDSENT: rw { //! Disable interrupt on TXDSENT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Disable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        14 => BB: rw { //! Disable interrupt on BB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => SUSPENDED: rw { //! Disable interrupt on SUSPENDED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x4C4 => reg32 ERRORSRC { //! Two-wire error source. Write error field to 1 to clear error.
        0 => OVERRUN: rw { //! Byte received in RXD register before read of the last received byte (data loss).
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        1 => ANACK: rw { //! NACK received after sending the address.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        2 => DNACK: rw { //! NACK received after sending a data byte.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
    }
    0x500 => reg32 ENABLE { //! Enable two-wire master.
        0..2 => ENABLE: rw { //! Enable or disable W2M
            0x00 => Disabled, //= Disabled.
            0x05 => Enabled, //= Enabled.
        }
    }
    0x508 => reg32 PSELSCL {} //! Pin select for SCL.
    0x50C => reg32 PSELSDA {} //! Pin select for SDA.
    0x518 => reg32 RXD { //! RX data register.
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD { //! TX data register.
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY { //! Two-wire frequency.
        0..31 => FREQUENCY: rw { //! Two-wire master clock frequency.
            0x01980000 => K100, //= 100 kbps.
            0x04000000 => K250, //= 250 kbps.
            0x06680000 => K400, //= 400 kbps.
        }
    }
    0x588 => reg32 ADDRESS { //! Address used in the two-wire transfer.
        0..6 => ADDRESS: rw,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (SPI1 @ 0x40004000 = {
    /// SPI master 1.
    0x108 => reg32 EVENTS_READY {} //! TXD byte sent and RXD byte received.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        2 => READY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        2 => READY: rw { //! Disable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x500 => reg32 ENABLE { //! Enable SPI.
        0..2 => ENABLE: rw { //! Enable or disable SPI.
            0x00 => Disabled, //= Disabled SPI.
            0x01 => Enabled, //= Enable SPI.
        }
    }
    0x508 => reg32 PSELSCK {} //! Pin select for SCK.
    0x50C => reg32 PSELMOSI {} //! Pin select for MOSI.
    0x510 => reg32 PSELMISO {} //! Pin select for MISO.
    0x518 => reg32 RXD { //! RX data.
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD { //! TX data.
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY { //! SPI frequency
        0..31 => FREQUENCY: rw { //! SPI data rate.
            0x02000000 => K125, //= 125kbps.
            0x04000000 => K250, //= 250kbps.
            0x08000000 => K500, //= 500kbps.
            0x10000000 => M1, //= 1Mbps.
            0x20000000 => M2, //= 2Mbps.
            0x40000000 => M4, //= 4Mbps.
            0x80000000 => M8, //= 8Mbps.
        }
    }
    0x554 => reg32 CONFIG { //! Configuration register.
        0 => ORDER: rw { //! Bit order.
            0 => MsbFirst, //= Most significant bit transmitted out first.
            1 => LsbFirst, //= Least significant bit transmitted out first.
        }
        1 => CPHA: rw { //! Serial clock (SCK) phase.
            0 => Leading, //= Sample on leading edge of the clock. Shift serial data on trailing edge.
            1 => Trailing, //= Sample on trailing edge of the clock. Shift serial data on leading edge.
        }
        2 => CPOL: rw { //! Serial clock (SCK) polarity.
            0 => ActiveHigh, //= Active high.
            1 => ActiveLow, //= Active low.
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TWI1 @ 0x40004000 = {
    /// Two-wire interface master 1.
    0x000 => reg32 TASKS_STARTRX {} //! Start 2-Wire master receive sequence.
    0x008 => reg32 TASKS_STARTTX {} //! Start 2-Wire master transmit sequence.
    0x014 => reg32 TASKS_STOP {} //! Stop 2-Wire transaction.
    0x01C => reg32 TASKS_SUSPEND {} //! Suspend 2-Wire transaction.
    0x020 => reg32 TASKS_RESUME {} //! Resume 2-Wire transaction.
    0x104 => reg32 EVENTS_STOPPED {} //! Two-wire stopped.
    0x108 => reg32 EVENTS_RXDREADY {} //! Two-wire ready to deliver new RXD byte received.
    0x11C => reg32 EVENTS_TXDSENT {} //! Two-wire finished sending last TXD byte.
    0x124 => reg32 EVENTS_ERROR {} //! Two-wire error detected.
    0x138 => reg32 EVENTS_BB {} //! Two-wire byte boundary.
    0x148 => reg32 EVENTS_SUSPENDED {} //! Two-wire suspended.
    0x200 => reg32 SHORTS { //! Shortcuts for TWI.
        0 => BB_SUSPEND: rw { //! Shortcut between BB event and the SUSPEND task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => BB_STOP: rw { //! Shortcut between BB event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        1 => STOPPED: rw { //! Enable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDREADY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDSENT: rw { //! Enable interrupt on TXDSENT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Enable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        14 => BB: rw { //! Enable interrupt on BB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => SUSPENDED: rw { //! Enable interrupt on SUSPENDED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        1 => STOPPED: rw { //! Disable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => RXDREADY: rw { //! Disable interrupt on RXDREADY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        7 => TXDSENT: rw { //! Disable interrupt on TXDSENT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        9 => ERROR: rw { //! Disable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        14 => BB: rw { //! Disable interrupt on BB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => SUSPENDED: rw { //! Disable interrupt on SUSPENDED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x4C4 => reg32 ERRORSRC { //! Two-wire error source. Write error field to 1 to clear error.
        0 => OVERRUN: rw { //! Byte received in RXD register before read of the last received byte (data loss).
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        1 => ANACK: rw { //! NACK received after sending the address.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        2 => DNACK: rw { //! NACK received after sending a data byte.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
    }
    0x500 => reg32 ENABLE { //! Enable two-wire master.
        0..2 => ENABLE: rw { //! Enable or disable W2M
            0x00 => Disabled, //= Disabled.
            0x05 => Enabled, //= Enabled.
        }
    }
    0x508 => reg32 PSELSCL {} //! Pin select for SCL.
    0x50C => reg32 PSELSDA {} //! Pin select for SDA.
    0x518 => reg32 RXD { //! RX data register.
        0..7 => RXD: ro,
    }
    0x51C => reg32 TXD { //! TX data register.
        0..7 => TXD: rw,
    }
    0x524 => reg32 FREQUENCY { //! Two-wire frequency.
        0..31 => FREQUENCY: rw { //! Two-wire master clock frequency.
            0x01980000 => K100, //= 100 kbps.
            0x04000000 => K250, //= 250 kbps.
            0x06680000 => K400, //= 400 kbps.
        }
    }
    0x588 => reg32 ADDRESS { //! Address used in the two-wire transfer.
        0..6 => ADDRESS: rw,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (SPIS1 @ 0x40004000 = {
    /// SPI slave 1.
    0x024 => reg32 TASKS_ACQUIRE {} //! Acquire SPI semaphore.
    0x028 => reg32 TASKS_RELEASE {} //! Release SPI semaphore.
    0x104 => reg32 EVENTS_END {} //! Granted transaction completed.
    0x128 => reg32 EVENTS_ACQUIRED {} //! Semaphore acquired.
    0x200 => reg32 SHORTS { //! Shortcuts for SPIS.
        2 => END_ACQUIRE: rw { //! Shortcut between END event and the ACQUIRE task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        1 => END: rw { //! Enable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        10 => ACQUIRED: rw { //! Enable interrupt on ACQUIRED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        1 => END: rw { //! Disable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        10 => ACQUIRED: rw { //! Disable interrupt on ACQUIRED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 SEMSTAT { //! Semaphore status.
        0..1 => SEMSTAT: ro { //! Semaphore status.
            0x00 => Free, //= Semaphore is free.
            0x01 => CPU, //= Semaphore is assigned to the CPU.
            0x02 => SPIS, //= Semaphore is assigned to the SPIS.
            0x03 => CPUPending, //= Semaphore is assigned to the SPIS, but a handover to the CPU is pending.
        }
    }
    0x440 => reg32 STATUS { //! Status from last transaction.
        0 => OVERREAD: rw { //! TX buffer overread detected, and prevented.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
        1 => OVERFLOW: rw { //! RX buffer overflow detected, and prevented.
            0 => NotPresent, //= Error not present.
            1 => Present, //= Error present.
        }
    }
    0x500 => reg32 ENABLE { //! Enable SPIS.
        0..2 => ENABLE: rw { //! Enable or disable SPIS.
            0x00 => Disabled, //= Disabled SPIS.
            0x02 => Enabled, //= Enable SPIS.
        }
    }
    0x508 => reg32 PSELSCK {} //! Pin select for SCK.
    0x50C => reg32 PSELMISO {} //! Pin select for MISO.
    0x510 => reg32 PSELMOSI {} //! Pin select for MOSI.
    0x514 => reg32 PSELCSN {} //! Pin select for CSN.
    0x534 => reg32 RXDPTR {} //! RX data pointer.
    0x538 => reg32 MAXRX { //! Maximum number of bytes in the receive buffer.
        0..7 => MAXRX: rw,
    }
    0x53C => reg32 AMOUNTRX { //! Number of bytes received in last granted transaction.
        0..7 => AMOUNTRX: ro,
    }
    0x544 => reg32 TXDPTR {} //! TX data pointer.
    0x548 => reg32 MAXTX { //! Maximum number of bytes in the transmit buffer.
        0..7 => MAXTX: rw,
    }
    0x54C => reg32 AMOUNTTX { //! Number of bytes transmitted in last granted transaction.
        0..7 => AMOUNTTX: ro,
    }
    0x554 => reg32 CONFIG { //! Configuration register.
        0 => ORDER: rw { //! Bit order.
            0 => MsbFirst, //= Most significant bit transmitted out first.
            1 => LsbFirst, //= Least significant bit transmitted out first.
        }
        1 => CPHA: rw { //! Serial clock (SCK) phase.
            0 => Leading, //= Sample on leading edge of the clock. Shift serial data on trailing edge.
            1 => Trailing, //= Sample on trailing edge of the clock. Shift serial data on leading edge.
        }
        2 => CPOL: rw { //! Serial clock (SCK) polarity.
            0 => ActiveHigh, //= Active high.
            1 => ActiveLow, //= Active low.
        }
    }
    0x55C => reg32 DEF { //! Default character.
        0..7 => DEF: rw,
    }
    0x5C0 => reg32 ORC { //! Over-read character.
        0..7 => ORC: rw,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (SPIM1 @ 0x40004000 = {
    /// SPI master with easyDMA 1.
    0x010 => reg32 TASKS_START {} //! Start SPI transaction.
    0x014 => reg32 TASKS_STOP {} //! Stop SPI transaction.
    0x01C => reg32 TASKS_SUSPEND {} //! Suspend SPI transaction.
    0x020 => reg32 TASKS_RESUME {} //! Resume SPI transaction.
    0x104 => reg32 EVENTS_STOPPED {} //! SPI transaction has stopped.
    0x110 => reg32 EVENTS_ENDRX {} //! End of RXD buffer reached.
    0x120 => reg32 EVENTS_ENDTX {} //! End of TXD buffer reached.
    0x14C => reg32 EVENTS_STARTED {} //! Transaction started.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        1 => STOPPED: rw { //! Enable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => ENDRX: rw { //! Enable interrupt on ENDRX event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        8 => ENDTX: rw { //! Enable interrupt on ENDTX event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => STARTED: rw { //! Enable interrupt on STARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        1 => STOPPED: rw { //! Disable interrupt on STOPPED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        4 => ENDRX: rw { //! Disable interrupt on ENDRX event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        8 => ENDTX: rw { //! Disable interrupt on ENDTX event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => STARTED: rw { //! Disable interrupt on STARTED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x500 => reg32 ENABLE { //! Enable SPIM.
        0..3 => ENABLE: rw { //! Enable or disable SPIM.
            0x00 => Disabled, //= Disabled SPIM.
            0x07 => Enabled, //= Enable SPIM.
        }
    }
    0x508 => group PSEL[1] { //! Pin select configuration.
        0x0 => reg32 SCK {} //! Pin select for SCK.
        0x4 => reg32 MOSI {} //! Pin select for MOSI.
        0x8 => reg32 MISO {} //! Pin select for MISO.
    }
    0x524 => reg32 FREQUENCY { //! SPI frequency.
        0..31 => FREQUENCY: rw { //! SPI master data rate.
            0x02000000 => K125, //= 125 kbps.
            0x04000000 => K250, //= 250 kbps.
            0x08000000 => K500, //= 500 kbps.
            0x10000000 => M1, //= 1 Mbps.
            0x20000000 => M2, //= 2 Mbps.
            0x40000000 => M4, //= 4 Mbps.
            0x80000000 => M8, //= 8 Mbps.
        }
    }
    0x534 => group RXD[1] { //! RXD EasyDMA configuration and status.
        0x0 => reg32 PTR { //! Data pointer.
            0..31 => PTR: rw,
        }
        0x4 => reg32 MAXCNT { //! Maximum number of buffer bytes to receive.
            0..7 => MAXCNT: rw,
        }
        0x8 => reg32 AMOUNT { //! Number of bytes received in the last transaction.
            0..7 => AMOUNT: ro,
        }
    }
    0x544 => group TXD[1] { //! TXD EasyDMA configuration and status.
        0x0 => reg32 PTR { //! Data pointer.
            0..31 => PTR: rw,
        }
        0x4 => reg32 MAXCNT { //! Maximum number of buffer bytes to send.
            0..7 => MAXCNT: rw,
        }
        0x8 => reg32 AMOUNT { //! Number of bytes sent in the last transaction.
            0..7 => AMOUNT: ro,
        }
    }
    0x554 => reg32 CONFIG { //! Configuration register.
        0 => ORDER: rw { //! Bit order.
            0 => MsbFirst, //= Most significant bit transmitted out first.
            1 => LsbFirst, //= Least significant bit transmitted out first.
        }
        1 => CPHA: rw { //! Serial clock (SCK) phase.
            0 => Leading, //= Sample on leading edge of the clock. Shift serial data on trailing edge.
            1 => Trailing, //= Sample on trailing edge of the clock. Shift serial data on leading edge.
        }
        2 => CPOL: rw { //! Serial clock (SCK) polarity.
            0 => ActiveHigh, //= Active high.
            1 => ActiveLow, //= Active low.
        }
    }
    0x5C0 => reg32 ORC { //! Over-read character.
        0..7 => ORC: rw,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (GPIOTE @ 0x40006000 = {
    /// GPIO tasks and events.
    0x000 => group TASKS_OUT[4] { //! Tasks asssociated with GPIOTE channels.
        0x0 => reg32 TASKS_OUT {} //! Tasks asssociated with GPIOTE channels.
    }
    0x100 => group EVENTS_IN[4] { //! Tasks asssociated with GPIOTE channels.
        0x0 => reg32 EVENTS_IN {} //! Tasks asssociated with GPIOTE channels.
    }
    0x17C => reg32 EVENTS_PORT {} //! Event generated from multiple pins.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => IN0: rw { //! Enable interrupt on IN[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => IN1: rw { //! Enable interrupt on IN[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => IN2: rw { //! Enable interrupt on IN[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => IN3: rw { //! Enable interrupt on IN[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        31 => PORT: rw { //! Enable interrupt on PORT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => IN0: rw { //! Disable interrupt on IN[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => IN1: rw { //! Disable interrupt on IN[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => IN2: rw { //! Disable interrupt on IN[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => IN3: rw { //! Disable interrupt on IN[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        31 => PORT: rw { //! Disable interrupt on PORT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x510 => group CONFIG[4] { //! Channel configuration registers.
        0x0 => reg32 CONFIG { //! Channel configuration registers.
            0..1 => MODE: rw { //! Mode
                0x00 => Disabled, //= Disabled.
                0x01 => Event, //= Channel configure in event mode.
                0x03 => Task, //= Channel configure in task mode.
            }
            8..12 => PSEL: rw,
            16..17 => POLARITY: rw { //! Effects on output when in Task mode, or events on input that generates an event.
                0x00 => None, //= No task or event.
                0x01 => LoToHi, //= Low to high.
                0x02 => HiToLo, //= High to low.
                0x03 => Toggle, //= Toggle.
            }
            20 => OUTINIT: rw { //! Initial value of the output when the GPIOTE channel is configured as a Task.
                0 => Low, //= Initial low output when in task mode.
                1 => High, //= Initial high output when in task mode.
            }
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (ADC @ 0x40007000 = {
    /// Analog to digital converter.
    0x000 => reg32 TASKS_START {} //! Start an ADC conversion.
    0x004 => reg32 TASKS_STOP {} //! Stop ADC.
    0x100 => reg32 EVENTS_END {} //! ADC conversion complete.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => END: rw { //! Enable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => END: rw { //! Disable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 BUSY { //! ADC busy register.
        0 => BUSY: ro { //! ADC busy register.
            0 => Ready, //= No ongoing ADC conversion is taking place. ADC is ready.
            1 => Busy, //= An ADC conversion is taking place. ADC is busy.
        }
    }
    0x500 => reg32 ENABLE { //! ADC enable.
        0..1 => ENABLE: rw { //! ADC enable.
            0x00 => Disabled, //= ADC is disabled.
            0x01 => Enabled, //= ADC is enabled. If an analog input pin is selected as source of the conversion, the selected pin is configured as an analog input.
        }
    }
    0x504 => reg32 CONFIG { //! ADC configuration register.
        0..1 => RES: rw { //! ADC resolution.
            0x00 => 8bit, //= 8bit ADC resolution.
            0x01 => 9bit, //= 9bit ADC resolution.
            0x02 => 10bit, //= 10bit ADC resolution.
        }
        2..4 => INPSEL: rw { //! ADC input selection.
            0x00 => AnalogInputNoPrescaling, //= Analog input specified by PSEL with no prescaling used as input for the conversion.
            0x01 => AnalogInputTwoThirdsPrescaling, //= Analog input specified by PSEL with 2/3 prescaling used as input for the conversion.
            0x02 => AnalogInputOneThirdPrescaling, //= Analog input specified by PSEL with 1/3 prescaling used as input for the conversion.
            0x05 => SupplyTwoThirdsPrescaling, //= Supply voltage with 2/3 prescaling used as input for the conversion.
            0x06 => SupplyOneThirdPrescaling, //= Supply voltage with 1/3 prescaling used as input for the conversion.
        }
        5..6 => REFSEL: rw { //! ADC reference selection.
            0x00 => VBG, //= Use internal 1.2V bandgap voltage as reference for conversion.
            0x01 => External, //= Use external source configured by EXTREFSEL as reference for conversion.
            0x02 => SupplyOneHalfPrescaling, //= Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V.
            0x03 => SupplyOneThirdPrescaling, //= Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V.
        }
        8..15 => PSEL: rw { //! ADC analog pin selection.
            0 => Disabled, //= Analog input pins disabled.
            1 => AnalogInput0, //= Use analog input 0 as analog input.
            2 => AnalogInput1, //= Use analog input 1 as analog input.
            4 => AnalogInput2, //= Use analog input 2 as analog input.
            8 => AnalogInput3, //= Use analog input 3 as analog input.
            16 => AnalogInput4, //= Use analog input 4 as analog input.
            32 => AnalogInput5, //= Use analog input 5 as analog input.
            64 => AnalogInput6, //= Use analog input 6 as analog input.
            128 => AnalogInput7, //= Use analog input 7 as analog input.
        }
        16..17 => EXTREFSEL: rw { //! ADC external reference pin selection.
            0 => None, //= Analog external reference inputs disabled.
            1 => AnalogReference0, //= Use analog reference 0 as reference.
            2 => AnalogReference1, //= Use analog reference 1 as reference.
        }
    }
    0x508 => reg32 RESULT { //! Result of ADC conversion.
        0..9 => RESULT: ro,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TIMER0 @ 0x40008000 = {
    /// Timer 0.
    0x000 => reg32 TASKS_START {} //! Start Timer.
    0x004 => reg32 TASKS_STOP {} //! Stop Timer.
    0x008 => reg32 TASKS_COUNT {} //! Increment Timer (In counter mode).
    0x00C => reg32 TASKS_CLEAR {} //! Clear timer.
    0x010 => reg32 TASKS_SHUTDOWN {} //! Shutdown timer.
    0x040 => group TASKS_CAPTURE[4] { //! Capture Timer value to CC[n] registers.
        0x0 => reg32 TASKS_CAPTURE {} //! Capture Timer value to CC[n] registers.
    }
    0x140 => group EVENTS_COMPARE[4] { //! Compare event on CC[n] match.
        0x0 => reg32 EVENTS_COMPARE {} //! Compare event on CC[n] match.
    }
    0x200 => reg32 SHORTS { //! Shortcuts for Timer.
        0 => COMPARE0_CLEAR: rw { //! Shortcut between CC[0] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => COMPARE1_CLEAR: rw { //! Shortcut between CC[1] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        2 => COMPARE2_CLEAR: rw { //! Shortcut between CC[2] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        3 => COMPARE3_CLEAR: rw { //! Shortcut between CC[3] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        8 => COMPARE0_STOP: rw { //! Shortcut between CC[0] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        9 => COMPARE1_STOP: rw { //! Shortcut between CC[1] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        10 => COMPARE2_STOP: rw { //! Shortcut between CC[2] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        11 => COMPARE3_STOP: rw { //! Shortcut between CC[3] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        16 => COMPARE0: rw { //! Enable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Enable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Enable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Enable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        16 => COMPARE0: rw { //! Disable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Disable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Disable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Disable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x504 => reg32 MODE { //! Timer Mode selection.
        0 => MODE: rw { //! Select Normal or Counter mode.
            1 => Counter, //= Timer in Counter mode.
            0 => Timer, //= Timer in Normal mode.
        }
    }
    0x508 => reg32 BITMODE { //! Sets timer behaviour.
        0..1 => BITMODE: rw { //! Sets timer behaviour ro be like the implementation of a timer with width as indicated.
            0x00 => 16Bit, //= 16-bit timer behaviour.
            0x01 => 08Bit, //= 8-bit timer behaviour.
            0x02 => 24Bit, //= 24-bit timer behaviour.
            0x03 => 32Bit, //= 32-bit timer behaviour.
        }
    }
    0x510 => reg32 PRESCALER { //! 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE.
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] { //! Capture/compare registers.
        0x0 => reg32 CC {} //! Capture/compare registers.
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TIMER1 @ 0x40009000 = {
    /// Timer 1.
    0x000 => reg32 TASKS_START {} //! Start Timer.
    0x004 => reg32 TASKS_STOP {} //! Stop Timer.
    0x008 => reg32 TASKS_COUNT {} //! Increment Timer (In counter mode).
    0x00C => reg32 TASKS_CLEAR {} //! Clear timer.
    0x010 => reg32 TASKS_SHUTDOWN {} //! Shutdown timer.
    0x040 => group TASKS_CAPTURE[4] { //! Capture Timer value to CC[n] registers.
        0x0 => reg32 TASKS_CAPTURE {} //! Capture Timer value to CC[n] registers.
    }
    0x140 => group EVENTS_COMPARE[4] { //! Compare event on CC[n] match.
        0x0 => reg32 EVENTS_COMPARE {} //! Compare event on CC[n] match.
    }
    0x200 => reg32 SHORTS { //! Shortcuts for Timer.
        0 => COMPARE0_CLEAR: rw { //! Shortcut between CC[0] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => COMPARE1_CLEAR: rw { //! Shortcut between CC[1] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        2 => COMPARE2_CLEAR: rw { //! Shortcut between CC[2] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        3 => COMPARE3_CLEAR: rw { //! Shortcut between CC[3] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        8 => COMPARE0_STOP: rw { //! Shortcut between CC[0] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        9 => COMPARE1_STOP: rw { //! Shortcut between CC[1] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        10 => COMPARE2_STOP: rw { //! Shortcut between CC[2] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        11 => COMPARE3_STOP: rw { //! Shortcut between CC[3] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        16 => COMPARE0: rw { //! Enable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Enable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Enable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Enable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        16 => COMPARE0: rw { //! Disable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Disable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Disable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Disable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x504 => reg32 MODE { //! Timer Mode selection.
        0 => MODE: rw { //! Select Normal or Counter mode.
            1 => Counter, //= Timer in Counter mode.
            0 => Timer, //= Timer in Normal mode.
        }
    }
    0x508 => reg32 BITMODE { //! Sets timer behaviour.
        0..1 => BITMODE: rw { //! Sets timer behaviour ro be like the implementation of a timer with width as indicated.
            0x00 => 16Bit, //= 16-bit timer behaviour.
            0x01 => 08Bit, //= 8-bit timer behaviour.
            0x02 => 24Bit, //= 24-bit timer behaviour.
            0x03 => 32Bit, //= 32-bit timer behaviour.
        }
    }
    0x510 => reg32 PRESCALER { //! 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE.
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] { //! Capture/compare registers.
        0x0 => reg32 CC {} //! Capture/compare registers.
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TIMER2 @ 0x4000A000 = {
    /// Timer 2.
    0x000 => reg32 TASKS_START {} //! Start Timer.
    0x004 => reg32 TASKS_STOP {} //! Stop Timer.
    0x008 => reg32 TASKS_COUNT {} //! Increment Timer (In counter mode).
    0x00C => reg32 TASKS_CLEAR {} //! Clear timer.
    0x010 => reg32 TASKS_SHUTDOWN {} //! Shutdown timer.
    0x040 => group TASKS_CAPTURE[4] { //! Capture Timer value to CC[n] registers.
        0x0 => reg32 TASKS_CAPTURE {} //! Capture Timer value to CC[n] registers.
    }
    0x140 => group EVENTS_COMPARE[4] { //! Compare event on CC[n] match.
        0x0 => reg32 EVENTS_COMPARE {} //! Compare event on CC[n] match.
    }
    0x200 => reg32 SHORTS { //! Shortcuts for Timer.
        0 => COMPARE0_CLEAR: rw { //! Shortcut between CC[0] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => COMPARE1_CLEAR: rw { //! Shortcut between CC[1] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        2 => COMPARE2_CLEAR: rw { //! Shortcut between CC[2] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        3 => COMPARE3_CLEAR: rw { //! Shortcut between CC[3] event and the CLEAR task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        8 => COMPARE0_STOP: rw { //! Shortcut between CC[0] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        9 => COMPARE1_STOP: rw { //! Shortcut between CC[1] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        10 => COMPARE2_STOP: rw { //! Shortcut between CC[2] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        11 => COMPARE3_STOP: rw { //! Shortcut between CC[3] event and the STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        16 => COMPARE0: rw { //! Enable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Enable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Enable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Enable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        16 => COMPARE0: rw { //! Disable interrupt on COMPARE[0]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Disable interrupt on COMPARE[1]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Disable interrupt on COMPARE[2]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Disable interrupt on COMPARE[3]
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x504 => reg32 MODE { //! Timer Mode selection.
        0 => MODE: rw { //! Select Normal or Counter mode.
            1 => Counter, //= Timer in Counter mode.
            0 => Timer, //= Timer in Normal mode.
        }
    }
    0x508 => reg32 BITMODE { //! Sets timer behaviour.
        0..1 => BITMODE: rw { //! Sets timer behaviour ro be like the implementation of a timer with width as indicated.
            0x00 => 16Bit, //= 16-bit timer behaviour.
            0x01 => 08Bit, //= 8-bit timer behaviour.
            0x02 => 24Bit, //= 24-bit timer behaviour.
            0x03 => 32Bit, //= 32-bit timer behaviour.
        }
    }
    0x510 => reg32 PRESCALER { //! 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE.
        0..3 => PRESCALER: rw,
    }
    0x540 => group CC[4] { //! Capture/compare registers.
        0x0 => reg32 CC {} //! Capture/compare registers.
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (RTC0 @ 0x4000B000 = {
    /// Real time counter 0.
    0x000 => reg32 TASKS_START {} //! Start RTC Counter.
    0x004 => reg32 TASKS_STOP {} //! Stop RTC Counter.
    0x008 => reg32 TASKS_CLEAR {} //! Clear RTC Counter.
    0x00C => reg32 TASKS_TRIGOVRFLW {} //! Set COUNTER to 0xFFFFFFF0.
    0x100 => reg32 EVENTS_TICK {} //! Event on COUNTER increment.
    0x104 => reg32 EVENTS_OVRFLW {} //! Event on COUNTER overflow.
    0x140 => group EVENTS_COMPARE[4] { //! Compare event on CC[n] match.
        0x0 => reg32 EVENTS_COMPARE {} //! Compare event on CC[n] match.
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => TICK: rw { //! Enable interrupt on TICK event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => OVRFLW: rw { //! Enable interrupt on OVRFLW event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        16 => COMPARE0: rw { //! Enable interrupt on COMPARE[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Enable interrupt on COMPARE[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Enable interrupt on COMPARE[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Enable interrupt on COMPARE[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => TICK: rw { //! Disable interrupt on TICK event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => OVRFLW: rw { //! Disable interrupt on OVRFLW event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        16 => COMPARE0: rw { //! Disable interrupt on COMPARE[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Disable interrupt on COMPARE[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Disable interrupt on COMPARE[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Disable interrupt on COMPARE[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x340 => reg32 EVTEN { //! Configures event enable routing to PPI for each RTC event.
        0 => TICK: rw { //! TICK event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! OVRFLW event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! COMPARE[0] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! COMPARE[1] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! COMPARE[2] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! COMPARE[3] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x344 => reg32 EVTENSET { //! Enable events routing to PPI. The reading of this register gives the value of EVTEN.
        0 => TICK: rw { //! Enable routing to PPI of TICK event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! Enable routing to PPI of OVRFLW event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! Enable routing to PPI of COMPARE[0] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! Enable routing to PPI of COMPARE[1] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! Enable routing to PPI of COMPARE[2] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! Enable routing to PPI of COMPARE[3] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x348 => reg32 EVTENCLR { //! Disable events routing to PPI. The reading of this register gives the value of EVTEN.
        0 => TICK: rw { //! Disable routing to PPI of TICK event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! Disable routing to PPI of OVRFLW event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! Disable routing to PPI of COMPARE[0] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! Disable routing to PPI of COMPARE[1] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! Disable routing to PPI of COMPARE[2] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! Disable routing to PPI of COMPARE[3] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x504 => reg32 COUNTER { //! Current COUNTER value.
        0..23 => COUNTER: ro,
    }
    0x508 => reg32 PRESCALER { //! 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed.
        0..11 => PRESCALER: rw,
    }
    0x540 => group CC[4] { //! Capture/compare registers.
        0x0 => reg32 CC { //! Capture/compare registers.
            0..23 => COMPARE: rw,
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (TEMP @ 0x4000C000 = {
    /// Temperature Sensor.
    0x000 => reg32 TASKS_START {} //! Start temperature measurement.
    0x004 => reg32 TASKS_STOP {} //! Stop temperature measurement.
    0x100 => reg32 EVENTS_DATARDY {} //! Temperature measurement complete, data ready event.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => DATARDY: rw { //! Enable interrupt on DATARDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => DATARDY: rw { //! Disable interrupt on DATARDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x508 => reg32 TEMP {} //! Die temperature in degC, 2's complement format, 0.25 degC pecision.
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (RNG @ 0x4000D000 = {
    /// Random Number Generator.
    0x000 => reg32 TASKS_START {} //! Start the random number generator.
    0x004 => reg32 TASKS_STOP {} //! Stop the random number generator.
    0x100 => reg32 EVENTS_VALRDY {} //! New random number generated and written to VALUE register.
    0x200 => reg32 SHORTS { //! Shortcuts for the RNG.
        0 => VALRDY_STOP: rw { //! Shortcut between VALRDY event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register
        0 => VALRDY: rw { //! Enable interrupt on VALRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register
        0 => VALRDY: rw { //! Disable interrupt on VALRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x504 => reg32 CONFIG { //! Configuration register.
        0 => DERCEN: rw { //! Digital error correction enable.
            0 => Disabled, //= Digital error correction disabled.
            1 => Enabled, //= Digital error correction enabled.
        }
    }
    0x508 => reg32 VALUE { //! RNG random number.
        0..7 => VALUE: ro,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (ECB @ 0x4000E000 = {
    /// AES ECB Mode Encryption.
    0x000 => reg32 TASKS_STARTECB {} //! Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered.
    0x004 => reg32 TASKS_STOPECB {} //! Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event.
    0x100 => reg32 EVENTS_ENDECB {} //! ECB block encrypt complete.
    0x104 => reg32 EVENTS_ERRORECB {} //! ECB block encrypt aborted due to a STOPECB task or due to an error.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => ENDECB: rw { //! Enable interrupt on ENDECB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ERRORECB: rw { //! Enable interrupt on ERRORECB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => ENDECB: rw { //! Disable interrupt on ENDECB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ERRORECB: rw { //! Disable interrupt on ERRORECB event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x504 => reg32 ECBDATAPTR {} //! ECB block encrypt memory pointer.
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (AAR @ 0x4000F000 = {
    /// Accelerated Address Resolver.
    0x000 => reg32 TASKS_START {} //! Start resolving addresses based on IRKs specified in the IRK data structure.
    0x008 => reg32 TASKS_STOP {} //! Stop resolving addresses.
    0x100 => reg32 EVENTS_END {} //! Address resolution procedure completed.
    0x104 => reg32 EVENTS_RESOLVED {} //! Address resolved.
    0x108 => reg32 EVENTS_NOTRESOLVED {} //! Address not resolved.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => END: rw { //! Enable interrupt on END event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => RESOLVED: rw { //! Enable interrupt on RESOLVED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => NOTRESOLVED: rw { //! Enable interrupt on NOTRESOLVED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => END: rw { //! Disable interrupt on ENDKSGEN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => RESOLVED: rw { //! Disable interrupt on RESOLVED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => NOTRESOLVED: rw { //! Disable interrupt on NOTRESOLVED event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 STATUS { //! Resolution status.
        0..3 => STATUS: ro,
    }
    0x500 => reg32 ENABLE { //! Enable AAR.
        0..1 => ENABLE: rw { //! Enable AAR.
            0x00 => Disabled, //= Disabled AAR.
            0x03 => Enabled, //= Enable AAR.
        }
    }
    0x504 => reg32 NIRK { //! Number of Identity root Keys in the IRK data structure.
        0..4 => NIRK: rw,
    }
    0x508 => reg32 IRKPTR {} //! Pointer to the IRK data structure.
    0x510 => reg32 ADDRPTR {} //! Pointer to the resolvable address (6 bytes).
    0x514 => reg32 SCRATCHPTR {} //! Pointer to a "scratch" data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved.
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (CCM @ 0x4000F000 = {
    /// AES CCM Mode Encryption.
    0x000 => reg32 TASKS_KSGEN {} //! Start generation of key-stream. This operation will stop by itself when completed.
    0x004 => reg32 TASKS_CRYPT {} //! Start encrypt/decrypt. This operation will stop by itself when completed.
    0x008 => reg32 TASKS_STOP {} //! Stop encrypt/decrypt.
    0x100 => reg32 EVENTS_ENDKSGEN {} //! Keystream generation completed.
    0x104 => reg32 EVENTS_ENDCRYPT {} //! Encrypt/decrypt completed.
    0x108 => reg32 EVENTS_ERROR {} //! Error happened.
    0x200 => reg32 SHORTS { //! Shortcuts for the CCM.
        0 => ENDKSGEN_CRYPT: rw { //! Shortcut between ENDKSGEN event and CRYPT task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => ENDKSGEN: rw { //! Enable interrupt on ENDKSGEN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ENDCRYPT: rw { //! Enable interrupt on ENDCRYPT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => ERROR: rw { //! Enable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => ENDKSGEN: rw { //! Disable interrupt on ENDKSGEN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => ENDCRYPT: rw { //! Disable interrupt on ENDCRYPT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => ERROR: rw { //! Disable interrupt on ERROR event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 MICSTATUS { //! CCM RX MIC check result.
        0 => MICSTATUS: ro { //! Result of the MIC check performed during the previous CCM RX STARTCRYPT
            0 => CheckFailed, //= MIC check failed.
            1 => CheckPassed, //= MIC check passed.
        }
    }
    0x500 => reg32 ENABLE { //! CCM enable.
        0..1 => ENABLE: rw { //! CCM enable.
            0x00 => Disabled, //= CCM is disabled.
            0x02 => Enabled, //= CCM is enabled.
        }
    }
    0x504 => reg32 MODE { //! Operation mode.
        0 => MODE: rw { //! CCM mode operation.
            0 => Encryption, //= CCM mode TX
            1 => Decryption, //= CCM mode TX
        }
    }
    0x508 => reg32 CNFPTR {} //! Pointer to a data structure holding AES key and NONCE vector.
    0x50C => reg32 INPTR {} //! Pointer to the input packet.
    0x510 => reg32 OUTPTR {} //! Pointer to the output packet.
    0x514 => reg32 SCRATCHPTR {} //! Pointer to a "scratch" data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved.
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (WDT @ 0x40010000 = {
    /// Watchdog Timer.
    0x000 => reg32 TASKS_START {} //! Start the watchdog.
    0x100 => reg32 EVENTS_TIMEOUT {} //! Watchdog timeout.
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => TIMEOUT: rw { //! Enable interrupt on TIMEOUT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => TIMEOUT: rw { //! Disable interrupt on TIMEOUT event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 RUNSTATUS { //! Watchdog running status.
        0 => RUNSTATUS: ro { //! Watchdog running status.
            0 => NotRunning, //= Watchdog timer is not running.
            1 => Running, //= Watchdog timer is running.
        }
    }
    0x404 => reg32 REQSTATUS { //! Request status.
        0 => RR0: ro { //! Request status for RR[0].
            0 => DisabledOrRequested, //= RR[0] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[0] register is enabled and has not jet requested.
        }
        1 => RR1: ro { //! Request status for RR[1].
            0 => DisabledOrRequested, //= RR[1] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[1] register is enabled and has not jet requested.
        }
        2 => RR2: ro { //! Request status for RR[2].
            0 => DisabledOrRequested, //= RR[2] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[2] register is enabled and has not jet requested.
        }
        3 => RR3: ro { //! Request status for RR[3].
            0 => DisabledOrRequested, //= RR[3] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[3] register is enabled and has not jet requested.
        }
        4 => RR4: ro { //! Request status for RR[4].
            0 => DisabledOrRequested, //= RR[4] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[4] register is enabled and has not jet requested.
        }
        5 => RR5: ro { //! Request status for RR[5].
            0 => DisabledOrRequested, //= RR[5] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[5] register is enabled and has not jet requested.
        }
        6 => RR6: ro { //! Request status for RR[6].
            0 => DisabledOrRequested, //= RR[6] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[6] register is enabled and has not jet requested.
        }
        7 => RR7: ro { //! Request status for RR[7].
            0 => DisabledOrRequested, //= RR[7] register is not enabled or has already requested reload.
            1 => EnabledAndUnrequested, //= RR[7] register is enabled and has not jet requested.
        }
    }
    0x504 => reg32 CRV {} //! Counter reload value in number of 32kiHz clock cycles.
    0x508 => reg32 RREN { //! Reload request enable.
        0 => RR0: rw { //! Enable or disable RR[0] register.
            0 => Disabled, //= RR[0] register is disabled.
            1 => Enabled, //= RR[0] register is enabled.
        }
        1 => RR1: rw { //! Enable or disable RR[1] register.
            0 => Disabled, //= RR[1] register is disabled.
            1 => Enabled, //= RR[1] register is enabled.
        }
        2 => RR2: rw { //! Enable or disable RR[2] register.
            0 => Disabled, //= RR[2] register is disabled.
            1 => Enabled, //= RR[2] register is enabled.
        }
        3 => RR3: rw { //! Enable or disable RR[3] register.
            0 => Disabled, //= RR[3] register is disabled.
            1 => Enabled, //= RR[3] register is enabled.
        }
        4 => RR4: rw { //! Enable or disable RR[4] register.
            0 => Disabled, //= RR[4] register is disabled.
            1 => Enabled, //= RR[4] register is enabled.
        }
        5 => RR5: rw { //! Enable or disable RR[5] register.
            0 => Disabled, //= RR[5] register is disabled.
            1 => Enabled, //= RR[5] register is enabled.
        }
        6 => RR6: rw { //! Enable or disable RR[6] register.
            0 => Disabled, //= RR[6] register is disabled.
            1 => Enabled, //= RR[6] register is enabled.
        }
        7 => RR7: rw { //! Enable or disable RR[7] register.
            0 => Disabled, //= RR[7] register is disabled.
            1 => Enabled, //= RR[7] register is enabled.
        }
    }
    0x50C => reg32 CONFIG { //! Configuration register.
        0 => SLEEP: rw { //! Configure the watchdog to pause or not while the CPU is sleeping.
            0 => Pause, //= Pause watchdog while the CPU is asleep.
            1 => Run, //= Do not pause watchdog while the CPU is asleep.
        }
        3 => HALT: rw { //! Configure the watchdog to pause or not while the CPU is halted by the debugger.
            0 => Pause, //= Pause watchdog while the CPU is halted by the debugger.
            1 => Run, //= Do not pause watchdog while the CPU is halted by the debugger.
        }
    }
    0x600 => group RR[8] { //! Reload requests registers.
        0x0 => reg32 RR { //! Reload requests registers.
            0..31 => RR: wo { //! Reload register.
                0x6E524635 => Reload, //= Value to request a reload of the watchdog timer.
            }
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (RTC1 @ 0x40011000 = {
    /// Real time counter 1.
    0x000 => reg32 TASKS_START {} //! Start RTC Counter.
    0x004 => reg32 TASKS_STOP {} //! Stop RTC Counter.
    0x008 => reg32 TASKS_CLEAR {} //! Clear RTC Counter.
    0x00C => reg32 TASKS_TRIGOVRFLW {} //! Set COUNTER to 0xFFFFFFF0.
    0x100 => reg32 EVENTS_TICK {} //! Event on COUNTER increment.
    0x104 => reg32 EVENTS_OVRFLW {} //! Event on COUNTER overflow.
    0x140 => group EVENTS_COMPARE[4] { //! Compare event on CC[n] match.
        0x0 => reg32 EVENTS_COMPARE {} //! Compare event on CC[n] match.
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => TICK: rw { //! Enable interrupt on TICK event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => OVRFLW: rw { //! Enable interrupt on OVRFLW event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        16 => COMPARE0: rw { //! Enable interrupt on COMPARE[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Enable interrupt on COMPARE[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Enable interrupt on COMPARE[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Enable interrupt on COMPARE[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => TICK: rw { //! Disable interrupt on TICK event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => OVRFLW: rw { //! Disable interrupt on OVRFLW event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        16 => COMPARE0: rw { //! Disable interrupt on COMPARE[0] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        17 => COMPARE1: rw { //! Disable interrupt on COMPARE[1] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        18 => COMPARE2: rw { //! Disable interrupt on COMPARE[2] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        19 => COMPARE3: rw { //! Disable interrupt on COMPARE[3] event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x340 => reg32 EVTEN { //! Configures event enable routing to PPI for each RTC event.
        0 => TICK: rw { //! TICK event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! OVRFLW event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! COMPARE[0] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! COMPARE[1] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! COMPARE[2] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! COMPARE[3] event enable.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x344 => reg32 EVTENSET { //! Enable events routing to PPI. The reading of this register gives the value of EVTEN.
        0 => TICK: rw { //! Enable routing to PPI of TICK event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! Enable routing to PPI of OVRFLW event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! Enable routing to PPI of COMPARE[0] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! Enable routing to PPI of COMPARE[1] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! Enable routing to PPI of COMPARE[2] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! Enable routing to PPI of COMPARE[3] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x348 => reg32 EVTENCLR { //! Disable events routing to PPI. The reading of this register gives the value of EVTEN.
        0 => TICK: rw { //! Disable routing to PPI of TICK event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        1 => OVRFLW: rw { //! Disable routing to PPI of OVRFLW event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        16 => COMPARE0: rw { //! Disable routing to PPI of COMPARE[0] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        17 => COMPARE1: rw { //! Disable routing to PPI of COMPARE[1] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        18 => COMPARE2: rw { //! Disable routing to PPI of COMPARE[2] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
        19 => COMPARE3: rw { //! Disable routing to PPI of COMPARE[3] event.
            0 => Disabled, //= Event disabled.
            1 => Enabled, //= Event enabled.
        }
    }
    0x504 => reg32 COUNTER { //! Current COUNTER value.
        0..23 => COUNTER: ro,
    }
    0x508 => reg32 PRESCALER { //! 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed.
        0..11 => PRESCALER: rw,
    }
    0x540 => group CC[4] { //! Capture/compare registers.
        0x0 => reg32 CC { //! Capture/compare registers.
            0..23 => COMPARE: rw,
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (QDEC @ 0x40012000 = {
    /// Rotary decoder.
    0x000 => reg32 TASKS_START {} //! Start the quadrature decoder.
    0x004 => reg32 TASKS_STOP {} //! Stop the quadrature decoder.
    0x008 => reg32 TASKS_READCLRACC {} //! Transfers the content from ACC registers to ACCREAD registers, and clears the ACC registers.
    0x100 => reg32 EVENTS_SAMPLERDY {} //! A new sample is written to the sample register.
    0x104 => reg32 EVENTS_REPORTRDY {} //! REPORTPER number of samples accumulated in ACC register, and ACC register different than zero.
    0x108 => reg32 EVENTS_ACCOF {} //! ACC or ACCDBL register overflow.
    0x200 => reg32 SHORTS { //! Shortcuts for the QDEC.
        0 => REPORTRDY_READCLRACC: rw { //! Shortcut between REPORTRDY event and READCLRACC task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => SAMPLERDY_STOP: rw { //! Shortcut between SAMPLERDY event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => SAMPLERDY: rw { //! Enable interrupt on SAMPLERDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => REPORTRDY: rw { //! Enable interrupt on REPORTRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => ACCOF: rw { //! Enable interrupt on ACCOF event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => SAMPLERDY: rw { //! Disable interrupt on SAMPLERDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => REPORTRDY: rw { //! Disable interrupt on REPORTRDY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => ACCOF: rw { //! Disable interrupt on ACCOF event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x500 => reg32 ENABLE { //! Enable the QDEC.
        0 => ENABLE: rw { //! Enable or disable QDEC.
            0 => Disabled, //= Disabled QDEC.
            1 => Enabled, //= Enable QDEC.
        }
    }
    0x504 => reg32 LEDPOL { //! LED output pin polarity.
        0 => LEDPOL: rw { //! LED output pin polarity.
            0 => ActiveLow, //= LED output is active low.
            1 => ActiveHigh, //= LED output is active high.
        }
    }
    0x508 => reg32 SAMPLEPER { //! Sample period.
        0..2 => SAMPLEPER: rw { //! Sample period.
            0x00 => 128us, //= 128us sample period.
            0x01 => 256us, //= 256us sample period.
            0x02 => 512us, //= 512us sample period.
            0x03 => 1024us, //= 1024us sample period.
            0x04 => 2048us, //= 2048us sample period.
            0x05 => 4096us, //= 4096us sample period.
            0x06 => 8192us, //= 8192us sample period.
            0x07 => 16384us, //= 16384us sample period.
        }
    }
    0x50C => reg32 SAMPLE { //! Motion sample value.
        0..31 => SAMPLE: ro,
    }
    0x510 => reg32 REPORTPER { //! Number of samples to generate an EVENT_REPORTRDY.
        0..2 => REPORTPER: rw { //! Number of samples to generate an EVENT_REPORTRDY.
            0x00 => 10Smpl, //= 10 samples per report.
            0x01 => 40Smpl, //= 40 samples per report.
            0x02 => 80Smpl, //= 80 samples per report.
            0x03 => 120Smpl, //= 120 samples per report.
            0x04 => 160Smpl, //= 160 samples per report.
            0x05 => 200Smpl, //= 200 samples per report.
            0x06 => 240Smpl, //= 240 samples per report.
            0x07 => 280Smpl, //= 280 samples per report.
        }
    }
    0x514 => reg32 ACC {} //! Accumulated valid transitions register.
    0x518 => reg32 ACCREAD {} //! Snapshot of ACC register. Value generated by the TASKS_READCLEACC task.
    0x51C => reg32 PSELLED {} //! Pin select for LED output.
    0x520 => reg32 PSELA {} //! Pin select for phase A input.
    0x524 => reg32 PSELB {} //! Pin select for phase B input.
    0x528 => reg32 DBFEN { //! Enable debouncer input filters.
        0 => DBFEN: rw { //! Enable debounce input filters.
            0 => Disabled, //= Debounce input filters disabled.
            1 => Enabled, //= Debounce input filters enabled.
        }
    }
    0x540 => reg32 LEDPRE { //! Time LED is switched ON before the sample.
        0..8 => LEDPRE: rw,
    }
    0x544 => reg32 ACCDBL { //! Accumulated double (error) transitions register.
        0..3 => ACCDBL: ro,
    }
    0x548 => reg32 ACCDBLREAD { //! Snapshot of ACCDBL register. Value generated by the TASKS_READCLEACC task.
        0..3 => ACCDBLREAD: ro,
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (LPCOMP @ 0x40013000 = {
    /// Low power comparator.
    0x000 => reg32 TASKS_START {} //! Start the comparator.
    0x004 => reg32 TASKS_STOP {} //! Stop the comparator.
    0x008 => reg32 TASKS_SAMPLE {} //! Sample comparator value.
    0x100 => reg32 EVENTS_READY {} //! LPCOMP is ready and output is valid.
    0x104 => reg32 EVENTS_DOWN {} //! Input voltage crossed the threshold going down.
    0x108 => reg32 EVENTS_UP {} //! Input voltage crossed the threshold going up.
    0x10C => reg32 EVENTS_CROSS {} //! Input voltage crossed the threshold in any direction.
    0x200 => reg32 SHORTS { //! Shortcuts for the LPCOMP.
        0 => READY_SAMPLE: rw { //! Shortcut between READY event and SAMPLE task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        1 => READY_STOP: rw { //! Shortcut between RADY event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        2 => DOWN_STOP: rw { //! Shortcut between DOWN event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        3 => UP_STOP: rw { //! Shortcut between UP event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
        4 => CROSS_STOP: rw { //! Shortcut between CROSS event and STOP task.
            0 => Disabled, //= Shortcut disabled.
            1 => Enabled, //= Shortcut enabled.
        }
    }
    0x304 => reg32 INTENSET { //! Interrupt enable set register.
        0 => READY: rw { //! Enable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => DOWN: rw { //! Enable interrupt on DOWN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => UP: rw { //! Enable interrupt on UP event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => CROSS: rw { //! Enable interrupt on CROSS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x308 => reg32 INTENCLR { //! Interrupt enable clear register.
        0 => READY: rw { //! Disable interrupt on READY event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        1 => DOWN: rw { //! Disable interrupt on DOWN event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        2 => UP: rw { //! Disable interrupt on UP event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
        3 => CROSS: rw { //! Disable interrupt on CROSS event.
            0 => Disabled, //= Interrupt disabled.
            1 => Enabled, //= Interrupt enabled.
        }
    }
    0x400 => reg32 RESULT { //! Result of last compare.
        0 => RESULT: ro { //! Result of last compare. Decision point SAMPLE task.
            0 => Bellow, //= Input voltage is bellow the reference threshold.
            1 => Above, //= Input voltage is above the reference threshold.
        }
    }
    0x500 => reg32 ENABLE { //! Enable the LPCOMP.
        0..1 => ENABLE: rw { //! Enable or disable LPCOMP.
            0x00 => Disabled, //= Disabled LPCOMP.
            0x01 => Enabled, //= Enable LPCOMP.
        }
    }
    0x504 => reg32 PSEL { //! Input pin select.
        0..2 => PSEL: rw { //! Analog input pin select.
            0 => AnalogInput0, //= Use analog input 0 as analog input.
            1 => AnalogInput1, //= Use analog input 1 as analog input.
            2 => AnalogInput2, //= Use analog input 2 as analog input.
            3 => AnalogInput3, //= Use analog input 3 as analog input.
            4 => AnalogInput4, //= Use analog input 4 as analog input.
            5 => AnalogInput5, //= Use analog input 5 as analog input.
            6 => AnalogInput6, //= Use analog input 6 as analog input.
            7 => AnalogInput7, //= Use analog input 7 as analog input.
        }
    }
    0x508 => reg32 REFSEL { //! Reference select.
        0..2 => REFSEL: rw { //! Reference select.
            0 => SupplyOneEighthPrescaling, //= Use supply with a 1/8 prescaler as reference.
            1 => SupplyTwoEighthsPrescaling, //= Use supply with a 2/8 prescaler as reference.
            2 => SupplyThreeEighthsPrescaling, //= Use supply with a 3/8 prescaler as reference.
            3 => SupplyFourEighthsPrescaling, //= Use supply with a 4/8 prescaler as reference.
            4 => SupplyFiveEighthsPrescaling, //= Use supply with a 5/8 prescaler as reference.
            5 => SupplySixEighthsPrescaling, //= Use supply with a 6/8 prescaler as reference.
            6 => SupplySevenEighthsPrescaling, //= Use supply with a 7/8 prescaler as reference.
            7 => ARef, //= Use external analog reference as reference.
        }
    }
    0x50C => reg32 EXTREFSEL { //! External reference select.
        0 => EXTREFSEL: rw { //! External analog reference pin selection.
            0 => AnalogReference0, //= Use analog reference 0 as reference.
            1 => AnalogReference1, //= Use analog reference 1 as reference.
        }
    }
    0x520 => reg32 ANADETECT { //! Analog detect configuration.
        0..1 => ANADETECT: rw { //! Analog detect configuration.
            0 => Cross, //= Generate ANADETEC on crossing, both upwards and downwards crossing.
            1 => Up, //= Generate ANADETEC on upwards crossing only.
            2 => Down, //= Generate ANADETEC on downwards crossing only.
        }
    }
    0xFFC => reg32 POWER { //! Peripheral power control.
        0 => POWER: rw { //! Peripheral power control.
            0 => Disabled, //= Module power disabled.
            1 => Enabled, //= Module power enabled.
        }
    }
});

ioregs! (SWI @ 0x40014000 = {
    /// SW Interrupts.
    0x000 => reg32 UNUSED {} //! Unused.
});

ioregs! (NVMC @ 0x4001E000 = {
    /// Non Volatile Memory Controller.
    0x400 => reg32 READY { //! Ready flag.
        0 => READY: ro { //! NVMC ready.
            0 => Busy, //= NVMC is busy (on-going write or erase operation).
            1 => Ready, //= NVMC is ready.
        }
    }
    0x504 => reg32 CONFIG { //! Configuration register.
        0..1 => WEN: rw { //! Program write enable.
            0x00 => Ren, //= Read only access.
            0x01 => Wen, //= Write enabled.
            0x02 => Een, //= Erase enabled.
        }
    }
    0x508 => reg32 ERASEPAGE {} //! Register for erasing a non-protected non-volatile memory page.
    0x508 => reg32 ERASEPCR1 {} //! Register for erasing a non-protected non-volatile memory page.
    0x50C => reg32 ERASEALL { //! Register for erasing all non-volatile user memory.
        0 => ERASEALL: rw { //! Starts the erasing of all user NVM (code region 0/1 and UICR registers).
            0 => NoOperation, //= No operation.
            1 => Erase, //= Start chip erase.
        }
    }
    0x510 => reg32 ERASEPCR0 {} //! Register for erasing a protected non-volatile memory page.
    0x514 => reg32 ERASEUICR { //! Register for start erasing User Information Congfiguration Registers.
        0 => ERASEUICR: rw { //! It can only be used when all contents of code region 1 are erased.
            0 => NoOperation, //= No operation.
            1 => Erase, //= Start UICR erase.
        }
    }
});

ioregs! (PPI @ 0x4001F000 = {
    /// PPI controller.
    0x000 => group TASKS_CHG[4] { //! Channel group tasks.
        0x000 => reg32 EN {} //! Enable channel group.
        0x004 => reg32 DIS {} //! Disable channel group.
    }
    0x500 => reg32 CHEN { //! Channel enable.
        0 => CH0: rw { //! Enable PPI channel 0.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        1 => CH1: rw { //! Enable PPI channel 1.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        2 => CH2: rw { //! Enable PPI channel 2.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        3 => CH3: rw { //! Enable PPI channel 3.
            0 => Disabled, //= Channel disabled
            1 => Enabled, //= Channel enabled
        }
        4 => CH4: rw { //! Enable PPI channel 4.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        5 => CH5: rw { //! Enable PPI channel 5.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        6 => CH6: rw { //! Enable PPI channel 6.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        7 => CH7: rw { //! Enable PPI channel 7.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        8 => CH8: rw { //! Enable PPI channel 8.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        9 => CH9: rw { //! Enable PPI channel 9.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        10 => CH10: rw { //! Enable PPI channel 10.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        11 => CH11: rw { //! Enable PPI channel 11.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        12 => CH12: rw { //! Enable PPI channel 12.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        13 => CH13: rw { //! Enable PPI channel 13.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        14 => CH14: rw { //! Enable PPI channel 14.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        15 => CH15: rw { //! Enable PPI channel 15.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        20 => CH20: rw { //! Enable PPI channel 20.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        21 => CH21: rw { //! Enable PPI channel 21.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        22 => CH22: rw { //! Enable PPI channel 22.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        23 => CH23: rw { //! Enable PPI channel 23.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        24 => CH24: rw { //! Enable PPI channel 24.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        25 => CH25: rw { //! Enable PPI channel 25.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        26 => CH26: rw { //! Enable PPI channel 26.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        27 => CH27: rw { //! Enable PPI channel 27.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        28 => CH28: rw { //! Enable PPI channel 28.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        29 => CH29: rw { //! Enable PPI channel 29.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        30 => CH30: rw { //! Enable PPI channel 30.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        31 => CH31: rw { //! Enable PPI channel 31.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
    }
    0x504 => reg32 CHENSET { //! Channel enable set.
        0 => CH0: rw { //! Enable PPI channel 0.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        1 => CH1: rw { //! Enable PPI channel 1.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        2 => CH2: rw { //! Enable PPI channel 2.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        3 => CH3: rw { //! Enable PPI channel 3.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        4 => CH4: rw { //! Enable PPI channel 4.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        5 => CH5: rw { //! Enable PPI channel 5.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        6 => CH6: rw { //! Enable PPI channel 6.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        7 => CH7: rw { //! Enable PPI channel 7.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        8 => CH8: rw { //! Enable PPI channel 8.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        9 => CH9: rw { //! Enable PPI channel 9.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        10 => CH10: rw { //! Enable PPI channel 10.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        11 => CH11: rw { //! Enable PPI channel 11.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        12 => CH12: rw { //! Enable PPI channel 12.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        13 => CH13: rw { //! Enable PPI channel 13.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        14 => CH14: rw { //! Enable PPI channel 14.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        15 => CH15: rw { //! Enable PPI channel 15.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        20 => CH20: rw { //! Enable PPI channel 20.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        21 => CH21: rw { //! Enable PPI channel 21.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        22 => CH22: rw { //! Enable PPI channel 22.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        23 => CH23: rw { //! Enable PPI channel 23.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        24 => CH24: rw { //! Enable PPI channel 24.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        25 => CH25: rw { //! Enable PPI channel 25.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        26 => CH26: rw { //! Enable PPI channel 26.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        27 => CH27: rw { //! Enable PPI channel 27.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        28 => CH28: rw { //! Enable PPI channel 28.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        29 => CH29: rw { //! Enable PPI channel 29.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        30 => CH30: rw { //! Enable PPI channel 30.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        31 => CH31: rw { //! Enable PPI channel 31.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
    }
    0x508 => reg32 CHENCLR { //! Channel enable clear.
        0 => CH0: rw { //! Disable PPI channel 0.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        1 => CH1: rw { //! Disable PPI channel 1.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        2 => CH2: rw { //! Disable PPI channel 2.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        3 => CH3: rw { //! Disable PPI channel 3.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        4 => CH4: rw { //! Disable PPI channel 4.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        5 => CH5: rw { //! Disable PPI channel 5.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        6 => CH6: rw { //! Disable PPI channel 6.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        7 => CH7: rw { //! Disable PPI channel 7.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        8 => CH8: rw { //! Disable PPI channel 8.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        9 => CH9: rw { //! Disable PPI channel 9.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        10 => CH10: rw { //! Disable PPI channel 10.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        11 => CH11: rw { //! Disable PPI channel 11.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        12 => CH12: rw { //! Disable PPI channel 12.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        13 => CH13: rw { //! Disable PPI channel 13.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        14 => CH14: rw { //! Disable PPI channel 14.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        15 => CH15: rw { //! Disable PPI channel 15.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        20 => CH20: rw { //! Disable PPI channel 20.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        21 => CH21: rw { //! Disable PPI channel 21.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        22 => CH22: rw { //! Disable PPI channel 22.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        23 => CH23: rw { //! Disable PPI channel 23.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        24 => CH24: rw { //! Disable PPI channel 24.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        25 => CH25: rw { //! Disable PPI channel 25.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        26 => CH26: rw { //! Disable PPI channel 26.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        27 => CH27: rw { //! Disable PPI channel 27.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        28 => CH28: rw { //! Disable PPI channel 28.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        29 => CH29: rw { //! Disable PPI channel 29.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        30 => CH30: rw { //! Disable PPI channel 30.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
        31 => CH31: rw { //! Disable PPI channel 31.
            0 => Disabled, //= Channel disabled.
            1 => Enabled, //= Channel enabled.
        }
    }
    0x510 => group CH[16] { //! PPI Channel.
        0x000 => reg32 EEP {} //! Channel event end-point.
        0x004 => reg32 TEP {} //! Channel task end-point.
    }
    0x800 => group CHG[4] { //! Channel group configuration.
        0x0 => reg32 CHG { //! Channel group configuration.
            0 => CH0: rw { //! Include CH0 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            1 => CH1: rw { //! Include CH1 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            2 => CH2: rw { //! Include CH2 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            3 => CH3: rw { //! Include CH3 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            4 => CH4: rw { //! Include CH4 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            5 => CH5: rw { //! Include CH5 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            6 => CH6: rw { //! Include CH6 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            7 => CH7: rw { //! Include CH7 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            8 => CH8: rw { //! Include CH8 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            9 => CH9: rw { //! Include CH9 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            10 => CH10: rw { //! Include CH10 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            11 => CH11: rw { //! Include CH11 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            12 => CH12: rw { //! Include CH12 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            13 => CH13: rw { //! Include CH13 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            14 => CH14: rw { //! Include CH14 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            15 => CH15: rw { //! Include CH15 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            20 => CH20: rw { //! Include CH20 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            21 => CH21: rw { //! Include CH21 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            22 => CH22: rw { //! Include CH22 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            23 => CH23: rw { //! Include CH23 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            24 => CH24: rw { //! Include CH24 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            25 => CH25: rw { //! Include CH25 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            26 => CH26: rw { //! Include CH26 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            27 => CH27: rw { //! Include CH27 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            28 => CH28: rw { //! Include CH28 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            29 => CH29: rw { //! Include CH29 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            30 => CH30: rw { //! Include CH30 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
            31 => CH31: rw { //! Include CH31 in channel group.
                0 => Excluded, //= Channel excluded.
                1 => Included, //= Channel included.
            }
        }
    }
});

ioregs! (FICR @ 0x10000000 = {
    /// Factory Information Configuration.
    0x010 => reg32 CODEPAGESIZE {} //! Code memory page size in bytes.
    0x014 => reg32 CODESIZE {} //! Code memory size in pages.
    0x028 => reg32 CLENR0 {} //! Length of code region 0 in bytes.
    0x02C => reg32 PPFC { //! Pre-programmed factory code present.
        0..7 => PPFC: ro { //! Pre-programmed factory code present.
            0xFF => NotPresent, //= Not present.
            0x00 => Present, //= Present.
        }
    }
    0x034 => reg32 NUMRAMBLOCK {} //! Number of individualy controllable RAM blocks.
    0x038 => reg32 SIZERAMBLOCKS {} //! Size of RAM blocks in bytes.
    0x038 => group SIZERAMBLOCK[4] { //! Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.
        0x0 => reg32 SIZERAMBLOCK {} //! Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.
    }
    0x05C => reg32 CONFIGID { //! Configuration identifier.
        0..15 => HWID: ro,
        16..31 => FWID: ro,
    }
    0x060 => group DEVICEID[2] { //! Device identifier.
        0x0 => reg32 DEVICEID {} //! Device identifier.
    }
    0x080 => group ER[4] { //! Encryption root.
        0x0 => reg32 ER {} //! Encryption root.
    }
    0x090 => group IR[4] { //! Identity root.
        0x0 => reg32 IR {} //! Identity root.
    }
    0x0A0 => reg32 DEVICEADDRTYPE { //! Device address type.
        0 => DEVICEADDRTYPE: ro { //! Device address type.
            0 => Public, //= Public address.
            1 => Random, //= Random address.
        }
    }
    0x0A4 => group DEVICEADDR[2] { //! Device address.
        0x0 => reg32 DEVICEADDR {} //! Device address.
    }
    0x0AC => reg32 OVERRIDEEN { //! Radio calibration override enable.
        0 => NRF_1MBIT: ro { //! Override default values for NRF_1Mbit mode.
            0 => Override, //= Override the default values for NRF_1Mbit mode.
            1 => NotOverride, //= Do not override the default values for NRF_1Mbit mode.
        }
        3 => BLE_1MBIT: ro { //! Override default values for BLE_1Mbit mode.
            0 => Override, //= Override the default values for BLE_1Mbit mode.
            1 => NotOverride, //= Do not override the default values for BLE_1Mbit mode.
        }
    }
    0x0B0 => group NRF_1MBIT[5] { //! Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.
        0x0 => reg32 NRF_1MBIT {} //! Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.
    }
    0x0EC => group BLE_1MBIT[5] { //! Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode.
        0x0 => reg32 BLE_1MBIT {} //! Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode.
    }
});

ioregs! (UICR @ 0x10001000 = {
    /// User Information Configuration.
    0x000 => reg32 CLENR0 {} //! Length of code region 0.
    0x004 => reg32 RBPCONF { //! Readback protection configuration.
        0..7 => PR0: rw { //! Readback protect region 0. Will be ignored if pre-programmed factory code is present on the chip.
            0xFF => Disabled, //= Disabled.
            0x00 => Enabled, //= Enabled.
        }
        8..15 => PALL: rw { //! Readback protect all code in the device.
            0xFF => Disabled, //= Disabled.
            0x00 => Enabled, //= Enabled.
        }
    }
    0x008 => reg32 XTALFREQ { //! Reset value for CLOCK XTALFREQ register.
        0..7 => XTALFREQ: rw { //! Reset value for CLOCK XTALFREQ register.
            0xFF => 16MHz, //= 16MHz Xtal is used.
            0x00 => 32MHz, //= 32MHz Xtal is used.
        }
    }
    0x010 => reg32 FWID { //! Firmware ID.
        0..15 => FWID: ro,
    }
    0x014 => reg32 BOOTLOADERADDR {} //! Bootloader start address.
    0x014 => group NRFFW[15] { //! Reserved for Nordic firmware design.
        0x0 => reg32 NRFFW {} //! Reserved for Nordic firmware design.
    }
    0x050 => group NRFHW[12] { //! Reserved for Nordic hardware design.
        0x0 => reg32 NRFHW {} //! Reserved for Nordic hardware design.
    }
    0x080 => group CUSTOMER[32] { //! Reserved for customer.
        0x0 => reg32 CUSTOMER {} //! Reserved for customer.
    }
});

ioregs! (GPIO @ 0x50000000 = {
    /// General purpose input and output.
    0x504 => reg32 OUT { //! Write GPIO port.
        0 => PIN0: rw { //! Pin 0.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        1 => PIN1: rw { //! Pin 1.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        2 => PIN2: rw { //! Pin 2.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        3 => PIN3: rw { //! Pin 3.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        4 => PIN4: rw { //! Pin 4.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        5 => PIN5: rw { //! Pin 5.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        6 => PIN6: rw { //! Pin 6.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        7 => PIN7: rw { //! Pin 7.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        8 => PIN8: rw { //! Pin 8.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        9 => PIN9: rw { //! Pin 9.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        10 => PIN10: rw { //! Pin 10.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        11 => PIN11: rw { //! Pin 11.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        12 => PIN12: rw { //! Pin 12.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        13 => PIN13: rw { //! Pin 13.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        14 => PIN14: rw { //! Pin 14.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        15 => PIN15: rw { //! Pin 15.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        16 => PIN16: rw { //! Pin 16.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        17 => PIN17: rw { //! Pin 17.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        18 => PIN18: rw { //! Pin 18.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        19 => PIN19: rw { //! Pin 19.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        20 => PIN20: rw { //! Pin 20.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        21 => PIN21: rw { //! Pin 21.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        22 => PIN22: rw { //! Pin 22.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        23 => PIN23: rw { //! Pin 23.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        24 => PIN24: rw { //! Pin 24.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        25 => PIN25: rw { //! Pin 25.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        26 => PIN26: rw { //! Pin 26.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        27 => PIN27: rw { //! Pin 27.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        28 => PIN28: rw { //! Pin 28.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        29 => PIN29: rw { //! Pin 29.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        30 => PIN30: rw { //! Pin 30.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        31 => PIN31: rw { //! Pin 31.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
    }
    0x508 => reg32 OUTSET { //! Set individual bits in GPIO port.
        0 => PIN0: rw { //! Pin 0.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        1 => PIN1: rw { //! Pin 1.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        2 => PIN2: rw { //! Pin 2.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        3 => PIN3: rw { //! Pin 3.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        4 => PIN4: rw { //! Pin 4.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        5 => PIN5: rw { //! Pin 5.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        6 => PIN6: rw { //! Pin 6.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        7 => PIN7: rw { //! Pin 7.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        8 => PIN8: rw { //! Pin 8.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        9 => PIN9: rw { //! Pin 9.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        10 => PIN10: rw { //! Pin 10.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        11 => PIN11: rw { //! Pin 11.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        12 => PIN12: rw { //! Pin 12.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        13 => PIN13: rw { //! Pin 13.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        14 => PIN14: rw { //! Pin 14.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        15 => PIN15: rw { //! Pin 15.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        16 => PIN16: rw { //! Pin 16.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        17 => PIN17: rw { //! Pin 17.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        18 => PIN18: rw { //! Pin 18.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        19 => PIN19: rw { //! Pin 19.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        20 => PIN20: rw { //! Pin 20.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        21 => PIN21: rw { //! Pin 21.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        22 => PIN22: rw { //! Pin 22.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        23 => PIN23: rw { //! Pin 23.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        24 => PIN24: rw { //! Pin 24.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        25 => PIN25: rw { //! Pin 25.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        26 => PIN26: rw { //! Pin 26.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        27 => PIN27: rw { //! Pin 27.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        28 => PIN28: rw { //! Pin 28.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        29 => PIN29: rw { //! Pin 29.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        30 => PIN30: rw { //! Pin 30.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        31 => PIN31: rw { //! Pin 31.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
    }
    0x50C => reg32 OUTCLR { //! Clear individual bits in GPIO port.
        0 => PIN0: rw { //! Pin 0.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        1 => PIN1: rw { //! Pin 1.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        2 => PIN2: rw { //! Pin 2.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        3 => PIN3: rw { //! Pin 3.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        4 => PIN4: rw { //! Pin 4.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        5 => PIN5: rw { //! Pin 5.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        6 => PIN6: rw { //! Pin 6.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        7 => PIN7: rw { //! Pin 7.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        8 => PIN8: rw { //! Pin 8.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        9 => PIN9: rw { //! Pin 9.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        10 => PIN10: rw { //! Pin 10.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        11 => PIN11: rw { //! Pin 11.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        12 => PIN12: rw { //! Pin 12.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        13 => PIN13: rw { //! Pin 13.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        14 => PIN14: rw { //! Pin 14.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        15 => PIN15: rw { //! Pin 15.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        16 => PIN16: rw { //! Pin 16.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        17 => PIN17: rw { //! Pin 17.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        18 => PIN18: rw { //! Pin 18.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        19 => PIN19: rw { //! Pin 19.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        20 => PIN20: rw { //! Pin 20.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        21 => PIN21: rw { //! Pin 21.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        22 => PIN22: rw { //! Pin 22.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        23 => PIN23: rw { //! Pin 23.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        24 => PIN24: rw { //! Pin 24.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        25 => PIN25: rw { //! Pin 25.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        26 => PIN26: rw { //! Pin 26.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        27 => PIN27: rw { //! Pin 27.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        28 => PIN28: rw { //! Pin 28.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        29 => PIN29: rw { //! Pin 29.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        30 => PIN30: rw { //! Pin 30.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
        31 => PIN31: rw { //! Pin 31.
            0 => Low, //= Pin driver is low.
            1 => High, //= Pin driver is high.
        }
    }
    0x510 => reg32 IN { //! Read GPIO port.
        0 => PIN0: ro { //! Pin 0.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        1 => PIN1: ro { //! Pin 1.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        2 => PIN2: ro { //! Pin 2.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        3 => PIN3: ro { //! Pin 3.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        4 => PIN4: ro { //! Pin 4.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        5 => PIN5: ro { //! Pin 5.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        6 => PIN6: ro { //! Pin 6.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        7 => PIN7: ro { //! Pin 7.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        8 => PIN8: ro { //! Pin 8.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        9 => PIN9: ro { //! Pin 9.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        10 => PIN10: ro { //! Pin 10.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        11 => PIN11: ro { //! Pin 11.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        12 => PIN12: ro { //! Pin 12.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        13 => PIN13: ro { //! Pin 13.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        14 => PIN14: ro { //! Pin 14.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        15 => PIN15: ro { //! Pin 15.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        16 => PIN16: ro { //! Pin 16.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        17 => PIN17: ro { //! Pin 17.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        18 => PIN18: ro { //! Pin 18.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        19 => PIN19: ro { //! Pin 19.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        20 => PIN20: ro { //! Pin 20.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        21 => PIN21: ro { //! Pin 21.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        22 => PIN22: ro { //! Pin 22.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        23 => PIN23: ro { //! Pin 23.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        24 => PIN24: ro { //! Pin 24.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        25 => PIN25: ro { //! Pin 25.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        26 => PIN26: ro { //! Pin 26.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        27 => PIN27: ro { //! Pin 27.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        28 => PIN28: ro { //! Pin 28.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        29 => PIN29: ro { //! Pin 29.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        30 => PIN30: ro { //! Pin 30.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
        31 => PIN31: ro { //! Pin 31.
            0 => Low, //= Pin input is low.
            1 => High, //= Pin input is high.
        }
    }
    0x514 => reg32 DIR { //! Direction of GPIO pins.
        0 => PIN0: rw { //! Pin 0.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        1 => PIN1: rw { //! Pin 1.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        2 => PIN2: rw { //! Pin 2.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        3 => PIN3: rw { //! Pin 3.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        4 => PIN4: rw { //! Pin 4.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        5 => PIN5: rw { //! Pin 5.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        6 => PIN6: rw { //! Pin 6.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        7 => PIN7: rw { //! Pin 7.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        8 => PIN8: rw { //! Pin 8.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        9 => PIN9: rw { //! Pin 9.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        10 => PIN10: rw { //! Pin 10.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        11 => PIN11: rw { //! Pin 11.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        12 => PIN12: rw { //! Pin 12.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        13 => PIN13: rw { //! Pin 13.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        14 => PIN14: rw { //! Pin 14.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        15 => PIN15: rw { //! Pin 15.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        16 => PIN16: rw { //! Pin 16.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        17 => PIN17: rw { //! Pin 17.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        18 => PIN18: rw { //! Pin 18.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        19 => PIN19: rw { //! Pin 19.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        20 => PIN20: rw { //! Pin 20.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        21 => PIN21: rw { //! Pin 21.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        22 => PIN22: rw { //! Pin 22.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        23 => PIN23: rw { //! Pin 23.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        24 => PIN24: rw { //! Pin 24.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        25 => PIN25: rw { //! Pin 25.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        26 => PIN26: rw { //! Pin 26.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        27 => PIN27: rw { //! Pin 27.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        28 => PIN28: rw { //! Pin 28.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        29 => PIN29: rw { //! Pin 29.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        30 => PIN30: rw { //! Pin 30.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        31 => PIN31: rw { //! Pin 31.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
    }
    0x518 => reg32 DIRSET { //! DIR set register.
        0 => PIN0: rw { //! Set as output pin 0.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        1 => PIN1: rw { //! Set as output pin 1.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        2 => PIN2: rw { //! Set as output pin 2.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        3 => PIN3: rw { //! Set as output pin 3.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        4 => PIN4: rw { //! Set as output pin 4.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        5 => PIN5: rw { //! Set as output pin 5.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        6 => PIN6: rw { //! Set as output pin 6.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        7 => PIN7: rw { //! Set as output pin 7.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        8 => PIN8: rw { //! Set as output pin 8.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        9 => PIN9: rw { //! Set as output pin 9.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        10 => PIN10: rw { //! Set as output pin 10.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        11 => PIN11: rw { //! Set as output pin 11.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        12 => PIN12: rw { //! Set as output pin 12.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        13 => PIN13: rw { //! Set as output pin 13.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        14 => PIN14: rw { //! Set as output pin 14.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        15 => PIN15: rw { //! Set as output pin 15.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        16 => PIN16: rw { //! Set as output pin 16.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        17 => PIN17: rw { //! Set as output pin 17.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        18 => PIN18: rw { //! Set as output pin 18.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        19 => PIN19: rw { //! Set as output pin 19.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        20 => PIN20: rw { //! Set as output pin 20.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        21 => PIN21: rw { //! Set as output pin 21.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        22 => PIN22: rw { //! Set as output pin 22.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        23 => PIN23: rw { //! Set as output pin 23.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        24 => PIN24: rw { //! Set as output pin 24.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        25 => PIN25: rw { //! Set as output pin 25.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        26 => PIN26: rw { //! Set as output pin 26.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        27 => PIN27: rw { //! Set as output pin 27.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        28 => PIN28: rw { //! Set as output pin 28.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        29 => PIN29: rw { //! Set as output pin 29.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        30 => PIN30: rw { //! Set as output pin 30.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        31 => PIN31: rw { //! Set as output pin 31.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
    }
    0x51C => reg32 DIRCLR { //! DIR clear register.
        0 => PIN0: rw { //! Set as input pin 0.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        1 => PIN1: rw { //! Set as input pin 1.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        2 => PIN2: rw { //! Set as input pin 2.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        3 => PIN3: rw { //! Set as input pin 3.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        4 => PIN4: rw { //! Set as input pin 4.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        5 => PIN5: rw { //! Set as input pin 5.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        6 => PIN6: rw { //! Set as input pin 6.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        7 => PIN7: rw { //! Set as input pin 7.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        8 => PIN8: rw { //! Set as input pin 8.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        9 => PIN9: rw { //! Set as input pin 9.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        10 => PIN10: rw { //! Set as input pin 10.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        11 => PIN11: rw { //! Set as input pin 11.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        12 => PIN12: rw { //! Set as input pin 12.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        13 => PIN13: rw { //! Set as input pin 13.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        14 => PIN14: rw { //! Set as input pin 14.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        15 => PIN15: rw { //! Set as input pin 15.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        16 => PIN16: rw { //! Set as input pin 16.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        17 => PIN17: rw { //! Set as input pin 17.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        18 => PIN18: rw { //! Set as input pin 18.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        19 => PIN19: rw { //! Set as input pin 19.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        20 => PIN20: rw { //! Set as input pin 20.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        21 => PIN21: rw { //! Set as input pin 21.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        22 => PIN22: rw { //! Set as input pin 22.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        23 => PIN23: rw { //! Set as input pin 23.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        24 => PIN24: rw { //! Set as input pin 24.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        25 => PIN25: rw { //! Set as input pin 25.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        26 => PIN26: rw { //! Set as input pin 26.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        27 => PIN27: rw { //! Set as input pin 27.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        28 => PIN28: rw { //! Set as input pin 28.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        29 => PIN29: rw { //! Set as input pin 29.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        30 => PIN30: rw { //! Set as input pin 30.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
        31 => PIN31: rw { //! Set as input pin 31.
            0 => Input, //= Pin set as input.
            1 => Output, //= Pin set as output.
        }
    }
    0x700 => group PIN_CNF[32] { //! Configuration of GPIO pins.
        0x0 => reg32 PIN_CNF { //! Configuration of GPIO pins.
            0 => DIR: rw { //! Pin direction.
                0 => Input, //= Configure pin as an input pin.
                1 => Output, //= Configure pin as an output pin.
            }
            1 => INPUT: rw { //! Connect or disconnect input path.
                0 => Connect, //= Connect input pin.
                1 => Disconnect, //= Disconnect input pin.
            }
            2..3 => PULL: rw { //! Pull-up or -down configuration.
                0x00 => Disabled, //= No pull.
                0x01 => Pulldown, //= Pulldown on pin.
                0x03 => Pullup, //= Pullup on pin.
            }
            8..10 => DRIVE: rw { //! Drive configuration.
                0x00 => S0S1, //= Standard '0', Standard '1'.
                0x01 => H0S1, //= High '0', Standard '1'.
                0x02 => S0H1, //= Standard '0', High '1'.
                0x03 => H0H1, //= High '0', High '1'.
                0x04 => D0S1, //= Disconnected '0', Standard '1'.
                0x05 => D0H1, //= Disconnected '0', High '1'.
                0x06 => S0D1, //= Standard '0', Disconnected '1'.
                0x07 => H0D1, //= High '0', Disconnected '1'.
            }
            16..17 => SENSE: rw { //! Pin sensing mechanism.
                0x00 => Disabled, //= Disabled.
                0x02 => High, //= Wakeup on high level.
                0x03 => Low, //= Wakeup on low level.
            }
        }
    }
});
