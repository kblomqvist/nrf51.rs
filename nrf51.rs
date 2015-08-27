// The MIT License (MIT)
//
// Copyright (c) 2015 Kim Blomqvist
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use volatile_cell::VolatileCell;
use core::ops::Drop


ioregs! (POWER @ 0x40000000) = {
	0x078 => reg32 TASKS_CONSTLAT
	0x07C => reg32 TASKS_LOWPWR
	0x108 => reg32 EVENTS_POFWARN
	0x304 => reg32 INTENSET = {
		2 => POFWARN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		2 => POFWARN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 RESETREAS = {
		0 => RESETPIN {
			0 => NotDetected
			1 => Detected
			
		}
		1 => DOG {
			0 => NotDetected
			1 => Detected
			
		}
		2 => SREQ {
			0 => NotDetected
			1 => Detected
			
		}
		3 => LOCKUP {
			0 => NotDetected
			1 => Detected
			
		}
		16 => OFF {
			0 => NotDetected
			1 => Detected
			
		}
		17 => LPCOMP {
			0 => NotDetected
			1 => Detected
			
		}
		18 => DIF {
			0 => NotDetected
			1 => Detected
			
		}
		
	}
	0x428 => reg32 RAMSTATUS = {
		0 => RAMBLOCK0 {
			0 => Off
			1 => On
			
		}
		1 => RAMBLOCK1 {
			0 => Off
			1 => On
			
		}
		2 => RAMBLOCK2 {
			0 => Off
			1 => On
			
		}
		3 => RAMBLOCK3 {
			0 => Off
			1 => On
			
		}
		
	}
	0x500 => reg32 SYSTEMOFF = {
		0 => SYSTEMOFF {
			1 => Enter
			
		}
		
	}
	0x510 => reg32 POFCON = {
		0 => POF {
			0 => Disabled
			1 => Enabled
			
		}
		1..2 => THRESHOLD {
			0x00 => V21
			0x01 => V23
			0x02 => V25
			0x03 => V27
			
		}
		
	}
	0x51C => reg32 GPREGRET = {
		0..7 => GPREGRET,
		
	}
	0x524 => reg32 RAMON = {
		0 => ONRAM0 {
			0 => RAM0Off
			1 => RAM0On
			
		}
		1 => ONRAM1 {
			0 => RAM1Off
			1 => RAM1On
			
		}
		16 => OFFRAM0 {
			0 => RAM0Off
			1 => RAM0On
			
		}
		17 => OFFRAM1 {
			0 => RAM1Off
			1 => RAM1On
			
		}
		
	}
	0x544 => reg32 RESET = {
		0 => RESET {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x554 => reg32 RAMONB = {
		0 => ONRAM2 {
			0 => RAM2Off
			1 => RAM2On
			
		}
		1 => ONRAM3 {
			0 => RAM3Off
			1 => RAM3On
			
		}
		16 => OFFRAM2 {
			0 => RAM2Off
			1 => RAM2On
			
		}
		17 => OFFRAM3 {
			0 => RAM3Off
			1 => RAM3On
			
		}
		
	}
	0x578 => reg32 DCDCEN = {
		0 => DCDCEN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0xA08 => reg32 DCDCFORCE = {
		0 => FORCEOFF {
			0 => NoForce
			1 => Force
			
		}
		1 => FORCEON {
			0 => NoForce
			1 => Force
			
		}
		
	}
	
}

ioregs! (CLOCK @ 0x40000000) = {
	0x000 => reg32 TASKS_HFCLKSTART
	0x004 => reg32 TASKS_HFCLKSTOP
	0x008 => reg32 TASKS_LFCLKSTART
	0x00C => reg32 TASKS_LFCLKSTOP
	0x010 => reg32 TASKS_CAL
	0x014 => reg32 TASKS_CTSTART
	0x018 => reg32 TASKS_CTSTOP
	0x100 => reg32 EVENTS_HFCLKSTARTED
	0x104 => reg32 EVENTS_LFCLKSTARTED
	0x10C => reg32 EVENTS_DONE
	0x110 => reg32 EVENTS_CTTO
	0x304 => reg32 INTENSET = {
		0 => HFCLKSTARTED {
			0 => Disabled
			1 => Enabled
			
		}
		1 => LFCLKSTARTED {
			0 => Disabled
			1 => Enabled
			
		}
		3 => DONE {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CTTO {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => HFCLKSTARTED {
			0 => Disabled
			1 => Enabled
			
		}
		1 => LFCLKSTARTED {
			0 => Disabled
			1 => Enabled
			
		}
		3 => DONE {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CTTO {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x408 => reg32 HFCLKRUN = {
		0 => STATUS {
			0 => NotTriggered
			1 => Triggered
			
		}
		
	}
	0x40C => reg32 HFCLKSTAT = {
		0 => SRC {
			0 => RC
			1 => Xtal
			
		}
		16 => STATE {
			0 => NotRunning
			1 => Running
			
		}
		
	}
	0x414 => reg32 LFCLKRUN = {
		0 => STATUS {
			0 => NotTriggered
			1 => Triggered
			
		}
		
	}
	0x418 => reg32 LFCLKSTAT = {
		0..1 => SRC {
			0 => RC
			1 => Xtal
			2 => Synth
			
		}
		16 => STATE {
			0 => NotRunning
			1 => Running
			
		}
		
	}
	0x41C => reg32 LFCLKSRCCOPY = {
		0..1 => SRC {
			0 => RC
			1 => Xtal
			2 => Synth
			
		}
		
	}
	0x518 => reg32 LFCLKSRC = {
		0..1 => SRC {
			0 => RC
			1 => Xtal
			2 => Synth
			
		}
		
	}
	0x538 => reg32 CTIV = {
		0..6 => CTIV,
		
	}
	0x550 => reg32 XTALFREQ = {
		0..7 => XTALFREQ {
			0xFF => 16MHz
			0x00 => 32MHz
			
		}
		
	}
	
}

ioregs! (MPU @ 0x40000000) = {
	0x528 => reg32 PERR0 = {
		0 => POWER_CLOCK {
			1 => InRegion0
			0 => InRegion1
			
		}
		1 => RADIO {
			1 => InRegion0
			0 => InRegion1
			
		}
		2 => UART0 {
			1 => InRegion0
			0 => InRegion1
			
		}
		3 => SPI0_TWI0 {
			1 => InRegion0
			0 => InRegion1
			
		}
		4 => SPI1_TWI1 {
			1 => InRegion0
			0 => InRegion1
			
		}
		6 => GPIOTE {
			1 => InRegion0
			0 => InRegion1
			
		}
		7 => ADC {
			1 => InRegion0
			0 => InRegion1
			
		}
		8 => TIMER0 {
			1 => InRegion0
			0 => InRegion1
			
		}
		9 => TIMER1 {
			1 => InRegion0
			0 => InRegion1
			
		}
		10 => TIMER2 {
			1 => InRegion0
			0 => InRegion1
			
		}
		11 => RTC0 {
			1 => InRegion0
			0 => InRegion1
			
		}
		12 => TEMP {
			1 => InRegion0
			0 => InRegion1
			
		}
		13 => RNG {
			1 => InRegion0
			0 => InRegion1
			
		}
		14 => ECB {
			1 => InRegion0
			0 => InRegion1
			
		}
		15 => CCM_AAR {
			1 => InRegion0
			0 => InRegion1
			
		}
		16 => WDT {
			1 => InRegion0
			0 => InRegion1
			
		}
		17 => RTC1 {
			1 => InRegion0
			0 => InRegion1
			
		}
		18 => QDEC {
			1 => InRegion0
			0 => InRegion1
			
		}
		19 => LPCOMP {
			1 => InRegion0
			0 => InRegion1
			
		}
		30 => NVMC {
			1 => InRegion0
			0 => InRegion1
			
		}
		31 => PPI {
			1 => InRegion0
			0 => InRegion1
			
		}
		
	}
	0x52C => reg32 RLENR0
	0x600 => reg32 PROTENSET0 = {
		0 => PROTREG0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => PROTREG1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => PROTREG2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => PROTREG3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => PROTREG4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => PROTREG5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => PROTREG6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => PROTREG7 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => PROTREG8 {
			0 => Disabled
			1 => Enabled
			
		}
		9 => PROTREG9 {
			0 => Disabled
			1 => Enabled
			
		}
		10 => PROTREG10 {
			0 => Disabled
			1 => Enabled
			
		}
		11 => PROTREG11 {
			0 => Disabled
			1 => Enabled
			
		}
		12 => PROTREG12 {
			0 => Disabled
			1 => Enabled
			
		}
		13 => PROTREG13 {
			0 => Disabled
			1 => Enabled
			
		}
		14 => PROTREG14 {
			0 => Disabled
			1 => Enabled
			
		}
		15 => PROTREG15 {
			0 => Disabled
			1 => Enabled
			
		}
		16 => PROTREG16 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => PROTREG17 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => PROTREG18 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => PROTREG19 {
			0 => Disabled
			1 => Enabled
			
		}
		20 => PROTREG20 {
			0 => Disabled
			1 => Enabled
			
		}
		21 => PROTREG21 {
			0 => Disabled
			1 => Enabled
			
		}
		22 => PROTREG22 {
			0 => Disabled
			1 => Enabled
			
		}
		23 => PROTREG23 {
			0 => Disabled
			1 => Enabled
			
		}
		24 => PROTREG24 {
			0 => Disabled
			1 => Enabled
			
		}
		25 => PROTREG25 {
			0 => Disabled
			1 => Enabled
			
		}
		26 => PROTREG26 {
			0 => Disabled
			1 => Enabled
			
		}
		27 => PROTREG27 {
			0 => Disabled
			1 => Enabled
			
		}
		28 => PROTREG28 {
			0 => Disabled
			1 => Enabled
			
		}
		29 => PROTREG29 {
			0 => Disabled
			1 => Enabled
			
		}
		30 => PROTREG30 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => PROTREG31 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x604 => reg32 PROTENSET1 = {
		0 => PROTREG32 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => PROTREG33 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => PROTREG34 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => PROTREG35 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => PROTREG36 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => PROTREG37 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => PROTREG38 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => PROTREG39 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => PROTREG40 {
			0 => Disabled
			1 => Enabled
			
		}
		9 => PROTREG41 {
			0 => Disabled
			1 => Enabled
			
		}
		10 => PROTREG42 {
			0 => Disabled
			1 => Enabled
			
		}
		11 => PROTREG43 {
			0 => Disabled
			1 => Enabled
			
		}
		12 => PROTREG44 {
			0 => Disabled
			1 => Enabled
			
		}
		13 => PROTREG45 {
			0 => Disabled
			1 => Enabled
			
		}
		14 => PROTREG46 {
			0 => Disabled
			1 => Enabled
			
		}
		15 => PROTREG47 {
			0 => Disabled
			1 => Enabled
			
		}
		16 => PROTREG48 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => PROTREG49 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => PROTREG50 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => PROTREG51 {
			0 => Disabled
			1 => Enabled
			
		}
		20 => PROTREG52 {
			0 => Disabled
			1 => Enabled
			
		}
		21 => PROTREG53 {
			0 => Disabled
			1 => Enabled
			
		}
		22 => PROTREG54 {
			0 => Disabled
			1 => Enabled
			
		}
		23 => PROTREG55 {
			0 => Disabled
			1 => Enabled
			
		}
		24 => PROTREG56 {
			0 => Disabled
			1 => Enabled
			
		}
		25 => PROTREG57 {
			0 => Disabled
			1 => Enabled
			
		}
		26 => PROTREG58 {
			0 => Disabled
			1 => Enabled
			
		}
		27 => PROTREG59 {
			0 => Disabled
			1 => Enabled
			
		}
		28 => PROTREG60 {
			0 => Disabled
			1 => Enabled
			
		}
		29 => PROTREG61 {
			0 => Disabled
			1 => Enabled
			
		}
		30 => PROTREG62 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => PROTREG63 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x608 => reg32 DISABLEINDEBUG = {
		0 => DISABLEINDEBUG {
			0 => Enabled
			1 => Disabled
			
		}
		
	}
	0x60C => reg32 PROTBLOCKSIZE = {
		0..1 => PROTBLOCKSIZE {
			0 => 4k
			
		}
		
	}
	
}

ioregs! (AMLI @ 0x40000000)

ioregs! (RADIO @ 0x40001000) = {
	0x000 => reg32 TASKS_TXEN
	0x004 => reg32 TASKS_RXEN
	0x008 => reg32 TASKS_START
	0x00C => reg32 TASKS_STOP
	0x010 => reg32 TASKS_DISABLE
	0x014 => reg32 TASKS_RSSISTART
	0x018 => reg32 TASKS_RSSISTOP
	0x01C => reg32 TASKS_BCSTART
	0x020 => reg32 TASKS_BCSTOP
	0x100 => reg32 EVENTS_READY
	0x104 => reg32 EVENTS_ADDRESS
	0x108 => reg32 EVENTS_PAYLOAD
	0x10C => reg32 EVENTS_END
	0x110 => reg32 EVENTS_DISABLED
	0x114 => reg32 EVENTS_DEVMATCH
	0x118 => reg32 EVENTS_DEVMISS
	0x11C => reg32 EVENTS_RSSIEND
	0x128 => reg32 EVENTS_BCMATCH
	0x200 => reg32 SHORTS = {
		0 => READY_START {
			0 => Disabled
			1 => Enabled
			
		}
		1 => END_DISABLE {
			0 => Disabled
			1 => Enabled
			
		}
		2 => DISABLED_TXEN {
			0 => Disabled
			1 => Enabled
			
		}
		3 => DISABLED_RXEN {
			0 => Disabled
			1 => Enabled
			
		}
		4 => ADDRESS_RSSISTART {
			0 => Disabled
			1 => Enabled
			
		}
		5 => END_START {
			0 => Disabled
			1 => Enabled
			
		}
		6 => ADDRESS_BCSTART {
			0 => Disabled
			1 => Enabled
			
		}
		8 => DISABLED_RSSISTOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ADDRESS {
			0 => Disabled
			1 => Enabled
			
		}
		2 => PAYLOAD {
			0 => Disabled
			1 => Enabled
			
		}
		3 => END {
			0 => Disabled
			1 => Enabled
			
		}
		4 => DISABLED {
			0 => Disabled
			1 => Enabled
			
		}
		5 => DEVMATCH {
			0 => Disabled
			1 => Enabled
			
		}
		6 => DEVMISS {
			0 => Disabled
			1 => Enabled
			
		}
		7 => RSSIEND {
			0 => Disabled
			1 => Enabled
			
		}
		10 => BCMATCH {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ADDRESS {
			0 => Disabled
			1 => Enabled
			
		}
		2 => PAYLOAD {
			0 => Disabled
			1 => Enabled
			
		}
		3 => END {
			0 => Disabled
			1 => Enabled
			
		}
		4 => DISABLED {
			0 => Disabled
			1 => Enabled
			
		}
		5 => DEVMATCH {
			0 => Disabled
			1 => Enabled
			
		}
		6 => DEVMISS {
			0 => Disabled
			1 => Enabled
			
		}
		7 => RSSIEND {
			0 => Disabled
			1 => Enabled
			
		}
		10 => BCMATCH {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 CRCSTATUS = {
		0 => CRCSTATUS {
			0 => CRCError
			1 => CRCOk
			
		}
		
	}
	0x408 => reg32 RXMATCH = {
		0..2 => RXMATCH,
		
	}
	0x40C => reg32 RXCRC = {
		0..23 => RXCRC,
		
	}
	0x410 => reg32 DAI = {
		0..2 => DAI,
		
	}
	0x504 => reg32 PACKETPTR
	0x508 => reg32 FREQUENCY = {
		0..6 => FREQUENCY,
		
	}
	0x50C => reg32 TXPOWER = {
		0..7 => TXPOWER {
			0x04 => Pos4dBm
			0x00 => 0dBm
			0xFC => Neg4dBm
			0xF8 => Neg8dBm
			0xF4 => Neg12dBm
			0xF0 => Neg16dBm
			0xEC => Neg20dBm
			0xD8 => Neg30dBm
			
		}
		
	}
	0x510 => reg32 MODE = {
		0..1 => MODE {
			0x00 => Nrf_1Mbit
			0x01 => Nrf_2Mbit
			0x02 => Nrf_250Kbit
			0x03 => Ble_1Mbit
			
		}
		
	}
	0x514 => reg32 PCNF0 = {
		0..3 => LFLEN,
		8 => S0LEN,
		16..19 => S1LEN,
		
	}
	0x518 => reg32 PCNF1 = {
		0..7 => MAXLEN,
		8..15 => STATLEN,
		16..18 => BALEN,
		24 => ENDIAN {
			0 => Little
			1 => Big
			
		}
		25 => WHITEEN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x51C => reg32 BASE0
	0x520 => reg32 BASE1
	0x524 => reg32 PREFIX0 = {
		0..7 => AP0,
		8..15 => AP1,
		16..23 => AP2,
		24..31 => AP3,
		
	}
	0x528 => reg32 PREFIX1 = {
		0..7 => AP4,
		8..15 => AP5,
		16..23 => AP6,
		24..31 => AP7,
		
	}
	0x52C => reg32 TXADDRESS = {
		0..2 => TXADDRESS,
		
	}
	0x530 => reg32 RXADDRESSES = {
		0 => ADDR0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ADDR1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ADDR2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => ADDR3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => ADDR4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => ADDR5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => ADDR6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => ADDR7 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x534 => reg32 CRCCNF = {
		0..1 => LEN {
			0 => Disabled
			1 => One
			2 => Two
			3 => Three
			
		}
		8 => SKIPADDR {
			0 => Include
			1 => Skip
			
		}
		
	}
	0x538 => reg32 CRCPOLY = {
		0..23 => CRCPOLY,
		
	}
	0x53C => reg32 CRCINIT = {
		0..23 => CRCINIT,
		
	}
	0x540 => reg32 TEST = {
		0 => CONSTCARRIER {
			0 => Disabled
			1 => Enabled
			
		}
		1 => PLLLOCK {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x544 => reg32 TIFS = {
		0..7 => TIFS,
		
	}
	0x548 => reg32 RSSISAMPLE = {
		0..6 => RSSISAMPLE,
		
	}
	0x550 => reg32 STATE = {
		0..3 => STATE {
			0x00 => Disabled
			0x01 => RxRu
			0x02 => RxIdle
			0x03 => Rx
			0x04 => RxDisable
			0x09 => TxRu
			0x0A => TxIdle
			0x0B => Tx
			0x0C => TxDisable
			
		}
		
	}
	0x554 => reg32 DATAWHITEIV = {
		0..6 => DATAWHITEIV,
		
	}
	0x560 => reg32 BCC
	0x600 => reg32 DAB[%s]
	0x620 => reg32 DAP[%s] = {
		0..15 => DAP,
		
	}
	0x640 => reg32 DACNF = {
		0 => ENA0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ENA1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ENA2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => ENA3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => ENA4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => ENA5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => ENA6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => ENA7 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => TXADD0,
		9 => TXADD1,
		10 => TXADD2,
		11 => TXADD3,
		12 => TXADD4,
		13 => TXADD5,
		14 => TXADD6,
		15 => TXADD7,
		
	}
	0x724 => reg32 OVERRIDE0 = {
		0..31 => OVERRIDE0,
		
	}
	0x728 => reg32 OVERRIDE1 = {
		0..31 => OVERRIDE1,
		
	}
	0x72C => reg32 OVERRIDE2 = {
		0..31 => OVERRIDE2,
		
	}
	0x730 => reg32 OVERRIDE3 = {
		0..31 => OVERRIDE3,
		
	}
	0x734 => reg32 OVERRIDE4 = {
		0..27 => OVERRIDE4,
		31 => ENABLE {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (UART0 @ 0x40002000) = {
	0x000 => reg32 TASKS_STARTRX
	0x004 => reg32 TASKS_STOPRX
	0x008 => reg32 TASKS_STARTTX
	0x00C => reg32 TASKS_STOPTX
	0x01C => reg32 TASKS_SUSPEND
	0x100 => reg32 EVENTS_CTS
	0x104 => reg32 EVENTS_NCTS
	0x108 => reg32 EVENTS_RXDRDY
	0x11C => reg32 EVENTS_TXDRDY
	0x124 => reg32 EVENTS_ERROR
	0x144 => reg32 EVENTS_RXTO
	0x200 => reg32 SHORTS = {
		3 => CTS_STARTRX {
			0 => Disabled
			1 => Enabled
			
		}
		4 => NCTS_STOPRX {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => CTS {
			0 => Disabled
			1 => Enabled
			
		}
		1 => NCTS {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDRDY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDRDY {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		17 => RXTO {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => CTS {
			0 => Disabled
			1 => Enabled
			
		}
		1 => NCTS {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDRDY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDRDY {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		17 => RXTO {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x480 => reg32 ERRORSRC = {
		0 => OVERRUN {
			0 => NotPresent
			1 => Present
			
		}
		1 => PARITY {
			0 => NotPresent
			1 => Present
			
		}
		2 => FRAMING {
			0 => NotPresent
			1 => Present
			
		}
		3 => BREAK {
			0 => NotPresent
			1 => Present
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x04 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELRTS
	0x50C => reg32 PSELTXD
	0x510 => reg32 PSELCTS
	0x514 => reg32 PSELRXD
	0x518 => reg32 RXD = {
		0..7 => RXD,
		
	}
	0x51C => reg32 TXD = {
		0..7 => TXD,
		
	}
	0x524 => reg32 BAUDRATE = {
		0..31 => BAUDRATE {
			0x0004F000 => Baud1200
			0x0009D000 => Baud2400
			0x0013B000 => Baud4800
			0x00275000 => Baud9600
			0x003B0000 => Baud14400
			0x004EA000 => Baud19200
			0x0075F000 => Baud28800
			0x009D5000 => Baud38400
			0x00EBF000 => Baud57600
			0x013A9000 => Baud76800
			0x01D7E000 => Baud115200
			0x03AFB000 => Baud230400
			0x04000000 => Baud250000
			0x075F7000 => Baud460800
			0x0EBED000 => Baud921600
			0x10000000 => Baud1M
			
		}
		
	}
	0x56C => reg32 CONFIG = {
		0 => HWFC {
			0 => Disabled
			1 => Enabled
			
		}
		1..3 => PARITY {
			0 => Excluded
			7 => Included
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (SPI0 @ 0x40003000) = {
	0x108 => reg32 EVENTS_READY
	0x304 => reg32 INTENSET = {
		2 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		2 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x01 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELSCK
	0x50C => reg32 PSELMOSI
	0x510 => reg32 PSELMISO
	0x518 => reg32 RXD = {
		0..7 => RXD,
		
	}
	0x51C => reg32 TXD = {
		0..7 => TXD,
		
	}
	0x524 => reg32 FREQUENCY = {
		0..31 => FREQUENCY {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
			
		}
		
	}
	0x554 => reg32 CONFIG = {
		0 => ORDER {
			0 => MsbFirst
			1 => LsbFirst
			
		}
		1 => CPHA {
			0 => Leading
			1 => Trailing
			
		}
		2 => CPOL {
			0 => ActiveHigh
			1 => ActiveLow
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TWI0 @ 0x40003000) = {
	0x000 => reg32 TASKS_STARTRX
	0x008 => reg32 TASKS_STARTTX
	0x014 => reg32 TASKS_STOP
	0x01C => reg32 TASKS_SUSPEND
	0x020 => reg32 TASKS_RESUME
	0x104 => reg32 EVENTS_STOPPED
	0x108 => reg32 EVENTS_RXDREADY
	0x11C => reg32 EVENTS_TXDSENT
	0x124 => reg32 EVENTS_ERROR
	0x138 => reg32 EVENTS_BB
	0x148 => reg32 EVENTS_SUSPENDED
	0x200 => reg32 SHORTS = {
		0 => BB_SUSPEND {
			0 => Disabled
			1 => Enabled
			
		}
		1 => BB_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDREADY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDSENT {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		14 => BB {
			0 => Disabled
			1 => Enabled
			
		}
		18 => SUSPENDED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDREADY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDSENT {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		14 => BB {
			0 => Disabled
			1 => Enabled
			
		}
		18 => SUSPENDED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x4C4 => reg32 ERRORSRC = {
		0 => OVERRUN {
			0 => NotPresent
			1 => Present
			
		}
		1 => ANACK {
			0 => NotPresent
			1 => Present
			
		}
		2 => DNACK {
			0 => NotPresent
			1 => Present
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x05 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELSCL
	0x50C => reg32 PSELSDA
	0x518 => reg32 RXD = {
		0..7 => RXD,
		
	}
	0x51C => reg32 TXD = {
		0..7 => TXD,
		
	}
	0x524 => reg32 FREQUENCY = {
		0..31 => FREQUENCY {
			0x01980000 => K100
			0x04000000 => K250
			0x06680000 => K400
			
		}
		
	}
	0x588 => reg32 ADDRESS = {
		0..6 => ADDRESS,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (SPI1 @ 0x40004000) = {
	0x108 => reg32 EVENTS_READY
	0x304 => reg32 INTENSET = {
		2 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		2 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x01 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELSCK
	0x50C => reg32 PSELMOSI
	0x510 => reg32 PSELMISO
	0x518 => reg32 RXD = {
		0..7 => RXD,
		
	}
	0x51C => reg32 TXD = {
		0..7 => TXD,
		
	}
	0x524 => reg32 FREQUENCY = {
		0..31 => FREQUENCY {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
			
		}
		
	}
	0x554 => reg32 CONFIG = {
		0 => ORDER {
			0 => MsbFirst
			1 => LsbFirst
			
		}
		1 => CPHA {
			0 => Leading
			1 => Trailing
			
		}
		2 => CPOL {
			0 => ActiveHigh
			1 => ActiveLow
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TWI1 @ 0x40004000) = {
	0x000 => reg32 TASKS_STARTRX
	0x008 => reg32 TASKS_STARTTX
	0x014 => reg32 TASKS_STOP
	0x01C => reg32 TASKS_SUSPEND
	0x020 => reg32 TASKS_RESUME
	0x104 => reg32 EVENTS_STOPPED
	0x108 => reg32 EVENTS_RXDREADY
	0x11C => reg32 EVENTS_TXDSENT
	0x124 => reg32 EVENTS_ERROR
	0x138 => reg32 EVENTS_BB
	0x148 => reg32 EVENTS_SUSPENDED
	0x200 => reg32 SHORTS = {
		0 => BB_SUSPEND {
			0 => Disabled
			1 => Enabled
			
		}
		1 => BB_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDREADY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDSENT {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		14 => BB {
			0 => Disabled
			1 => Enabled
			
		}
		18 => SUSPENDED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RXDREADY {
			0 => Disabled
			1 => Enabled
			
		}
		7 => TXDSENT {
			0 => Disabled
			1 => Enabled
			
		}
		9 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		14 => BB {
			0 => Disabled
			1 => Enabled
			
		}
		18 => SUSPENDED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x4C4 => reg32 ERRORSRC = {
		0 => OVERRUN {
			0 => NotPresent
			1 => Present
			
		}
		1 => ANACK {
			0 => NotPresent
			1 => Present
			
		}
		2 => DNACK {
			0 => NotPresent
			1 => Present
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x05 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELSCL
	0x50C => reg32 PSELSDA
	0x518 => reg32 RXD = {
		0..7 => RXD,
		
	}
	0x51C => reg32 TXD = {
		0..7 => TXD,
		
	}
	0x524 => reg32 FREQUENCY = {
		0..31 => FREQUENCY {
			0x01980000 => K100
			0x04000000 => K250
			0x06680000 => K400
			
		}
		
	}
	0x588 => reg32 ADDRESS = {
		0..6 => ADDRESS,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (SPIS1 @ 0x40004000) = {
	0x024 => reg32 TASKS_ACQUIRE
	0x028 => reg32 TASKS_RELEASE
	0x104 => reg32 EVENTS_END
	0x128 => reg32 EVENTS_ACQUIRED
	0x200 => reg32 SHORTS = {
		2 => END_ACQUIRE {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		1 => END {
			0 => Disabled
			1 => Enabled
			
		}
		10 => ACQUIRED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		1 => END {
			0 => Disabled
			1 => Enabled
			
		}
		10 => ACQUIRED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 SEMSTAT = {
		0..1 => SEMSTAT {
			0x00 => Free
			0x01 => CPU
			0x02 => SPIS
			0x03 => CPUPending
			
		}
		
	}
	0x440 => reg32 STATUS = {
		0 => OVERREAD {
			0 => NotPresent
			1 => Present
			
		}
		1 => OVERFLOW {
			0 => NotPresent
			1 => Present
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..2 => ENABLE {
			0x00 => Disabled
			0x02 => Enabled
			
		}
		
	}
	0x508 => reg32 PSELSCK
	0x50C => reg32 PSELMISO
	0x510 => reg32 PSELMOSI
	0x514 => reg32 PSELCSN
	0x534 => reg32 RXDPTR
	0x538 => reg32 MAXRX = {
		0..7 => MAXRX,
		
	}
	0x53C => reg32 AMOUNTRX = {
		0..7 => AMOUNTRX,
		
	}
	0x544 => reg32 TXDPTR
	0x548 => reg32 MAXTX = {
		0..7 => MAXTX,
		
	}
	0x54C => reg32 AMOUNTTX = {
		0..7 => AMOUNTTX,
		
	}
	0x554 => reg32 CONFIG = {
		0 => ORDER {
			0 => MsbFirst
			1 => LsbFirst
			
		}
		1 => CPHA {
			0 => Leading
			1 => Trailing
			
		}
		2 => CPOL {
			0 => ActiveHigh
			1 => ActiveLow
			
		}
		
	}
	0x55C => reg32 DEF = {
		0..7 => DEF,
		
	}
	0x5C0 => reg32 ORC = {
		0..7 => ORC,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (SPIM1 @ 0x40004000) = {
	0x010 => reg32 TASKS_START
	0x014 => reg32 TASKS_STOP
	0x01C => reg32 TASKS_SUSPEND
	0x020 => reg32 TASKS_RESUME
	0x104 => reg32 EVENTS_STOPPED
	0x110 => reg32 EVENTS_ENDRX
	0x120 => reg32 EVENTS_ENDTX
	0x14C => reg32 EVENTS_STARTED
	0x304 => reg32 INTENSET = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		4 => ENDRX {
			0 => Disabled
			1 => Enabled
			
		}
		8 => ENDTX {
			0 => Disabled
			1 => Enabled
			
		}
		19 => STARTED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		1 => STOPPED {
			0 => Disabled
			1 => Enabled
			
		}
		4 => ENDRX {
			0 => Disabled
			1 => Enabled
			
		}
		8 => ENDTX {
			0 => Disabled
			1 => Enabled
			
		}
		19 => STARTED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..3 => ENABLE {
			0x00 => Disabled
			0x07 => Enabled
			
		}
		
	}
	0x524 => reg32 FREQUENCY = {
		0..31 => FREQUENCY {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
			
		}
		
	}
	0x554 => reg32 CONFIG = {
		0 => ORDER {
			0 => MsbFirst
			1 => LsbFirst
			
		}
		1 => CPHA {
			0 => Leading
			1 => Trailing
			
		}
		2 => CPOL {
			0 => ActiveHigh
			1 => ActiveLow
			
		}
		
	}
	0x5C0 => reg32 ORC = {
		0..7 => ORC,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (GPIOTE @ 0x40006000) = {
	0x000 => reg32 TASKS_OUT[%s]
	0x100 => reg32 EVENTS_IN[%s]
	0x17C => reg32 EVENTS_PORT
	0x304 => reg32 INTENSET = {
		0 => IN0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => IN1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => IN2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => IN3 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => PORT {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => IN0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => IN1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => IN2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => IN3 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => PORT {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x510 => reg32 CONFIG[%s] = {
		0..1 => MODE {
			0x00 => Disabled
			0x01 => Event
			0x03 => Task
			
		}
		8..12 => PSEL,
		16..17 => POLARITY {
			0x00 => None
			0x01 => LoToHi
			0x02 => HiToLo
			0x03 => Toggle
			
		}
		20 => OUTINIT {
			0 => Low
			1 => High
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (ADC @ 0x40007000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x100 => reg32 EVENTS_END
	0x304 => reg32 INTENSET = {
		0 => END {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => END {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 BUSY = {
		0 => BUSY {
			0 => Ready
			1 => Busy
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..1 => ENABLE {
			0x00 => Disabled
			0x01 => Enabled
			
		}
		
	}
	0x504 => reg32 CONFIG = {
		0..1 => RES {
			0x00 => 8bit
			0x01 => 9bit
			0x02 => 10bit
			
		}
		2..4 => INPSEL {
			0x00 => AnalogInputNoPrescaling
			0x01 => AnalogInputTwoThirdsPrescaling
			0x02 => AnalogInputOneThirdPrescaling
			0x05 => SupplyTwoThirdsPrescaling
			0x06 => SupplyOneThirdPrescaling
			
		}
		5..6 => REFSEL {
			0x00 => VBG
			0x01 => External
			0x02 => SupplyOneHalfPrescaling
			0x03 => SupplyOneThirdPrescaling
			
		}
		8..15 => PSEL {
			0 => Disabled
			1 => AnalogInput0
			2 => AnalogInput1
			4 => AnalogInput2
			8 => AnalogInput3
			16 => AnalogInput4
			32 => AnalogInput5
			64 => AnalogInput6
			128 => AnalogInput7
			
		}
		16..17 => EXTREFSEL {
			0 => None
			1 => AnalogReference0
			2 => AnalogReference1
			
		}
		
	}
	0x508 => reg32 RESULT = {
		0..9 => RESULT,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TIMER0 @ 0x40008000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_COUNT
	0x00C => reg32 TASKS_CLEAR
	0x010 => reg32 TASKS_SHUTDOWN
	0x040 => reg32 TASKS_CAPTURE[%s]
	0x140 => reg32 EVENTS_COMPARE[%s]
	0x200 => reg32 SHORTS = {
		0 => COMPARE0_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		1 => COMPARE1_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		2 => COMPARE2_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		3 => COMPARE3_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		8 => COMPARE0_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		9 => COMPARE1_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		10 => COMPARE2_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		11 => COMPARE3_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 MODE = {
		0 => MODE {
			1 => Counter
			0 => Timer
			
		}
		
	}
	0x508 => reg32 BITMODE = {
		0..1 => BITMODE {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
			
		}
		
	}
	0x510 => reg32 PRESCALER = {
		0..3 => PRESCALER,
		
	}
	0x540 => reg32 CC[%s]
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TIMER1 @ 0x40009000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_COUNT
	0x00C => reg32 TASKS_CLEAR
	0x010 => reg32 TASKS_SHUTDOWN
	0x040 => reg32 TASKS_CAPTURE[%s]
	0x140 => reg32 EVENTS_COMPARE[%s]
	0x200 => reg32 SHORTS = {
		0 => COMPARE0_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		1 => COMPARE1_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		2 => COMPARE2_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		3 => COMPARE3_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		8 => COMPARE0_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		9 => COMPARE1_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		10 => COMPARE2_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		11 => COMPARE3_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 MODE = {
		0 => MODE {
			1 => Counter
			0 => Timer
			
		}
		
	}
	0x508 => reg32 BITMODE = {
		0..1 => BITMODE {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
			
		}
		
	}
	0x510 => reg32 PRESCALER = {
		0..3 => PRESCALER,
		
	}
	0x540 => reg32 CC[%s]
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TIMER2 @ 0x4000A000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_COUNT
	0x00C => reg32 TASKS_CLEAR
	0x010 => reg32 TASKS_SHUTDOWN
	0x040 => reg32 TASKS_CAPTURE[%s]
	0x140 => reg32 EVENTS_COMPARE[%s]
	0x200 => reg32 SHORTS = {
		0 => COMPARE0_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		1 => COMPARE1_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		2 => COMPARE2_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		3 => COMPARE3_CLEAR {
			0 => Disabled
			1 => Enabled
			
		}
		8 => COMPARE0_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		9 => COMPARE1_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		10 => COMPARE2_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		11 => COMPARE3_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 MODE = {
		0 => MODE {
			1 => Counter
			0 => Timer
			
		}
		
	}
	0x508 => reg32 BITMODE = {
		0..1 => BITMODE {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
			
		}
		
	}
	0x510 => reg32 PRESCALER = {
		0..3 => PRESCALER,
		
	}
	0x540 => reg32 CC[%s]
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (RTC0 @ 0x4000B000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_CLEAR
	0x00C => reg32 TASKS_TRIGOVRFLW
	0x100 => reg32 EVENTS_TICK
	0x104 => reg32 EVENTS_OVRFLW
	0x140 => reg32 EVENTS_COMPARE[%s]
	0x304 => reg32 INTENSET = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x340 => reg32 EVTEN = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x344 => reg32 EVTENSET = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x348 => reg32 EVTENCLR = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 COUNTER = {
		0..23 => COUNTER,
		
	}
	0x508 => reg32 PRESCALER = {
		0..11 => PRESCALER,
		
	}
	0x540 => reg32 CC[%s] = {
		0..23 => COMPARE,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (TEMP @ 0x4000C000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x100 => reg32 EVENTS_DATARDY
	0x304 => reg32 INTENSET = {
		0 => DATARDY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => DATARDY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x508 => reg32 TEMP
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (RNG @ 0x4000D000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x100 => reg32 EVENTS_VALRDY
	0x200 => reg32 SHORTS = {
		0 => VALRDY_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => VALRDY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => VALRDY {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 CONFIG = {
		0 => DERCEN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x508 => reg32 VALUE = {
		0..7 => VALUE,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (ECB @ 0x4000E000) = {
	0x000 => reg32 TASKS_STARTECB
	0x004 => reg32 TASKS_STOPECB
	0x100 => reg32 EVENTS_ENDECB
	0x104 => reg32 EVENTS_ERRORECB
	0x304 => reg32 INTENSET = {
		0 => ENDECB {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ERRORECB {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => ENDECB {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ERRORECB {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 ECBDATAPTR
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (AAR @ 0x4000F000) = {
	0x000 => reg32 TASKS_START
	0x008 => reg32 TASKS_STOP
	0x100 => reg32 EVENTS_END
	0x104 => reg32 EVENTS_RESOLVED
	0x108 => reg32 EVENTS_NOTRESOLVED
	0x304 => reg32 INTENSET = {
		0 => END {
			0 => Disabled
			1 => Enabled
			
		}
		1 => RESOLVED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => NOTRESOLVED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => END {
			0 => Disabled
			1 => Enabled
			
		}
		1 => RESOLVED {
			0 => Disabled
			1 => Enabled
			
		}
		2 => NOTRESOLVED {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 STATUS = {
		0..3 => STATUS,
		
	}
	0x500 => reg32 ENABLE = {
		0..1 => ENABLE {
			0x00 => Disabled
			0x03 => Enabled
			
		}
		
	}
	0x504 => reg32 NIRK = {
		0..4 => NIRK,
		
	}
	0x508 => reg32 IRKPTR
	0x510 => reg32 ADDRPTR
	0x514 => reg32 SCRATCHPTR
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (CCM @ 0x4000F000) = {
	0x000 => reg32 TASKS_KSGEN
	0x004 => reg32 TASKS_CRYPT
	0x008 => reg32 TASKS_STOP
	0x100 => reg32 EVENTS_ENDKSGEN
	0x104 => reg32 EVENTS_ENDCRYPT
	0x108 => reg32 EVENTS_ERROR
	0x200 => reg32 SHORTS = {
		0 => ENDKSGEN_CRYPT {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => ENDKSGEN {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ENDCRYPT {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => ENDKSGEN {
			0 => Disabled
			1 => Enabled
			
		}
		1 => ENDCRYPT {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ERROR {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 MICSTATUS = {
		0 => MICSTATUS {
			0 => CheckFailed
			1 => CheckPassed
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..1 => ENABLE {
			0x00 => Disabled
			0x02 => Enabled
			
		}
		
	}
	0x504 => reg32 MODE = {
		0 => MODE {
			0 => Encryption
			1 => Decryption
			
		}
		
	}
	0x508 => reg32 CNFPTR
	0x50C => reg32 INPTR
	0x510 => reg32 OUTPTR
	0x514 => reg32 SCRATCHPTR
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (WDT @ 0x40010000) = {
	0x000 => reg32 TASKS_START
	0x100 => reg32 EVENTS_TIMEOUT
	0x304 => reg32 INTENSET = {
		0 => TIMEOUT {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => TIMEOUT {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 RUNSTATUS = {
		0 => RUNSTATUS {
			0 => NotRunning
			1 => Running
			
		}
		
	}
	0x404 => reg32 REQSTATUS = {
		0 => RR0 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		1 => RR1 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		2 => RR2 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		3 => RR3 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		4 => RR4 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		5 => RR5 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		6 => RR6 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		7 => RR7 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
			
		}
		
	}
	0x504 => reg32 CRV
	0x508 => reg32 RREN = {
		0 => RR0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => RR1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => RR2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => RR3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => RR4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => RR5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => RR6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => RR7 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x50C => reg32 CONFIG = {
		0 => SLEEP {
			0 => Pause
			1 => Run
			
		}
		3 => HALT {
			0 => Pause
			1 => Run
			
		}
		
	}
	0x600 => reg32 RR[%s] = {
		0..31 => RR {
			0x6E524635 => Reload
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (RTC1 @ 0x40011000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_CLEAR
	0x00C => reg32 TASKS_TRIGOVRFLW
	0x100 => reg32 EVENTS_TICK
	0x104 => reg32 EVENTS_OVRFLW
	0x140 => reg32 EVENTS_COMPARE[%s]
	0x304 => reg32 INTENSET = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x340 => reg32 EVTEN = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x344 => reg32 EVTENSET = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x348 => reg32 EVTENCLR = {
		0 => TICK {
			0 => Disabled
			1 => Enabled
			
		}
		1 => OVRFLW {
			0 => Disabled
			1 => Enabled
			
		}
		16 => COMPARE0 {
			0 => Disabled
			1 => Enabled
			
		}
		17 => COMPARE1 {
			0 => Disabled
			1 => Enabled
			
		}
		18 => COMPARE2 {
			0 => Disabled
			1 => Enabled
			
		}
		19 => COMPARE3 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 COUNTER = {
		0..23 => COUNTER,
		
	}
	0x508 => reg32 PRESCALER = {
		0..11 => PRESCALER,
		
	}
	0x540 => reg32 CC[%s] = {
		0..23 => COMPARE,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (QDEC @ 0x40012000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_READCLRACC
	0x100 => reg32 EVENTS_SAMPLERDY
	0x104 => reg32 EVENTS_REPORTRDY
	0x108 => reg32 EVENTS_ACCOF
	0x200 => reg32 SHORTS = {
		0 => REPORTRDY_READCLRACC {
			0 => Disabled
			1 => Enabled
			
		}
		1 => SAMPLERDY_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => SAMPLERDY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => REPORTRDY {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ACCOF {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => SAMPLERDY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => REPORTRDY {
			0 => Disabled
			1 => Enabled
			
		}
		2 => ACCOF {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0 => ENABLE {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 LEDPOL = {
		0 => LEDPOL {
			0 => ActiveLow
			1 => ActiveHigh
			
		}
		
	}
	0x508 => reg32 SAMPLEPER = {
		0..2 => SAMPLEPER {
			0x00 => 128us
			0x01 => 256us
			0x02 => 512us
			0x03 => 1024us
			0x04 => 2048us
			0x05 => 4096us
			0x06 => 8192us
			0x07 => 16384us
			
		}
		
	}
	0x50C => reg32 SAMPLE = {
		0..31 => SAMPLE,
		
	}
	0x510 => reg32 REPORTPER = {
		0..2 => REPORTPER {
			0x00 => 10Smpl
			0x01 => 40Smpl
			0x02 => 80Smpl
			0x03 => 120Smpl
			0x04 => 160Smpl
			0x05 => 200Smpl
			0x06 => 240Smpl
			0x07 => 280Smpl
			
		}
		
	}
	0x514 => reg32 ACC
	0x518 => reg32 ACCREAD
	0x51C => reg32 PSELLED
	0x520 => reg32 PSELA
	0x524 => reg32 PSELB
	0x528 => reg32 DBFEN = {
		0 => DBFEN {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x540 => reg32 LEDPRE = {
		0..8 => LEDPRE,
		
	}
	0x544 => reg32 ACCDBL = {
		0..3 => ACCDBL,
		
	}
	0x548 => reg32 ACCDBLREAD = {
		0..3 => ACCDBLREAD,
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (LPCOMP @ 0x40013000) = {
	0x000 => reg32 TASKS_START
	0x004 => reg32 TASKS_STOP
	0x008 => reg32 TASKS_SAMPLE
	0x100 => reg32 EVENTS_READY
	0x104 => reg32 EVENTS_DOWN
	0x108 => reg32 EVENTS_UP
	0x10C => reg32 EVENTS_CROSS
	0x200 => reg32 SHORTS = {
		0 => READY_SAMPLE {
			0 => Disabled
			1 => Enabled
			
		}
		1 => READY_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		2 => DOWN_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		3 => UP_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CROSS_STOP {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x304 => reg32 INTENSET = {
		0 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => DOWN {
			0 => Disabled
			1 => Enabled
			
		}
		2 => UP {
			0 => Disabled
			1 => Enabled
			
		}
		3 => CROSS {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x308 => reg32 INTENCLR = {
		0 => READY {
			0 => Disabled
			1 => Enabled
			
		}
		1 => DOWN {
			0 => Disabled
			1 => Enabled
			
		}
		2 => UP {
			0 => Disabled
			1 => Enabled
			
		}
		3 => CROSS {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x400 => reg32 RESULT = {
		0 => RESULT {
			0 => Bellow
			1 => Above
			
		}
		
	}
	0x500 => reg32 ENABLE = {
		0..1 => ENABLE {
			0x00 => Disabled
			0x01 => Enabled
			
		}
		
	}
	0x504 => reg32 PSEL = {
		0..2 => PSEL {
			0 => AnalogInput0
			1 => AnalogInput1
			2 => AnalogInput2
			3 => AnalogInput3
			4 => AnalogInput4
			5 => AnalogInput5
			6 => AnalogInput6
			7 => AnalogInput7
			
		}
		
	}
	0x508 => reg32 REFSEL = {
		0..2 => REFSEL {
			0 => SupplyOneEighthPrescaling
			1 => SupplyTwoEighthsPrescaling
			2 => SupplyThreeEighthsPrescaling
			3 => SupplyFourEighthsPrescaling
			4 => SupplyFiveEighthsPrescaling
			5 => SupplySixEighthsPrescaling
			6 => SupplySevenEighthsPrescaling
			7 => ARef
			
		}
		
	}
	0x50C => reg32 EXTREFSEL = {
		0 => EXTREFSEL {
			0 => AnalogReference0
			1 => AnalogReference1
			
		}
		
	}
	0x520 => reg32 ANADETECT = {
		0..1 => ANADETECT {
			0 => Cross
			1 => Up
			2 => Down
			
		}
		
	}
	0xFFC => reg32 POWER = {
		0 => POWER {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	
}

ioregs! (SWI @ 0x40014000) = {
	0x000 => reg32 UNUSED
	
}

ioregs! (NVMC @ 0x4001E000) = {
	0x400 => reg32 READY = {
		0 => READY {
			0 => Busy
			1 => Ready
			
		}
		
	}
	0x504 => reg32 CONFIG = {
		0..1 => WEN {
			0x00 => Ren
			0x01 => Wen
			0x02 => Een
			
		}
		
	}
	0x508 => reg32 ERASEPAGE
	0x508 => reg32 ERASEPCR1
	0x50C => reg32 ERASEALL = {
		0 => ERASEALL {
			0 => NoOperation
			1 => Erase
			
		}
		
	}
	0x510 => reg32 ERASEPCR0
	0x514 => reg32 ERASEUICR = {
		0 => ERASEUICR {
			0 => NoOperation
			1 => Erase
			
		}
		
	}
	
}

ioregs! (PPI @ 0x4001F000) = {
	0x500 => reg32 CHEN = {
		0 => CH0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => CH1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => CH2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => CH3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CH4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => CH5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => CH6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => CH7 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => CH8 {
			0 => Disabled
			1 => Enabled
			
		}
		9 => CH9 {
			0 => Disabled
			1 => Enabled
			
		}
		10 => CH10 {
			0 => Disabled
			1 => Enabled
			
		}
		11 => CH11 {
			0 => Disabled
			1 => Enabled
			
		}
		12 => CH12 {
			0 => Disabled
			1 => Enabled
			
		}
		13 => CH13 {
			0 => Disabled
			1 => Enabled
			
		}
		14 => CH14 {
			0 => Disabled
			1 => Enabled
			
		}
		15 => CH15 {
			0 => Disabled
			1 => Enabled
			
		}
		20 => CH20 {
			0 => Disabled
			1 => Enabled
			
		}
		21 => CH21 {
			0 => Disabled
			1 => Enabled
			
		}
		22 => CH22 {
			0 => Disabled
			1 => Enabled
			
		}
		23 => CH23 {
			0 => Disabled
			1 => Enabled
			
		}
		24 => CH24 {
			0 => Disabled
			1 => Enabled
			
		}
		25 => CH25 {
			0 => Disabled
			1 => Enabled
			
		}
		26 => CH26 {
			0 => Disabled
			1 => Enabled
			
		}
		27 => CH27 {
			0 => Disabled
			1 => Enabled
			
		}
		28 => CH28 {
			0 => Disabled
			1 => Enabled
			
		}
		29 => CH29 {
			0 => Disabled
			1 => Enabled
			
		}
		30 => CH30 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => CH31 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x504 => reg32 CHENSET = {
		0 => CH0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => CH1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => CH2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => CH3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CH4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => CH5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => CH6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => CH7 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => CH8 {
			0 => Disabled
			1 => Enabled
			
		}
		9 => CH9 {
			0 => Disabled
			1 => Enabled
			
		}
		10 => CH10 {
			0 => Disabled
			1 => Enabled
			
		}
		11 => CH11 {
			0 => Disabled
			1 => Enabled
			
		}
		12 => CH12 {
			0 => Disabled
			1 => Enabled
			
		}
		13 => CH13 {
			0 => Disabled
			1 => Enabled
			
		}
		14 => CH14 {
			0 => Disabled
			1 => Enabled
			
		}
		15 => CH15 {
			0 => Disabled
			1 => Enabled
			
		}
		20 => CH20 {
			0 => Disabled
			1 => Enabled
			
		}
		21 => CH21 {
			0 => Disabled
			1 => Enabled
			
		}
		22 => CH22 {
			0 => Disabled
			1 => Enabled
			
		}
		23 => CH23 {
			0 => Disabled
			1 => Enabled
			
		}
		24 => CH24 {
			0 => Disabled
			1 => Enabled
			
		}
		25 => CH25 {
			0 => Disabled
			1 => Enabled
			
		}
		26 => CH26 {
			0 => Disabled
			1 => Enabled
			
		}
		27 => CH27 {
			0 => Disabled
			1 => Enabled
			
		}
		28 => CH28 {
			0 => Disabled
			1 => Enabled
			
		}
		29 => CH29 {
			0 => Disabled
			1 => Enabled
			
		}
		30 => CH30 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => CH31 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x508 => reg32 CHENCLR = {
		0 => CH0 {
			0 => Disabled
			1 => Enabled
			
		}
		1 => CH1 {
			0 => Disabled
			1 => Enabled
			
		}
		2 => CH2 {
			0 => Disabled
			1 => Enabled
			
		}
		3 => CH3 {
			0 => Disabled
			1 => Enabled
			
		}
		4 => CH4 {
			0 => Disabled
			1 => Enabled
			
		}
		5 => CH5 {
			0 => Disabled
			1 => Enabled
			
		}
		6 => CH6 {
			0 => Disabled
			1 => Enabled
			
		}
		7 => CH7 {
			0 => Disabled
			1 => Enabled
			
		}
		8 => CH8 {
			0 => Disabled
			1 => Enabled
			
		}
		9 => CH9 {
			0 => Disabled
			1 => Enabled
			
		}
		10 => CH10 {
			0 => Disabled
			1 => Enabled
			
		}
		11 => CH11 {
			0 => Disabled
			1 => Enabled
			
		}
		12 => CH12 {
			0 => Disabled
			1 => Enabled
			
		}
		13 => CH13 {
			0 => Disabled
			1 => Enabled
			
		}
		14 => CH14 {
			0 => Disabled
			1 => Enabled
			
		}
		15 => CH15 {
			0 => Disabled
			1 => Enabled
			
		}
		20 => CH20 {
			0 => Disabled
			1 => Enabled
			
		}
		21 => CH21 {
			0 => Disabled
			1 => Enabled
			
		}
		22 => CH22 {
			0 => Disabled
			1 => Enabled
			
		}
		23 => CH23 {
			0 => Disabled
			1 => Enabled
			
		}
		24 => CH24 {
			0 => Disabled
			1 => Enabled
			
		}
		25 => CH25 {
			0 => Disabled
			1 => Enabled
			
		}
		26 => CH26 {
			0 => Disabled
			1 => Enabled
			
		}
		27 => CH27 {
			0 => Disabled
			1 => Enabled
			
		}
		28 => CH28 {
			0 => Disabled
			1 => Enabled
			
		}
		29 => CH29 {
			0 => Disabled
			1 => Enabled
			
		}
		30 => CH30 {
			0 => Disabled
			1 => Enabled
			
		}
		31 => CH31 {
			0 => Disabled
			1 => Enabled
			
		}
		
	}
	0x800 => reg32 CHG[%s] = {
		0 => CH0 {
			0 => Excluded
			1 => Included
			
		}
		1 => CH1 {
			0 => Excluded
			1 => Included
			
		}
		2 => CH2 {
			0 => Excluded
			1 => Included
			
		}
		3 => CH3 {
			0 => Excluded
			1 => Included
			
		}
		4 => CH4 {
			0 => Excluded
			1 => Included
			
		}
		5 => CH5 {
			0 => Excluded
			1 => Included
			
		}
		6 => CH6 {
			0 => Excluded
			1 => Included
			
		}
		7 => CH7 {
			0 => Excluded
			1 => Included
			
		}
		8 => CH8 {
			0 => Excluded
			1 => Included
			
		}
		9 => CH9 {
			0 => Excluded
			1 => Included
			
		}
		10 => CH10 {
			0 => Excluded
			1 => Included
			
		}
		11 => CH11 {
			0 => Excluded
			1 => Included
			
		}
		12 => CH12 {
			0 => Excluded
			1 => Included
			
		}
		13 => CH13 {
			0 => Excluded
			1 => Included
			
		}
		14 => CH14 {
			0 => Excluded
			1 => Included
			
		}
		15 => CH15 {
			0 => Excluded
			1 => Included
			
		}
		20 => CH20 {
			0 => Excluded
			1 => Included
			
		}
		21 => CH21 {
			0 => Excluded
			1 => Included
			
		}
		22 => CH22 {
			0 => Excluded
			1 => Included
			
		}
		23 => CH23 {
			0 => Excluded
			1 => Included
			
		}
		24 => CH24 {
			0 => Excluded
			1 => Included
			
		}
		25 => CH25 {
			0 => Excluded
			1 => Included
			
		}
		26 => CH26 {
			0 => Excluded
			1 => Included
			
		}
		27 => CH27 {
			0 => Excluded
			1 => Included
			
		}
		28 => CH28 {
			0 => Excluded
			1 => Included
			
		}
		29 => CH29 {
			0 => Excluded
			1 => Included
			
		}
		30 => CH30 {
			0 => Excluded
			1 => Included
			
		}
		31 => CH31 {
			0 => Excluded
			1 => Included
			
		}
		
	}
	
}

ioregs! (FICR @ 0x10000000) = {
	0x010 => reg32 CODEPAGESIZE
	0x014 => reg32 CODESIZE
	0x028 => reg32 CLENR0
	0x02C => reg32 PPFC = {
		0..7 => PPFC {
			0xFF => NotPresent
			0x00 => Present
			
		}
		
	}
	0x034 => reg32 NUMRAMBLOCK
	0x038 => reg32 SIZERAMBLOCKS
	0x038 => reg32 SIZERAMBLOCK[%s]
	0x05C => reg32 CONFIGID = {
		0..15 => HWID,
		16..31 => FWID,
		
	}
	0x060 => reg32 DEVICEID[%s]
	0x080 => reg32 ER[%s]
	0x090 => reg32 IR[%s]
	0x0A0 => reg32 DEVICEADDRTYPE = {
		0 => DEVICEADDRTYPE {
			0 => Public
			1 => Random
			
		}
		
	}
	0x0A4 => reg32 DEVICEADDR[%s]
	0x0AC => reg32 OVERRIDEEN = {
		0 => NRF_1MBIT {
			0 => Override
			1 => NotOverride
			
		}
		3 => BLE_1MBIT {
			0 => Override
			1 => NotOverride
			
		}
		
	}
	0x0B0 => reg32 NRF_1MBIT[%s]
	0x0EC => reg32 BLE_1MBIT[%s]
	
}

ioregs! (UICR @ 0x10001000) = {
	0x000 => reg32 CLENR0
	0x004 => reg32 RBPCONF = {
		0..7 => PR0 {
			0xFF => Disabled
			0x00 => Enabled
			
		}
		8..15 => PALL {
			0xFF => Disabled
			0x00 => Enabled
			
		}
		
	}
	0x008 => reg32 XTALFREQ = {
		0..7 => XTALFREQ {
			0xFF => 16MHz
			0x00 => 32MHz
			
		}
		
	}
	0x010 => reg32 FWID = {
		0..15 => FWID,
		
	}
	0x014 => reg32 BOOTLOADERADDR
	0x014 => reg32 NRFFW[%s]
	0x050 => reg32 NRFHW[%s]
	0x080 => reg32 CUSTOMER[%s]
	
}

ioregs! (GPIO @ 0x50000000) = {
	0x504 => reg32 OUT = {
		0 => PIN0 {
			0 => Low
			1 => High
			
		}
		1 => PIN1 {
			0 => Low
			1 => High
			
		}
		2 => PIN2 {
			0 => Low
			1 => High
			
		}
		3 => PIN3 {
			0 => Low
			1 => High
			
		}
		4 => PIN4 {
			0 => Low
			1 => High
			
		}
		5 => PIN5 {
			0 => Low
			1 => High
			
		}
		6 => PIN6 {
			0 => Low
			1 => High
			
		}
		7 => PIN7 {
			0 => Low
			1 => High
			
		}
		8 => PIN8 {
			0 => Low
			1 => High
			
		}
		9 => PIN9 {
			0 => Low
			1 => High
			
		}
		10 => PIN10 {
			0 => Low
			1 => High
			
		}
		11 => PIN11 {
			0 => Low
			1 => High
			
		}
		12 => PIN12 {
			0 => Low
			1 => High
			
		}
		13 => PIN13 {
			0 => Low
			1 => High
			
		}
		14 => PIN14 {
			0 => Low
			1 => High
			
		}
		15 => PIN15 {
			0 => Low
			1 => High
			
		}
		16 => PIN16 {
			0 => Low
			1 => High
			
		}
		17 => PIN17 {
			0 => Low
			1 => High
			
		}
		18 => PIN18 {
			0 => Low
			1 => High
			
		}
		19 => PIN19 {
			0 => Low
			1 => High
			
		}
		20 => PIN20 {
			0 => Low
			1 => High
			
		}
		21 => PIN21 {
			0 => Low
			1 => High
			
		}
		22 => PIN22 {
			0 => Low
			1 => High
			
		}
		23 => PIN23 {
			0 => Low
			1 => High
			
		}
		24 => PIN24 {
			0 => Low
			1 => High
			
		}
		25 => PIN25 {
			0 => Low
			1 => High
			
		}
		26 => PIN26 {
			0 => Low
			1 => High
			
		}
		27 => PIN27 {
			0 => Low
			1 => High
			
		}
		28 => PIN28 {
			0 => Low
			1 => High
			
		}
		29 => PIN29 {
			0 => Low
			1 => High
			
		}
		30 => PIN30 {
			0 => Low
			1 => High
			
		}
		31 => PIN31 {
			0 => Low
			1 => High
			
		}
		
	}
	0x508 => reg32 OUTSET = {
		0 => PIN0 {
			0 => Low
			1 => High
			
		}
		1 => PIN1 {
			0 => Low
			1 => High
			
		}
		2 => PIN2 {
			0 => Low
			1 => High
			
		}
		3 => PIN3 {
			0 => Low
			1 => High
			
		}
		4 => PIN4 {
			0 => Low
			1 => High
			
		}
		5 => PIN5 {
			0 => Low
			1 => High
			
		}
		6 => PIN6 {
			0 => Low
			1 => High
			
		}
		7 => PIN7 {
			0 => Low
			1 => High
			
		}
		8 => PIN8 {
			0 => Low
			1 => High
			
		}
		9 => PIN9 {
			0 => Low
			1 => High
			
		}
		10 => PIN10 {
			0 => Low
			1 => High
			
		}
		11 => PIN11 {
			0 => Low
			1 => High
			
		}
		12 => PIN12 {
			0 => Low
			1 => High
			
		}
		13 => PIN13 {
			0 => Low
			1 => High
			
		}
		14 => PIN14 {
			0 => Low
			1 => High
			
		}
		15 => PIN15 {
			0 => Low
			1 => High
			
		}
		16 => PIN16 {
			0 => Low
			1 => High
			
		}
		17 => PIN17 {
			0 => Low
			1 => High
			
		}
		18 => PIN18 {
			0 => Low
			1 => High
			
		}
		19 => PIN19 {
			0 => Low
			1 => High
			
		}
		20 => PIN20 {
			0 => Low
			1 => High
			
		}
		21 => PIN21 {
			0 => Low
			1 => High
			
		}
		22 => PIN22 {
			0 => Low
			1 => High
			
		}
		23 => PIN23 {
			0 => Low
			1 => High
			
		}
		24 => PIN24 {
			0 => Low
			1 => High
			
		}
		25 => PIN25 {
			0 => Low
			1 => High
			
		}
		26 => PIN26 {
			0 => Low
			1 => High
			
		}
		27 => PIN27 {
			0 => Low
			1 => High
			
		}
		28 => PIN28 {
			0 => Low
			1 => High
			
		}
		29 => PIN29 {
			0 => Low
			1 => High
			
		}
		30 => PIN30 {
			0 => Low
			1 => High
			
		}
		31 => PIN31 {
			0 => Low
			1 => High
			
		}
		
	}
	0x50C => reg32 OUTCLR = {
		0 => PIN0 {
			0 => Low
			1 => High
			
		}
		1 => PIN1 {
			0 => Low
			1 => High
			
		}
		2 => PIN2 {
			0 => Low
			1 => High
			
		}
		3 => PIN3 {
			0 => Low
			1 => High
			
		}
		4 => PIN4 {
			0 => Low
			1 => High
			
		}
		5 => PIN5 {
			0 => Low
			1 => High
			
		}
		6 => PIN6 {
			0 => Low
			1 => High
			
		}
		7 => PIN7 {
			0 => Low
			1 => High
			
		}
		8 => PIN8 {
			0 => Low
			1 => High
			
		}
		9 => PIN9 {
			0 => Low
			1 => High
			
		}
		10 => PIN10 {
			0 => Low
			1 => High
			
		}
		11 => PIN11 {
			0 => Low
			1 => High
			
		}
		12 => PIN12 {
			0 => Low
			1 => High
			
		}
		13 => PIN13 {
			0 => Low
			1 => High
			
		}
		14 => PIN14 {
			0 => Low
			1 => High
			
		}
		15 => PIN15 {
			0 => Low
			1 => High
			
		}
		16 => PIN16 {
			0 => Low
			1 => High
			
		}
		17 => PIN17 {
			0 => Low
			1 => High
			
		}
		18 => PIN18 {
			0 => Low
			1 => High
			
		}
		19 => PIN19 {
			0 => Low
			1 => High
			
		}
		20 => PIN20 {
			0 => Low
			1 => High
			
		}
		21 => PIN21 {
			0 => Low
			1 => High
			
		}
		22 => PIN22 {
			0 => Low
			1 => High
			
		}
		23 => PIN23 {
			0 => Low
			1 => High
			
		}
		24 => PIN24 {
			0 => Low
			1 => High
			
		}
		25 => PIN25 {
			0 => Low
			1 => High
			
		}
		26 => PIN26 {
			0 => Low
			1 => High
			
		}
		27 => PIN27 {
			0 => Low
			1 => High
			
		}
		28 => PIN28 {
			0 => Low
			1 => High
			
		}
		29 => PIN29 {
			0 => Low
			1 => High
			
		}
		30 => PIN30 {
			0 => Low
			1 => High
			
		}
		31 => PIN31 {
			0 => Low
			1 => High
			
		}
		
	}
	0x510 => reg32 IN = {
		0 => PIN0 {
			0 => Low
			1 => High
			
		}
		1 => PIN1 {
			0 => Low
			1 => High
			
		}
		2 => PIN2 {
			0 => Low
			1 => High
			
		}
		3 => PIN3 {
			0 => Low
			1 => High
			
		}
		4 => PIN4 {
			0 => Low
			1 => High
			
		}
		5 => PIN5 {
			0 => Low
			1 => High
			
		}
		6 => PIN6 {
			0 => Low
			1 => High
			
		}
		7 => PIN7 {
			0 => Low
			1 => High
			
		}
		8 => PIN8 {
			0 => Low
			1 => High
			
		}
		9 => PIN9 {
			0 => Low
			1 => High
			
		}
		10 => PIN10 {
			0 => Low
			1 => High
			
		}
		11 => PIN11 {
			0 => Low
			1 => High
			
		}
		12 => PIN12 {
			0 => Low
			1 => High
			
		}
		13 => PIN13 {
			0 => Low
			1 => High
			
		}
		14 => PIN14 {
			0 => Low
			1 => High
			
		}
		15 => PIN15 {
			0 => Low
			1 => High
			
		}
		16 => PIN16 {
			0 => Low
			1 => High
			
		}
		17 => PIN17 {
			0 => Low
			1 => High
			
		}
		18 => PIN18 {
			0 => Low
			1 => High
			
		}
		19 => PIN19 {
			0 => Low
			1 => High
			
		}
		20 => PIN20 {
			0 => Low
			1 => High
			
		}
		21 => PIN21 {
			0 => Low
			1 => High
			
		}
		22 => PIN22 {
			0 => Low
			1 => High
			
		}
		23 => PIN23 {
			0 => Low
			1 => High
			
		}
		24 => PIN24 {
			0 => Low
			1 => High
			
		}
		25 => PIN25 {
			0 => Low
			1 => High
			
		}
		26 => PIN26 {
			0 => Low
			1 => High
			
		}
		27 => PIN27 {
			0 => Low
			1 => High
			
		}
		28 => PIN28 {
			0 => Low
			1 => High
			
		}
		29 => PIN29 {
			0 => Low
			1 => High
			
		}
		30 => PIN30 {
			0 => Low
			1 => High
			
		}
		31 => PIN31 {
			0 => Low
			1 => High
			
		}
		
	}
	0x514 => reg32 DIR = {
		0 => PIN0 {
			0 => Input
			1 => Output
			
		}
		1 => PIN1 {
			0 => Input
			1 => Output
			
		}
		2 => PIN2 {
			0 => Input
			1 => Output
			
		}
		3 => PIN3 {
			0 => Input
			1 => Output
			
		}
		4 => PIN4 {
			0 => Input
			1 => Output
			
		}
		5 => PIN5 {
			0 => Input
			1 => Output
			
		}
		6 => PIN6 {
			0 => Input
			1 => Output
			
		}
		7 => PIN7 {
			0 => Input
			1 => Output
			
		}
		8 => PIN8 {
			0 => Input
			1 => Output
			
		}
		9 => PIN9 {
			0 => Input
			1 => Output
			
		}
		10 => PIN10 {
			0 => Input
			1 => Output
			
		}
		11 => PIN11 {
			0 => Input
			1 => Output
			
		}
		12 => PIN12 {
			0 => Input
			1 => Output
			
		}
		13 => PIN13 {
			0 => Input
			1 => Output
			
		}
		14 => PIN14 {
			0 => Input
			1 => Output
			
		}
		15 => PIN15 {
			0 => Input
			1 => Output
			
		}
		16 => PIN16 {
			0 => Input
			1 => Output
			
		}
		17 => PIN17 {
			0 => Input
			1 => Output
			
		}
		18 => PIN18 {
			0 => Input
			1 => Output
			
		}
		19 => PIN19 {
			0 => Input
			1 => Output
			
		}
		20 => PIN20 {
			0 => Input
			1 => Output
			
		}
		21 => PIN21 {
			0 => Input
			1 => Output
			
		}
		22 => PIN22 {
			0 => Input
			1 => Output
			
		}
		23 => PIN23 {
			0 => Input
			1 => Output
			
		}
		24 => PIN24 {
			0 => Input
			1 => Output
			
		}
		25 => PIN25 {
			0 => Input
			1 => Output
			
		}
		26 => PIN26 {
			0 => Input
			1 => Output
			
		}
		27 => PIN27 {
			0 => Input
			1 => Output
			
		}
		28 => PIN28 {
			0 => Input
			1 => Output
			
		}
		29 => PIN29 {
			0 => Input
			1 => Output
			
		}
		30 => PIN30 {
			0 => Input
			1 => Output
			
		}
		31 => PIN31 {
			0 => Input
			1 => Output
			
		}
		
	}
	0x518 => reg32 DIRSET = {
		0 => PIN0 {
			0 => Input
			1 => Output
			
		}
		1 => PIN1 {
			0 => Input
			1 => Output
			
		}
		2 => PIN2 {
			0 => Input
			1 => Output
			
		}
		3 => PIN3 {
			0 => Input
			1 => Output
			
		}
		4 => PIN4 {
			0 => Input
			1 => Output
			
		}
		5 => PIN5 {
			0 => Input
			1 => Output
			
		}
		6 => PIN6 {
			0 => Input
			1 => Output
			
		}
		7 => PIN7 {
			0 => Input
			1 => Output
			
		}
		8 => PIN8 {
			0 => Input
			1 => Output
			
		}
		9 => PIN9 {
			0 => Input
			1 => Output
			
		}
		10 => PIN10 {
			0 => Input
			1 => Output
			
		}
		11 => PIN11 {
			0 => Input
			1 => Output
			
		}
		12 => PIN12 {
			0 => Input
			1 => Output
			
		}
		13 => PIN13 {
			0 => Input
			1 => Output
			
		}
		14 => PIN14 {
			0 => Input
			1 => Output
			
		}
		15 => PIN15 {
			0 => Input
			1 => Output
			
		}
		16 => PIN16 {
			0 => Input
			1 => Output
			
		}
		17 => PIN17 {
			0 => Input
			1 => Output
			
		}
		18 => PIN18 {
			0 => Input
			1 => Output
			
		}
		19 => PIN19 {
			0 => Input
			1 => Output
			
		}
		20 => PIN20 {
			0 => Input
			1 => Output
			
		}
		21 => PIN21 {
			0 => Input
			1 => Output
			
		}
		22 => PIN22 {
			0 => Input
			1 => Output
			
		}
		23 => PIN23 {
			0 => Input
			1 => Output
			
		}
		24 => PIN24 {
			0 => Input
			1 => Output
			
		}
		25 => PIN25 {
			0 => Input
			1 => Output
			
		}
		26 => PIN26 {
			0 => Input
			1 => Output
			
		}
		27 => PIN27 {
			0 => Input
			1 => Output
			
		}
		28 => PIN28 {
			0 => Input
			1 => Output
			
		}
		29 => PIN29 {
			0 => Input
			1 => Output
			
		}
		30 => PIN30 {
			0 => Input
			1 => Output
			
		}
		31 => PIN31 {
			0 => Input
			1 => Output
			
		}
		
	}
	0x51C => reg32 DIRCLR = {
		0 => PIN0 {
			0 => Input
			1 => Output
			
		}
		1 => PIN1 {
			0 => Input
			1 => Output
			
		}
		2 => PIN2 {
			0 => Input
			1 => Output
			
		}
		3 => PIN3 {
			0 => Input
			1 => Output
			
		}
		4 => PIN4 {
			0 => Input
			1 => Output
			
		}
		5 => PIN5 {
			0 => Input
			1 => Output
			
		}
		6 => PIN6 {
			0 => Input
			1 => Output
			
		}
		7 => PIN7 {
			0 => Input
			1 => Output
			
		}
		8 => PIN8 {
			0 => Input
			1 => Output
			
		}
		9 => PIN9 {
			0 => Input
			1 => Output
			
		}
		10 => PIN10 {
			0 => Input
			1 => Output
			
		}
		11 => PIN11 {
			0 => Input
			1 => Output
			
		}
		12 => PIN12 {
			0 => Input
			1 => Output
			
		}
		13 => PIN13 {
			0 => Input
			1 => Output
			
		}
		14 => PIN14 {
			0 => Input
			1 => Output
			
		}
		15 => PIN15 {
			0 => Input
			1 => Output
			
		}
		16 => PIN16 {
			0 => Input
			1 => Output
			
		}
		17 => PIN17 {
			0 => Input
			1 => Output
			
		}
		18 => PIN18 {
			0 => Input
			1 => Output
			
		}
		19 => PIN19 {
			0 => Input
			1 => Output
			
		}
		20 => PIN20 {
			0 => Input
			1 => Output
			
		}
		21 => PIN21 {
			0 => Input
			1 => Output
			
		}
		22 => PIN22 {
			0 => Input
			1 => Output
			
		}
		23 => PIN23 {
			0 => Input
			1 => Output
			
		}
		24 => PIN24 {
			0 => Input
			1 => Output
			
		}
		25 => PIN25 {
			0 => Input
			1 => Output
			
		}
		26 => PIN26 {
			0 => Input
			1 => Output
			
		}
		27 => PIN27 {
			0 => Input
			1 => Output
			
		}
		28 => PIN28 {
			0 => Input
			1 => Output
			
		}
		29 => PIN29 {
			0 => Input
			1 => Output
			
		}
		30 => PIN30 {
			0 => Input
			1 => Output
			
		}
		31 => PIN31 {
			0 => Input
			1 => Output
			
		}
		
	}
	0x700 => reg32 PIN_CNF[%s] = {
		0 => DIR {
			0 => Input
			1 => Output
			
		}
		1 => INPUT {
			0 => Connect
			1 => Disconnect
			
		}
		2..3 => PULL {
			0x00 => Disabled
			0x01 => Pulldown
			0x03 => Pullup
			
		}
		8..10 => DRIVE {
			0x00 => S0S1
			0x01 => H0S1
			0x02 => S0H1
			0x03 => H0H1
			0x04 => D0S1
			0x05 => D0H1
			0x06 => S0D1
			0x07 => H0D1
			
		}
		16..17 => SENSE {
			0x00 => Disabled
			0x02 => High
			0x03 => Low
			
		}
		
	}
	
}
