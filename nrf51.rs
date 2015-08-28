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


ioregs! (POWER @ 0x40000000 = {
	0x078 => reg32 tasks_constlat {}
	0x07C => reg32 tasks_lowpwr {}
	0x108 => reg32 events_pofwarn {}
	0x304 => reg32 intenset {
		2 => pofwarn {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		2 => pofwarn {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 resetreas {
		0 => resetpin {
			0 => NotDetected
			1 => Detected
		}
		1 => dog {
			0 => NotDetected
			1 => Detected
		}
		2 => sreq {
			0 => NotDetected
			1 => Detected
		}
		3 => lockup {
			0 => NotDetected
			1 => Detected
		}
		16 => off {
			0 => NotDetected
			1 => Detected
		}
		17 => lpcomp {
			0 => NotDetected
			1 => Detected
		}
		18 => dif {
			0 => NotDetected
			1 => Detected
		}
	}
	0x428 => reg32 ramstatus {
		0 => ramblock0 {
			0 => Off
			1 => On
		}
		1 => ramblock1 {
			0 => Off
			1 => On
		}
		2 => ramblock2 {
			0 => Off
			1 => On
		}
		3 => ramblock3 {
			0 => Off
			1 => On
		}
	}
	0x500 => reg32 systemoff {
		0 => systemoff {
			1 => Enter
		}
	}
	0x510 => reg32 pofcon {
		0 => pof {
			0 => Disabled
			1 => Enabled
		}
		1..2 => threshold {
			0x00 => V21
			0x01 => V23
			0x02 => V25
			0x03 => V27
		}
	}
	0x51C => reg32 gpregret {
		0..7 => gpregret,
	}
	0x524 => reg32 ramon {
		0 => onram0 {
			0 => RAM0Off
			1 => RAM0On
		}
		1 => onram1 {
			0 => RAM1Off
			1 => RAM1On
		}
		16 => offram0 {
			0 => RAM0Off
			1 => RAM0On
		}
		17 => offram1 {
			0 => RAM1Off
			1 => RAM1On
		}
	}
	0x544 => reg32 reset {
		0 => reset {
			0 => Disabled
			1 => Enabled
		}
	}
	0x554 => reg32 ramonb {
		0 => onram2 {
			0 => RAM2Off
			1 => RAM2On
		}
		1 => onram3 {
			0 => RAM3Off
			1 => RAM3On
		}
		16 => offram2 {
			0 => RAM2Off
			1 => RAM2On
		}
		17 => offram3 {
			0 => RAM3Off
			1 => RAM3On
		}
	}
	0x578 => reg32 dcdcen {
		0 => dcdcen {
			0 => Disabled
			1 => Enabled
		}
	}
	0xA08 => reg32 dcdcforce {
		0 => forceoff {
			0 => NoForce
			1 => Force
		}
		1 => forceon {
			0 => NoForce
			1 => Force
		}
	}
});

ioregs! (CLOCK @ 0x40000000 = {
	0x000 => reg32 tasks_hfclkstart {}
	0x004 => reg32 tasks_hfclkstop {}
	0x008 => reg32 tasks_lfclkstart {}
	0x00C => reg32 tasks_lfclkstop {}
	0x010 => reg32 tasks_cal {}
	0x014 => reg32 tasks_ctstart {}
	0x018 => reg32 tasks_ctstop {}
	0x100 => reg32 events_hfclkstarted {}
	0x104 => reg32 events_lfclkstarted {}
	0x10C => reg32 events_done {}
	0x110 => reg32 events_ctto {}
	0x304 => reg32 intenset {
		0 => hfclkstarted {
			0 => Disabled
			1 => Enabled
		}
		1 => lfclkstarted {
			0 => Disabled
			1 => Enabled
		}
		3 => done {
			0 => Disabled
			1 => Enabled
		}
		4 => ctto {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => hfclkstarted {
			0 => Disabled
			1 => Enabled
		}
		1 => lfclkstarted {
			0 => Disabled
			1 => Enabled
		}
		3 => done {
			0 => Disabled
			1 => Enabled
		}
		4 => ctto {
			0 => Disabled
			1 => Enabled
		}
	}
	0x408 => reg32 hfclkrun {
		0 => status {
			0 => NotTriggered
			1 => Triggered
		}
	}
	0x40C => reg32 hfclkstat {
		0 => src {
			0 => RC
			1 => Xtal
		}
		16 => state {
			0 => NotRunning
			1 => Running
		}
	}
	0x414 => reg32 lfclkrun {
		0 => status {
			0 => NotTriggered
			1 => Triggered
		}
	}
	0x418 => reg32 lfclkstat {
		0..1 => src {
			0 => RC
			1 => Xtal
			2 => Synth
		}
		16 => state {
			0 => NotRunning
			1 => Running
		}
	}
	0x41C => reg32 lfclksrccopy {
		0..1 => src {
			0 => RC
			1 => Xtal
			2 => Synth
		}
	}
	0x518 => reg32 lfclksrc {
		0..1 => src {
			0 => RC
			1 => Xtal
			2 => Synth
		}
	}
	0x538 => reg32 ctiv {
		0..6 => ctiv,
	}
	0x550 => reg32 xtalfreq {
		0..7 => xtalfreq {
			0xFF => 16MHz
			0x00 => 32MHz
		}
	}
});

ioregs! (MPU @ 0x40000000 = {
	0x528 => reg32 perr0 {
		0 => power_clock {
			1 => InRegion0
			0 => InRegion1
		}
		1 => radio {
			1 => InRegion0
			0 => InRegion1
		}
		2 => uart0 {
			1 => InRegion0
			0 => InRegion1
		}
		3 => spi0_twi0 {
			1 => InRegion0
			0 => InRegion1
		}
		4 => spi1_twi1 {
			1 => InRegion0
			0 => InRegion1
		}
		6 => gpiote {
			1 => InRegion0
			0 => InRegion1
		}
		7 => adc {
			1 => InRegion0
			0 => InRegion1
		}
		8 => timer0 {
			1 => InRegion0
			0 => InRegion1
		}
		9 => timer1 {
			1 => InRegion0
			0 => InRegion1
		}
		10 => timer2 {
			1 => InRegion0
			0 => InRegion1
		}
		11 => rtc0 {
			1 => InRegion0
			0 => InRegion1
		}
		12 => temp {
			1 => InRegion0
			0 => InRegion1
		}
		13 => rng {
			1 => InRegion0
			0 => InRegion1
		}
		14 => ecb {
			1 => InRegion0
			0 => InRegion1
		}
		15 => ccm_aar {
			1 => InRegion0
			0 => InRegion1
		}
		16 => wdt {
			1 => InRegion0
			0 => InRegion1
		}
		17 => rtc1 {
			1 => InRegion0
			0 => InRegion1
		}
		18 => qdec {
			1 => InRegion0
			0 => InRegion1
		}
		19 => lpcomp {
			1 => InRegion0
			0 => InRegion1
		}
		30 => nvmc {
			1 => InRegion0
			0 => InRegion1
		}
		31 => ppi {
			1 => InRegion0
			0 => InRegion1
		}
	}
	0x52C => reg32 rlenr0 {}
	0x600 => reg32 protenset0 {
		0 => protreg0 {
			0 => Disabled
			1 => Enabled
		}
		1 => protreg1 {
			0 => Disabled
			1 => Enabled
		}
		2 => protreg2 {
			0 => Disabled
			1 => Enabled
		}
		3 => protreg3 {
			0 => Disabled
			1 => Enabled
		}
		4 => protreg4 {
			0 => Disabled
			1 => Enabled
		}
		5 => protreg5 {
			0 => Disabled
			1 => Enabled
		}
		6 => protreg6 {
			0 => Disabled
			1 => Enabled
		}
		7 => protreg7 {
			0 => Disabled
			1 => Enabled
		}
		8 => protreg8 {
			0 => Disabled
			1 => Enabled
		}
		9 => protreg9 {
			0 => Disabled
			1 => Enabled
		}
		10 => protreg10 {
			0 => Disabled
			1 => Enabled
		}
		11 => protreg11 {
			0 => Disabled
			1 => Enabled
		}
		12 => protreg12 {
			0 => Disabled
			1 => Enabled
		}
		13 => protreg13 {
			0 => Disabled
			1 => Enabled
		}
		14 => protreg14 {
			0 => Disabled
			1 => Enabled
		}
		15 => protreg15 {
			0 => Disabled
			1 => Enabled
		}
		16 => protreg16 {
			0 => Disabled
			1 => Enabled
		}
		17 => protreg17 {
			0 => Disabled
			1 => Enabled
		}
		18 => protreg18 {
			0 => Disabled
			1 => Enabled
		}
		19 => protreg19 {
			0 => Disabled
			1 => Enabled
		}
		20 => protreg20 {
			0 => Disabled
			1 => Enabled
		}
		21 => protreg21 {
			0 => Disabled
			1 => Enabled
		}
		22 => protreg22 {
			0 => Disabled
			1 => Enabled
		}
		23 => protreg23 {
			0 => Disabled
			1 => Enabled
		}
		24 => protreg24 {
			0 => Disabled
			1 => Enabled
		}
		25 => protreg25 {
			0 => Disabled
			1 => Enabled
		}
		26 => protreg26 {
			0 => Disabled
			1 => Enabled
		}
		27 => protreg27 {
			0 => Disabled
			1 => Enabled
		}
		28 => protreg28 {
			0 => Disabled
			1 => Enabled
		}
		29 => protreg29 {
			0 => Disabled
			1 => Enabled
		}
		30 => protreg30 {
			0 => Disabled
			1 => Enabled
		}
		31 => protreg31 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x604 => reg32 protenset1 {
		0 => protreg32 {
			0 => Disabled
			1 => Enabled
		}
		1 => protreg33 {
			0 => Disabled
			1 => Enabled
		}
		2 => protreg34 {
			0 => Disabled
			1 => Enabled
		}
		3 => protreg35 {
			0 => Disabled
			1 => Enabled
		}
		4 => protreg36 {
			0 => Disabled
			1 => Enabled
		}
		5 => protreg37 {
			0 => Disabled
			1 => Enabled
		}
		6 => protreg38 {
			0 => Disabled
			1 => Enabled
		}
		7 => protreg39 {
			0 => Disabled
			1 => Enabled
		}
		8 => protreg40 {
			0 => Disabled
			1 => Enabled
		}
		9 => protreg41 {
			0 => Disabled
			1 => Enabled
		}
		10 => protreg42 {
			0 => Disabled
			1 => Enabled
		}
		11 => protreg43 {
			0 => Disabled
			1 => Enabled
		}
		12 => protreg44 {
			0 => Disabled
			1 => Enabled
		}
		13 => protreg45 {
			0 => Disabled
			1 => Enabled
		}
		14 => protreg46 {
			0 => Disabled
			1 => Enabled
		}
		15 => protreg47 {
			0 => Disabled
			1 => Enabled
		}
		16 => protreg48 {
			0 => Disabled
			1 => Enabled
		}
		17 => protreg49 {
			0 => Disabled
			1 => Enabled
		}
		18 => protreg50 {
			0 => Disabled
			1 => Enabled
		}
		19 => protreg51 {
			0 => Disabled
			1 => Enabled
		}
		20 => protreg52 {
			0 => Disabled
			1 => Enabled
		}
		21 => protreg53 {
			0 => Disabled
			1 => Enabled
		}
		22 => protreg54 {
			0 => Disabled
			1 => Enabled
		}
		23 => protreg55 {
			0 => Disabled
			1 => Enabled
		}
		24 => protreg56 {
			0 => Disabled
			1 => Enabled
		}
		25 => protreg57 {
			0 => Disabled
			1 => Enabled
		}
		26 => protreg58 {
			0 => Disabled
			1 => Enabled
		}
		27 => protreg59 {
			0 => Disabled
			1 => Enabled
		}
		28 => protreg60 {
			0 => Disabled
			1 => Enabled
		}
		29 => protreg61 {
			0 => Disabled
			1 => Enabled
		}
		30 => protreg62 {
			0 => Disabled
			1 => Enabled
		}
		31 => protreg63 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x608 => reg32 disableindebug {
		0 => disableindebug {
			0 => Enabled
			1 => Disabled
		}
	}
	0x60C => reg32 protblocksize {
		0..1 => protblocksize {
			0 => 4k
		}
	}
});

ioregs! (AMLI @ 0x40000000);

ioregs! (RADIO @ 0x40001000 = {
	0x000 => reg32 tasks_txen {}
	0x004 => reg32 tasks_rxen {}
	0x008 => reg32 tasks_start {}
	0x00C => reg32 tasks_stop {}
	0x010 => reg32 tasks_disable {}
	0x014 => reg32 tasks_rssistart {}
	0x018 => reg32 tasks_rssistop {}
	0x01C => reg32 tasks_bcstart {}
	0x020 => reg32 tasks_bcstop {}
	0x100 => reg32 events_ready {}
	0x104 => reg32 events_address {}
	0x108 => reg32 events_payload {}
	0x10C => reg32 events_end {}
	0x110 => reg32 events_disabled {}
	0x114 => reg32 events_devmatch {}
	0x118 => reg32 events_devmiss {}
	0x11C => reg32 events_rssiend {}
	0x128 => reg32 events_bcmatch {}
	0x200 => reg32 shorts {
		0 => ready_start {
			0 => Disabled
			1 => Enabled
		}
		1 => end_disable {
			0 => Disabled
			1 => Enabled
		}
		2 => disabled_txen {
			0 => Disabled
			1 => Enabled
		}
		3 => disabled_rxen {
			0 => Disabled
			1 => Enabled
		}
		4 => address_rssistart {
			0 => Disabled
			1 => Enabled
		}
		5 => end_start {
			0 => Disabled
			1 => Enabled
		}
		6 => address_bcstart {
			0 => Disabled
			1 => Enabled
		}
		8 => disabled_rssistop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => ready {
			0 => Disabled
			1 => Enabled
		}
		1 => address {
			0 => Disabled
			1 => Enabled
		}
		2 => payload {
			0 => Disabled
			1 => Enabled
		}
		3 => end {
			0 => Disabled
			1 => Enabled
		}
		4 => disabled {
			0 => Disabled
			1 => Enabled
		}
		5 => devmatch {
			0 => Disabled
			1 => Enabled
		}
		6 => devmiss {
			0 => Disabled
			1 => Enabled
		}
		7 => rssiend {
			0 => Disabled
			1 => Enabled
		}
		10 => bcmatch {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => ready {
			0 => Disabled
			1 => Enabled
		}
		1 => address {
			0 => Disabled
			1 => Enabled
		}
		2 => payload {
			0 => Disabled
			1 => Enabled
		}
		3 => end {
			0 => Disabled
			1 => Enabled
		}
		4 => disabled {
			0 => Disabled
			1 => Enabled
		}
		5 => devmatch {
			0 => Disabled
			1 => Enabled
		}
		6 => devmiss {
			0 => Disabled
			1 => Enabled
		}
		7 => rssiend {
			0 => Disabled
			1 => Enabled
		}
		10 => bcmatch {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 crcstatus {
		0 => crcstatus {
			0 => CRCError
			1 => CRCOk
		}
	}
	0x408 => reg32 rxmatch {
		0..2 => rxmatch,
	}
	0x40C => reg32 rxcrc {
		0..23 => rxcrc,
	}
	0x410 => reg32 dai {
		0..2 => dai,
	}
	0x504 => reg32 packetptr {}
	0x508 => reg32 frequency {
		0..6 => frequency,
	}
	0x50C => reg32 txpower {
		0..7 => txpower {
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
	0x510 => reg32 mode {
		0..1 => mode {
			0x00 => Nrf_1Mbit
			0x01 => Nrf_2Mbit
			0x02 => Nrf_250Kbit
			0x03 => Ble_1Mbit
		}
	}
	0x514 => reg32 pcnf0 {
		0..3 => lflen,
		8 => s0len,
		16..19 => s1len,
	}
	0x518 => reg32 pcnf1 {
		0..7 => maxlen,
		8..15 => statlen,
		16..18 => balen,
		24 => endian {
			0 => Little
			1 => Big
		}
		25 => whiteen {
			0 => Disabled
			1 => Enabled
		}
	}
	0x51C => reg32 base0 {}
	0x520 => reg32 base1 {}
	0x524 => reg32 prefix0 {
		0..7 => ap0,
		8..15 => ap1,
		16..23 => ap2,
		24..31 => ap3,
	}
	0x528 => reg32 prefix1 {
		0..7 => ap4,
		8..15 => ap5,
		16..23 => ap6,
		24..31 => ap7,
	}
	0x52C => reg32 txaddress {
		0..2 => txaddress,
	}
	0x530 => reg32 rxaddresses {
		0 => addr0 {
			0 => Disabled
			1 => Enabled
		}
		1 => addr1 {
			0 => Disabled
			1 => Enabled
		}
		2 => addr2 {
			0 => Disabled
			1 => Enabled
		}
		3 => addr3 {
			0 => Disabled
			1 => Enabled
		}
		4 => addr4 {
			0 => Disabled
			1 => Enabled
		}
		5 => addr5 {
			0 => Disabled
			1 => Enabled
		}
		6 => addr6 {
			0 => Disabled
			1 => Enabled
		}
		7 => addr7 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x534 => reg32 crccnf {
		0..1 => len {
			0 => Disabled
			1 => One
			2 => Two
			3 => Three
		}
		8 => skipaddr {
			0 => Include
			1 => Skip
		}
	}
	0x538 => reg32 crcpoly {
		0..23 => crcpoly,
	}
	0x53C => reg32 crcinit {
		0..23 => crcinit,
	}
	0x540 => reg32 test {
		0 => constcarrier {
			0 => Disabled
			1 => Enabled
		}
		1 => plllock {
			0 => Disabled
			1 => Enabled
		}
	}
	0x544 => reg32 tifs {
		0..7 => tifs,
	}
	0x548 => reg32 rssisample {
		0..6 => rssisample,
	}
	0x550 => reg32 state {
		0..3 => state {
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
	0x554 => reg32 datawhiteiv {
		0..6 => datawhiteiv,
	}
	0x560 => reg32 bcc {}
	0x600 => reg32 dab[%s] {}
	0x620 => reg32 dap[%s] {
		0..15 => dap,
	}
	0x640 => reg32 dacnf {
		0 => ena0 {
			0 => Disabled
			1 => Enabled
		}
		1 => ena1 {
			0 => Disabled
			1 => Enabled
		}
		2 => ena2 {
			0 => Disabled
			1 => Enabled
		}
		3 => ena3 {
			0 => Disabled
			1 => Enabled
		}
		4 => ena4 {
			0 => Disabled
			1 => Enabled
		}
		5 => ena5 {
			0 => Disabled
			1 => Enabled
		}
		6 => ena6 {
			0 => Disabled
			1 => Enabled
		}
		7 => ena7 {
			0 => Disabled
			1 => Enabled
		}
		8 => txadd0,
		9 => txadd1,
		10 => txadd2,
		11 => txadd3,
		12 => txadd4,
		13 => txadd5,
		14 => txadd6,
		15 => txadd7,
	}
	0x724 => reg32 override0 {
		0..31 => override0,
	}
	0x728 => reg32 override1 {
		0..31 => override1,
	}
	0x72C => reg32 override2 {
		0..31 => override2,
	}
	0x730 => reg32 override3 {
		0..31 => override3,
	}
	0x734 => reg32 override4 {
		0..27 => override4,
		31 => enable {
			0 => Disabled
			1 => Enabled
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (UART0 @ 0x40002000 = {
	0x000 => reg32 tasks_startrx {}
	0x004 => reg32 tasks_stoprx {}
	0x008 => reg32 tasks_starttx {}
	0x00C => reg32 tasks_stoptx {}
	0x01C => reg32 tasks_suspend {}
	0x100 => reg32 events_cts {}
	0x104 => reg32 events_ncts {}
	0x108 => reg32 events_rxdrdy {}
	0x11C => reg32 events_txdrdy {}
	0x124 => reg32 events_error {}
	0x144 => reg32 events_rxto {}
	0x200 => reg32 shorts {
		3 => cts_startrx {
			0 => Disabled
			1 => Enabled
		}
		4 => ncts_stoprx {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => cts {
			0 => Disabled
			1 => Enabled
		}
		1 => ncts {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdrdy {
			0 => Disabled
			1 => Enabled
		}
		7 => txdrdy {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		17 => rxto {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => cts {
			0 => Disabled
			1 => Enabled
		}
		1 => ncts {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdrdy {
			0 => Disabled
			1 => Enabled
		}
		7 => txdrdy {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		17 => rxto {
			0 => Disabled
			1 => Enabled
		}
	}
	0x480 => reg32 errorsrc {
		0 => overrun {
			0 => NotPresent
			1 => Present
		}
		1 => parity {
			0 => NotPresent
			1 => Present
		}
		2 => framing {
			0 => NotPresent
			1 => Present
		}
		3 => break {
			0 => NotPresent
			1 => Present
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x04 => Enabled
		}
	}
	0x508 => reg32 pselrts {}
	0x50C => reg32 pseltxd {}
	0x510 => reg32 pselcts {}
	0x514 => reg32 pselrxd {}
	0x518 => reg32 rxd {
		0..7 => rxd,
	}
	0x51C => reg32 txd {
		0..7 => txd,
	}
	0x524 => reg32 baudrate {
		0..31 => baudrate {
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
	0x56C => reg32 config {
		0 => hwfc {
			0 => Disabled
			1 => Enabled
		}
		1..3 => parity {
			0 => Excluded
			7 => Included
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (SPI0 @ 0x40003000 = {
	0x108 => reg32 events_ready {}
	0x304 => reg32 intenset {
		2 => ready {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		2 => ready {
			0 => Disabled
			1 => Enabled
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x01 => Enabled
		}
	}
	0x508 => reg32 pselsck {}
	0x50C => reg32 pselmosi {}
	0x510 => reg32 pselmiso {}
	0x518 => reg32 rxd {
		0..7 => rxd,
	}
	0x51C => reg32 txd {
		0..7 => txd,
	}
	0x524 => reg32 frequency {
		0..31 => frequency {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
		}
	}
	0x554 => reg32 config {
		0 => order {
			0 => MsbFirst
			1 => LsbFirst
		}
		1 => cpha {
			0 => Leading
			1 => Trailing
		}
		2 => cpol {
			0 => ActiveHigh
			1 => ActiveLow
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TWI0 @ 0x40003000 = {
	0x000 => reg32 tasks_startrx {}
	0x008 => reg32 tasks_starttx {}
	0x014 => reg32 tasks_stop {}
	0x01C => reg32 tasks_suspend {}
	0x020 => reg32 tasks_resume {}
	0x104 => reg32 events_stopped {}
	0x108 => reg32 events_rxdready {}
	0x11C => reg32 events_txdsent {}
	0x124 => reg32 events_error {}
	0x138 => reg32 events_bb {}
	0x148 => reg32 events_suspended {}
	0x200 => reg32 shorts {
		0 => bb_suspend {
			0 => Disabled
			1 => Enabled
		}
		1 => bb_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdready {
			0 => Disabled
			1 => Enabled
		}
		7 => txdsent {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		14 => bb {
			0 => Disabled
			1 => Enabled
		}
		18 => suspended {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdready {
			0 => Disabled
			1 => Enabled
		}
		7 => txdsent {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		14 => bb {
			0 => Disabled
			1 => Enabled
		}
		18 => suspended {
			0 => Disabled
			1 => Enabled
		}
	}
	0x4C4 => reg32 errorsrc {
		0 => overrun {
			0 => NotPresent
			1 => Present
		}
		1 => anack {
			0 => NotPresent
			1 => Present
		}
		2 => dnack {
			0 => NotPresent
			1 => Present
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x05 => Enabled
		}
	}
	0x508 => reg32 pselscl {}
	0x50C => reg32 pselsda {}
	0x518 => reg32 rxd {
		0..7 => rxd,
	}
	0x51C => reg32 txd {
		0..7 => txd,
	}
	0x524 => reg32 frequency {
		0..31 => frequency {
			0x01980000 => K100
			0x04000000 => K250
			0x06680000 => K400
		}
	}
	0x588 => reg32 address {
		0..6 => address,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (SPI1 @ 0x40004000 = {
	0x108 => reg32 events_ready {}
	0x304 => reg32 intenset {
		2 => ready {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		2 => ready {
			0 => Disabled
			1 => Enabled
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x01 => Enabled
		}
	}
	0x508 => reg32 pselsck {}
	0x50C => reg32 pselmosi {}
	0x510 => reg32 pselmiso {}
	0x518 => reg32 rxd {
		0..7 => rxd,
	}
	0x51C => reg32 txd {
		0..7 => txd,
	}
	0x524 => reg32 frequency {
		0..31 => frequency {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
		}
	}
	0x554 => reg32 config {
		0 => order {
			0 => MsbFirst
			1 => LsbFirst
		}
		1 => cpha {
			0 => Leading
			1 => Trailing
		}
		2 => cpol {
			0 => ActiveHigh
			1 => ActiveLow
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TWI1 @ 0x40004000 = {
	0x000 => reg32 tasks_startrx {}
	0x008 => reg32 tasks_starttx {}
	0x014 => reg32 tasks_stop {}
	0x01C => reg32 tasks_suspend {}
	0x020 => reg32 tasks_resume {}
	0x104 => reg32 events_stopped {}
	0x108 => reg32 events_rxdready {}
	0x11C => reg32 events_txdsent {}
	0x124 => reg32 events_error {}
	0x138 => reg32 events_bb {}
	0x148 => reg32 events_suspended {}
	0x200 => reg32 shorts {
		0 => bb_suspend {
			0 => Disabled
			1 => Enabled
		}
		1 => bb_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdready {
			0 => Disabled
			1 => Enabled
		}
		7 => txdsent {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		14 => bb {
			0 => Disabled
			1 => Enabled
		}
		18 => suspended {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		2 => rxdready {
			0 => Disabled
			1 => Enabled
		}
		7 => txdsent {
			0 => Disabled
			1 => Enabled
		}
		9 => error {
			0 => Disabled
			1 => Enabled
		}
		14 => bb {
			0 => Disabled
			1 => Enabled
		}
		18 => suspended {
			0 => Disabled
			1 => Enabled
		}
	}
	0x4C4 => reg32 errorsrc {
		0 => overrun {
			0 => NotPresent
			1 => Present
		}
		1 => anack {
			0 => NotPresent
			1 => Present
		}
		2 => dnack {
			0 => NotPresent
			1 => Present
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x05 => Enabled
		}
	}
	0x508 => reg32 pselscl {}
	0x50C => reg32 pselsda {}
	0x518 => reg32 rxd {
		0..7 => rxd,
	}
	0x51C => reg32 txd {
		0..7 => txd,
	}
	0x524 => reg32 frequency {
		0..31 => frequency {
			0x01980000 => K100
			0x04000000 => K250
			0x06680000 => K400
		}
	}
	0x588 => reg32 address {
		0..6 => address,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (SPIS1 @ 0x40004000 = {
	0x024 => reg32 tasks_acquire {}
	0x028 => reg32 tasks_release {}
	0x104 => reg32 events_end {}
	0x128 => reg32 events_acquired {}
	0x200 => reg32 shorts {
		2 => end_acquire {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		1 => end {
			0 => Disabled
			1 => Enabled
		}
		10 => acquired {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		1 => end {
			0 => Disabled
			1 => Enabled
		}
		10 => acquired {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 semstat {
		0..1 => semstat {
			0x00 => Free
			0x01 => CPU
			0x02 => SPIS
			0x03 => CPUPending
		}
	}
	0x440 => reg32 status {
		0 => overread {
			0 => NotPresent
			1 => Present
		}
		1 => overflow {
			0 => NotPresent
			1 => Present
		}
	}
	0x500 => reg32 enable {
		0..2 => enable {
			0x00 => Disabled
			0x02 => Enabled
		}
	}
	0x508 => reg32 pselsck {}
	0x50C => reg32 pselmiso {}
	0x510 => reg32 pselmosi {}
	0x514 => reg32 pselcsn {}
	0x534 => reg32 rxdptr {}
	0x538 => reg32 maxrx {
		0..7 => maxrx,
	}
	0x53C => reg32 amountrx {
		0..7 => amountrx,
	}
	0x544 => reg32 txdptr {}
	0x548 => reg32 maxtx {
		0..7 => maxtx,
	}
	0x54C => reg32 amounttx {
		0..7 => amounttx,
	}
	0x554 => reg32 config {
		0 => order {
			0 => MsbFirst
			1 => LsbFirst
		}
		1 => cpha {
			0 => Leading
			1 => Trailing
		}
		2 => cpol {
			0 => ActiveHigh
			1 => ActiveLow
		}
	}
	0x55C => reg32 def {
		0..7 => def,
	}
	0x5C0 => reg32 orc {
		0..7 => orc,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (SPIM1 @ 0x40004000 = {
	0x010 => reg32 tasks_start {}
	0x014 => reg32 tasks_stop {}
	0x01C => reg32 tasks_suspend {}
	0x020 => reg32 tasks_resume {}
	0x104 => reg32 events_stopped {}
	0x110 => reg32 events_endrx {}
	0x120 => reg32 events_endtx {}
	0x14C => reg32 events_started {}
	0x304 => reg32 intenset {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		4 => endrx {
			0 => Disabled
			1 => Enabled
		}
		8 => endtx {
			0 => Disabled
			1 => Enabled
		}
		19 => started {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		1 => stopped {
			0 => Disabled
			1 => Enabled
		}
		4 => endrx {
			0 => Disabled
			1 => Enabled
		}
		8 => endtx {
			0 => Disabled
			1 => Enabled
		}
		19 => started {
			0 => Disabled
			1 => Enabled
		}
	}
	0x500 => reg32 enable {
		0..3 => enable {
			0x00 => Disabled
			0x07 => Enabled
		}
	}
	0x524 => reg32 frequency {
		0..31 => frequency {
			0x02000000 => K125
			0x04000000 => K250
			0x08000000 => K500
			0x10000000 => M1
			0x20000000 => M2
			0x40000000 => M4
			0x80000000 => M8
		}
	}
	0x554 => reg32 config {
		0 => order {
			0 => MsbFirst
			1 => LsbFirst
		}
		1 => cpha {
			0 => Leading
			1 => Trailing
		}
		2 => cpol {
			0 => ActiveHigh
			1 => ActiveLow
		}
	}
	0x5C0 => reg32 orc {
		0..7 => orc,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (GPIOTE @ 0x40006000 = {
	0x000 => reg32 tasks_out[%s] {}
	0x100 => reg32 events_in[%s] {}
	0x17C => reg32 events_port {}
	0x304 => reg32 intenset {
		0 => in0 {
			0 => Disabled
			1 => Enabled
		}
		1 => in1 {
			0 => Disabled
			1 => Enabled
		}
		2 => in2 {
			0 => Disabled
			1 => Enabled
		}
		3 => in3 {
			0 => Disabled
			1 => Enabled
		}
		31 => port {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => in0 {
			0 => Disabled
			1 => Enabled
		}
		1 => in1 {
			0 => Disabled
			1 => Enabled
		}
		2 => in2 {
			0 => Disabled
			1 => Enabled
		}
		3 => in3 {
			0 => Disabled
			1 => Enabled
		}
		31 => port {
			0 => Disabled
			1 => Enabled
		}
	}
	0x510 => reg32 config[%s] {
		0..1 => mode {
			0x00 => Disabled
			0x01 => Event
			0x03 => Task
		}
		8..12 => psel,
		16..17 => polarity {
			0x00 => None
			0x01 => LoToHi
			0x02 => HiToLo
			0x03 => Toggle
		}
		20 => outinit {
			0 => Low
			1 => High
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (ADC @ 0x40007000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x100 => reg32 events_end {}
	0x304 => reg32 intenset {
		0 => end {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => end {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 busy {
		0 => busy {
			0 => Ready
			1 => Busy
		}
	}
	0x500 => reg32 enable {
		0..1 => enable {
			0x00 => Disabled
			0x01 => Enabled
		}
	}
	0x504 => reg32 config {
		0..1 => res {
			0x00 => 8bit
			0x01 => 9bit
			0x02 => 10bit
		}
		2..4 => inpsel {
			0x00 => AnalogInputNoPrescaling
			0x01 => AnalogInputTwoThirdsPrescaling
			0x02 => AnalogInputOneThirdPrescaling
			0x05 => SupplyTwoThirdsPrescaling
			0x06 => SupplyOneThirdPrescaling
		}
		5..6 => refsel {
			0x00 => VBG
			0x01 => External
			0x02 => SupplyOneHalfPrescaling
			0x03 => SupplyOneThirdPrescaling
		}
		8..15 => psel {
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
		16..17 => extrefsel {
			0 => None
			1 => AnalogReference0
			2 => AnalogReference1
		}
	}
	0x508 => reg32 result {
		0..9 => result,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TIMER0 @ 0x40008000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_count {}
	0x00C => reg32 tasks_clear {}
	0x010 => reg32 tasks_shutdown {}
	0x040 => reg32 tasks_capture[%s] {}
	0x140 => reg32 events_compare[%s] {}
	0x200 => reg32 shorts {
		0 => compare0_clear {
			0 => Disabled
			1 => Enabled
		}
		1 => compare1_clear {
			0 => Disabled
			1 => Enabled
		}
		2 => compare2_clear {
			0 => Disabled
			1 => Enabled
		}
		3 => compare3_clear {
			0 => Disabled
			1 => Enabled
		}
		8 => compare0_stop {
			0 => Disabled
			1 => Enabled
		}
		9 => compare1_stop {
			0 => Disabled
			1 => Enabled
		}
		10 => compare2_stop {
			0 => Disabled
			1 => Enabled
		}
		11 => compare3_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 mode {
		0 => mode {
			1 => Counter
			0 => Timer
		}
	}
	0x508 => reg32 bitmode {
		0..1 => bitmode {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
		}
	}
	0x510 => reg32 prescaler {
		0..3 => prescaler,
	}
	0x540 => reg32 cc[%s] {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TIMER1 @ 0x40009000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_count {}
	0x00C => reg32 tasks_clear {}
	0x010 => reg32 tasks_shutdown {}
	0x040 => reg32 tasks_capture[%s] {}
	0x140 => reg32 events_compare[%s] {}
	0x200 => reg32 shorts {
		0 => compare0_clear {
			0 => Disabled
			1 => Enabled
		}
		1 => compare1_clear {
			0 => Disabled
			1 => Enabled
		}
		2 => compare2_clear {
			0 => Disabled
			1 => Enabled
		}
		3 => compare3_clear {
			0 => Disabled
			1 => Enabled
		}
		8 => compare0_stop {
			0 => Disabled
			1 => Enabled
		}
		9 => compare1_stop {
			0 => Disabled
			1 => Enabled
		}
		10 => compare2_stop {
			0 => Disabled
			1 => Enabled
		}
		11 => compare3_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 mode {
		0 => mode {
			1 => Counter
			0 => Timer
		}
	}
	0x508 => reg32 bitmode {
		0..1 => bitmode {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
		}
	}
	0x510 => reg32 prescaler {
		0..3 => prescaler,
	}
	0x540 => reg32 cc[%s] {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TIMER2 @ 0x4000A000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_count {}
	0x00C => reg32 tasks_clear {}
	0x010 => reg32 tasks_shutdown {}
	0x040 => reg32 tasks_capture[%s] {}
	0x140 => reg32 events_compare[%s] {}
	0x200 => reg32 shorts {
		0 => compare0_clear {
			0 => Disabled
			1 => Enabled
		}
		1 => compare1_clear {
			0 => Disabled
			1 => Enabled
		}
		2 => compare2_clear {
			0 => Disabled
			1 => Enabled
		}
		3 => compare3_clear {
			0 => Disabled
			1 => Enabled
		}
		8 => compare0_stop {
			0 => Disabled
			1 => Enabled
		}
		9 => compare1_stop {
			0 => Disabled
			1 => Enabled
		}
		10 => compare2_stop {
			0 => Disabled
			1 => Enabled
		}
		11 => compare3_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 mode {
		0 => mode {
			1 => Counter
			0 => Timer
		}
	}
	0x508 => reg32 bitmode {
		0..1 => bitmode {
			0x00 => 16Bit
			0x01 => 08Bit
			0x02 => 24Bit
			0x03 => 32Bit
		}
	}
	0x510 => reg32 prescaler {
		0..3 => prescaler,
	}
	0x540 => reg32 cc[%s] {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (RTC0 @ 0x4000B000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_clear {}
	0x00C => reg32 tasks_trigovrflw {}
	0x100 => reg32 events_tick {}
	0x104 => reg32 events_ovrflw {}
	0x140 => reg32 events_compare[%s] {}
	0x304 => reg32 intenset {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x340 => reg32 evten {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x344 => reg32 evtenset {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x348 => reg32 evtenclr {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 counter {
		0..23 => counter,
	}
	0x508 => reg32 prescaler {
		0..11 => prescaler,
	}
	0x540 => reg32 cc[%s] {
		0..23 => compare,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (TEMP @ 0x4000C000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x100 => reg32 events_datardy {}
	0x304 => reg32 intenset {
		0 => datardy {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => datardy {
			0 => Disabled
			1 => Enabled
		}
	}
	0x508 => reg32 temp {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (RNG @ 0x4000D000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x100 => reg32 events_valrdy {}
	0x200 => reg32 shorts {
		0 => valrdy_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => valrdy {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => valrdy {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 config {
		0 => dercen {
			0 => Disabled
			1 => Enabled
		}
	}
	0x508 => reg32 value {
		0..7 => value,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (ECB @ 0x4000E000 = {
	0x000 => reg32 tasks_startecb {}
	0x004 => reg32 tasks_stopecb {}
	0x100 => reg32 events_endecb {}
	0x104 => reg32 events_errorecb {}
	0x304 => reg32 intenset {
		0 => endecb {
			0 => Disabled
			1 => Enabled
		}
		1 => errorecb {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => endecb {
			0 => Disabled
			1 => Enabled
		}
		1 => errorecb {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 ecbdataptr {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (AAR @ 0x4000F000 = {
	0x000 => reg32 tasks_start {}
	0x008 => reg32 tasks_stop {}
	0x100 => reg32 events_end {}
	0x104 => reg32 events_resolved {}
	0x108 => reg32 events_notresolved {}
	0x304 => reg32 intenset {
		0 => end {
			0 => Disabled
			1 => Enabled
		}
		1 => resolved {
			0 => Disabled
			1 => Enabled
		}
		2 => notresolved {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => end {
			0 => Disabled
			1 => Enabled
		}
		1 => resolved {
			0 => Disabled
			1 => Enabled
		}
		2 => notresolved {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 status {
		0..3 => status,
	}
	0x500 => reg32 enable {
		0..1 => enable {
			0x00 => Disabled
			0x03 => Enabled
		}
	}
	0x504 => reg32 nirk {
		0..4 => nirk,
	}
	0x508 => reg32 irkptr {}
	0x510 => reg32 addrptr {}
	0x514 => reg32 scratchptr {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (CCM @ 0x4000F000 = {
	0x000 => reg32 tasks_ksgen {}
	0x004 => reg32 tasks_crypt {}
	0x008 => reg32 tasks_stop {}
	0x100 => reg32 events_endksgen {}
	0x104 => reg32 events_endcrypt {}
	0x108 => reg32 events_error {}
	0x200 => reg32 shorts {
		0 => endksgen_crypt {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => endksgen {
			0 => Disabled
			1 => Enabled
		}
		1 => endcrypt {
			0 => Disabled
			1 => Enabled
		}
		2 => error {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => endksgen {
			0 => Disabled
			1 => Enabled
		}
		1 => endcrypt {
			0 => Disabled
			1 => Enabled
		}
		2 => error {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 micstatus {
		0 => micstatus {
			0 => CheckFailed
			1 => CheckPassed
		}
	}
	0x500 => reg32 enable {
		0..1 => enable {
			0x00 => Disabled
			0x02 => Enabled
		}
	}
	0x504 => reg32 mode {
		0 => mode {
			0 => Encryption
			1 => Decryption
		}
	}
	0x508 => reg32 cnfptr {}
	0x50C => reg32 inptr {}
	0x510 => reg32 outptr {}
	0x514 => reg32 scratchptr {}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (WDT @ 0x40010000 = {
	0x000 => reg32 tasks_start {}
	0x100 => reg32 events_timeout {}
	0x304 => reg32 intenset {
		0 => timeout {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => timeout {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 runstatus {
		0 => runstatus {
			0 => NotRunning
			1 => Running
		}
	}
	0x404 => reg32 reqstatus {
		0 => rr0 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		1 => rr1 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		2 => rr2 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		3 => rr3 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		4 => rr4 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		5 => rr5 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		6 => rr6 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
		7 => rr7 {
			0 => DisabledOrRequested
			1 => EnabledAndUnrequested
		}
	}
	0x504 => reg32 crv {}
	0x508 => reg32 rren {
		0 => rr0 {
			0 => Disabled
			1 => Enabled
		}
		1 => rr1 {
			0 => Disabled
			1 => Enabled
		}
		2 => rr2 {
			0 => Disabled
			1 => Enabled
		}
		3 => rr3 {
			0 => Disabled
			1 => Enabled
		}
		4 => rr4 {
			0 => Disabled
			1 => Enabled
		}
		5 => rr5 {
			0 => Disabled
			1 => Enabled
		}
		6 => rr6 {
			0 => Disabled
			1 => Enabled
		}
		7 => rr7 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x50C => reg32 config {
		0 => sleep {
			0 => Pause
			1 => Run
		}
		3 => halt {
			0 => Pause
			1 => Run
		}
	}
	0x600 => reg32 rr[%s] {
		0..31 => rr {
			0x6E524635 => Reload
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (RTC1 @ 0x40011000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_clear {}
	0x00C => reg32 tasks_trigovrflw {}
	0x100 => reg32 events_tick {}
	0x104 => reg32 events_ovrflw {}
	0x140 => reg32 events_compare[%s] {}
	0x304 => reg32 intenset {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x340 => reg32 evten {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x344 => reg32 evtenset {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x348 => reg32 evtenclr {
		0 => tick {
			0 => Disabled
			1 => Enabled
		}
		1 => ovrflw {
			0 => Disabled
			1 => Enabled
		}
		16 => compare0 {
			0 => Disabled
			1 => Enabled
		}
		17 => compare1 {
			0 => Disabled
			1 => Enabled
		}
		18 => compare2 {
			0 => Disabled
			1 => Enabled
		}
		19 => compare3 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 counter {
		0..23 => counter,
	}
	0x508 => reg32 prescaler {
		0..11 => prescaler,
	}
	0x540 => reg32 cc[%s] {
		0..23 => compare,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (QDEC @ 0x40012000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_readclracc {}
	0x100 => reg32 events_samplerdy {}
	0x104 => reg32 events_reportrdy {}
	0x108 => reg32 events_accof {}
	0x200 => reg32 shorts {
		0 => reportrdy_readclracc {
			0 => Disabled
			1 => Enabled
		}
		1 => samplerdy_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => samplerdy {
			0 => Disabled
			1 => Enabled
		}
		1 => reportrdy {
			0 => Disabled
			1 => Enabled
		}
		2 => accof {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => samplerdy {
			0 => Disabled
			1 => Enabled
		}
		1 => reportrdy {
			0 => Disabled
			1 => Enabled
		}
		2 => accof {
			0 => Disabled
			1 => Enabled
		}
	}
	0x500 => reg32 enable {
		0 => enable {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 ledpol {
		0 => ledpol {
			0 => ActiveLow
			1 => ActiveHigh
		}
	}
	0x508 => reg32 sampleper {
		0..2 => sampleper {
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
	0x50C => reg32 sample {
		0..31 => sample,
	}
	0x510 => reg32 reportper {
		0..2 => reportper {
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
	0x514 => reg32 acc {}
	0x518 => reg32 accread {}
	0x51C => reg32 pselled {}
	0x520 => reg32 psela {}
	0x524 => reg32 pselb {}
	0x528 => reg32 dbfen {
		0 => dbfen {
			0 => Disabled
			1 => Enabled
		}
	}
	0x540 => reg32 ledpre {
		0..8 => ledpre,
	}
	0x544 => reg32 accdbl {
		0..3 => accdbl,
	}
	0x548 => reg32 accdblread {
		0..3 => accdblread,
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (LPCOMP @ 0x40013000 = {
	0x000 => reg32 tasks_start {}
	0x004 => reg32 tasks_stop {}
	0x008 => reg32 tasks_sample {}
	0x100 => reg32 events_ready {}
	0x104 => reg32 events_down {}
	0x108 => reg32 events_up {}
	0x10C => reg32 events_cross {}
	0x200 => reg32 shorts {
		0 => ready_sample {
			0 => Disabled
			1 => Enabled
		}
		1 => ready_stop {
			0 => Disabled
			1 => Enabled
		}
		2 => down_stop {
			0 => Disabled
			1 => Enabled
		}
		3 => up_stop {
			0 => Disabled
			1 => Enabled
		}
		4 => cross_stop {
			0 => Disabled
			1 => Enabled
		}
	}
	0x304 => reg32 intenset {
		0 => ready {
			0 => Disabled
			1 => Enabled
		}
		1 => down {
			0 => Disabled
			1 => Enabled
		}
		2 => up {
			0 => Disabled
			1 => Enabled
		}
		3 => cross {
			0 => Disabled
			1 => Enabled
		}
	}
	0x308 => reg32 intenclr {
		0 => ready {
			0 => Disabled
			1 => Enabled
		}
		1 => down {
			0 => Disabled
			1 => Enabled
		}
		2 => up {
			0 => Disabled
			1 => Enabled
		}
		3 => cross {
			0 => Disabled
			1 => Enabled
		}
	}
	0x400 => reg32 result {
		0 => result {
			0 => Bellow
			1 => Above
		}
	}
	0x500 => reg32 enable {
		0..1 => enable {
			0x00 => Disabled
			0x01 => Enabled
		}
	}
	0x504 => reg32 psel {
		0..2 => psel {
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
	0x508 => reg32 refsel {
		0..2 => refsel {
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
	0x50C => reg32 extrefsel {
		0 => extrefsel {
			0 => AnalogReference0
			1 => AnalogReference1
		}
	}
	0x520 => reg32 anadetect {
		0..1 => anadetect {
			0 => Cross
			1 => Up
			2 => Down
		}
	}
	0xFFC => reg32 power {
		0 => power {
			0 => Disabled
			1 => Enabled
		}
	}
});

ioregs! (SWI @ 0x40014000 = {
	0x000 => reg32 unused {}
});

ioregs! (NVMC @ 0x4001E000 = {
	0x400 => reg32 ready {
		0 => ready {
			0 => Busy
			1 => Ready
		}
	}
	0x504 => reg32 config {
		0..1 => wen {
			0x00 => Ren
			0x01 => Wen
			0x02 => Een
		}
	}
	0x508 => reg32 erasepage {}
	0x508 => reg32 erasepcr1 {}
	0x50C => reg32 eraseall {
		0 => eraseall {
			0 => NoOperation
			1 => Erase
		}
	}
	0x510 => reg32 erasepcr0 {}
	0x514 => reg32 eraseuicr {
		0 => eraseuicr {
			0 => NoOperation
			1 => Erase
		}
	}
});

ioregs! (PPI @ 0x4001F000 = {
	0x500 => reg32 chen {
		0 => ch0 {
			0 => Disabled
			1 => Enabled
		}
		1 => ch1 {
			0 => Disabled
			1 => Enabled
		}
		2 => ch2 {
			0 => Disabled
			1 => Enabled
		}
		3 => ch3 {
			0 => Disabled
			1 => Enabled
		}
		4 => ch4 {
			0 => Disabled
			1 => Enabled
		}
		5 => ch5 {
			0 => Disabled
			1 => Enabled
		}
		6 => ch6 {
			0 => Disabled
			1 => Enabled
		}
		7 => ch7 {
			0 => Disabled
			1 => Enabled
		}
		8 => ch8 {
			0 => Disabled
			1 => Enabled
		}
		9 => ch9 {
			0 => Disabled
			1 => Enabled
		}
		10 => ch10 {
			0 => Disabled
			1 => Enabled
		}
		11 => ch11 {
			0 => Disabled
			1 => Enabled
		}
		12 => ch12 {
			0 => Disabled
			1 => Enabled
		}
		13 => ch13 {
			0 => Disabled
			1 => Enabled
		}
		14 => ch14 {
			0 => Disabled
			1 => Enabled
		}
		15 => ch15 {
			0 => Disabled
			1 => Enabled
		}
		20 => ch20 {
			0 => Disabled
			1 => Enabled
		}
		21 => ch21 {
			0 => Disabled
			1 => Enabled
		}
		22 => ch22 {
			0 => Disabled
			1 => Enabled
		}
		23 => ch23 {
			0 => Disabled
			1 => Enabled
		}
		24 => ch24 {
			0 => Disabled
			1 => Enabled
		}
		25 => ch25 {
			0 => Disabled
			1 => Enabled
		}
		26 => ch26 {
			0 => Disabled
			1 => Enabled
		}
		27 => ch27 {
			0 => Disabled
			1 => Enabled
		}
		28 => ch28 {
			0 => Disabled
			1 => Enabled
		}
		29 => ch29 {
			0 => Disabled
			1 => Enabled
		}
		30 => ch30 {
			0 => Disabled
			1 => Enabled
		}
		31 => ch31 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x504 => reg32 chenset {
		0 => ch0 {
			0 => Disabled
			1 => Enabled
		}
		1 => ch1 {
			0 => Disabled
			1 => Enabled
		}
		2 => ch2 {
			0 => Disabled
			1 => Enabled
		}
		3 => ch3 {
			0 => Disabled
			1 => Enabled
		}
		4 => ch4 {
			0 => Disabled
			1 => Enabled
		}
		5 => ch5 {
			0 => Disabled
			1 => Enabled
		}
		6 => ch6 {
			0 => Disabled
			1 => Enabled
		}
		7 => ch7 {
			0 => Disabled
			1 => Enabled
		}
		8 => ch8 {
			0 => Disabled
			1 => Enabled
		}
		9 => ch9 {
			0 => Disabled
			1 => Enabled
		}
		10 => ch10 {
			0 => Disabled
			1 => Enabled
		}
		11 => ch11 {
			0 => Disabled
			1 => Enabled
		}
		12 => ch12 {
			0 => Disabled
			1 => Enabled
		}
		13 => ch13 {
			0 => Disabled
			1 => Enabled
		}
		14 => ch14 {
			0 => Disabled
			1 => Enabled
		}
		15 => ch15 {
			0 => Disabled
			1 => Enabled
		}
		20 => ch20 {
			0 => Disabled
			1 => Enabled
		}
		21 => ch21 {
			0 => Disabled
			1 => Enabled
		}
		22 => ch22 {
			0 => Disabled
			1 => Enabled
		}
		23 => ch23 {
			0 => Disabled
			1 => Enabled
		}
		24 => ch24 {
			0 => Disabled
			1 => Enabled
		}
		25 => ch25 {
			0 => Disabled
			1 => Enabled
		}
		26 => ch26 {
			0 => Disabled
			1 => Enabled
		}
		27 => ch27 {
			0 => Disabled
			1 => Enabled
		}
		28 => ch28 {
			0 => Disabled
			1 => Enabled
		}
		29 => ch29 {
			0 => Disabled
			1 => Enabled
		}
		30 => ch30 {
			0 => Disabled
			1 => Enabled
		}
		31 => ch31 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x508 => reg32 chenclr {
		0 => ch0 {
			0 => Disabled
			1 => Enabled
		}
		1 => ch1 {
			0 => Disabled
			1 => Enabled
		}
		2 => ch2 {
			0 => Disabled
			1 => Enabled
		}
		3 => ch3 {
			0 => Disabled
			1 => Enabled
		}
		4 => ch4 {
			0 => Disabled
			1 => Enabled
		}
		5 => ch5 {
			0 => Disabled
			1 => Enabled
		}
		6 => ch6 {
			0 => Disabled
			1 => Enabled
		}
		7 => ch7 {
			0 => Disabled
			1 => Enabled
		}
		8 => ch8 {
			0 => Disabled
			1 => Enabled
		}
		9 => ch9 {
			0 => Disabled
			1 => Enabled
		}
		10 => ch10 {
			0 => Disabled
			1 => Enabled
		}
		11 => ch11 {
			0 => Disabled
			1 => Enabled
		}
		12 => ch12 {
			0 => Disabled
			1 => Enabled
		}
		13 => ch13 {
			0 => Disabled
			1 => Enabled
		}
		14 => ch14 {
			0 => Disabled
			1 => Enabled
		}
		15 => ch15 {
			0 => Disabled
			1 => Enabled
		}
		20 => ch20 {
			0 => Disabled
			1 => Enabled
		}
		21 => ch21 {
			0 => Disabled
			1 => Enabled
		}
		22 => ch22 {
			0 => Disabled
			1 => Enabled
		}
		23 => ch23 {
			0 => Disabled
			1 => Enabled
		}
		24 => ch24 {
			0 => Disabled
			1 => Enabled
		}
		25 => ch25 {
			0 => Disabled
			1 => Enabled
		}
		26 => ch26 {
			0 => Disabled
			1 => Enabled
		}
		27 => ch27 {
			0 => Disabled
			1 => Enabled
		}
		28 => ch28 {
			0 => Disabled
			1 => Enabled
		}
		29 => ch29 {
			0 => Disabled
			1 => Enabled
		}
		30 => ch30 {
			0 => Disabled
			1 => Enabled
		}
		31 => ch31 {
			0 => Disabled
			1 => Enabled
		}
	}
	0x800 => reg32 chg[%s] {
		0 => ch0 {
			0 => Excluded
			1 => Included
		}
		1 => ch1 {
			0 => Excluded
			1 => Included
		}
		2 => ch2 {
			0 => Excluded
			1 => Included
		}
		3 => ch3 {
			0 => Excluded
			1 => Included
		}
		4 => ch4 {
			0 => Excluded
			1 => Included
		}
		5 => ch5 {
			0 => Excluded
			1 => Included
		}
		6 => ch6 {
			0 => Excluded
			1 => Included
		}
		7 => ch7 {
			0 => Excluded
			1 => Included
		}
		8 => ch8 {
			0 => Excluded
			1 => Included
		}
		9 => ch9 {
			0 => Excluded
			1 => Included
		}
		10 => ch10 {
			0 => Excluded
			1 => Included
		}
		11 => ch11 {
			0 => Excluded
			1 => Included
		}
		12 => ch12 {
			0 => Excluded
			1 => Included
		}
		13 => ch13 {
			0 => Excluded
			1 => Included
		}
		14 => ch14 {
			0 => Excluded
			1 => Included
		}
		15 => ch15 {
			0 => Excluded
			1 => Included
		}
		20 => ch20 {
			0 => Excluded
			1 => Included
		}
		21 => ch21 {
			0 => Excluded
			1 => Included
		}
		22 => ch22 {
			0 => Excluded
			1 => Included
		}
		23 => ch23 {
			0 => Excluded
			1 => Included
		}
		24 => ch24 {
			0 => Excluded
			1 => Included
		}
		25 => ch25 {
			0 => Excluded
			1 => Included
		}
		26 => ch26 {
			0 => Excluded
			1 => Included
		}
		27 => ch27 {
			0 => Excluded
			1 => Included
		}
		28 => ch28 {
			0 => Excluded
			1 => Included
		}
		29 => ch29 {
			0 => Excluded
			1 => Included
		}
		30 => ch30 {
			0 => Excluded
			1 => Included
		}
		31 => ch31 {
			0 => Excluded
			1 => Included
		}
	}
});

ioregs! (FICR @ 0x10000000 = {
	0x010 => reg32 codepagesize {}
	0x014 => reg32 codesize {}
	0x028 => reg32 clenr0 {}
	0x02C => reg32 ppfc {
		0..7 => ppfc {
			0xFF => NotPresent
			0x00 => Present
		}
	}
	0x034 => reg32 numramblock {}
	0x038 => reg32 sizeramblocks {}
	0x038 => reg32 sizeramblock[%s] {}
	0x05C => reg32 configid {
		0..15 => hwid,
		16..31 => fwid,
	}
	0x060 => reg32 deviceid[%s] {}
	0x080 => reg32 er[%s] {}
	0x090 => reg32 ir[%s] {}
	0x0A0 => reg32 deviceaddrtype {
		0 => deviceaddrtype {
			0 => Public
			1 => Random
		}
	}
	0x0A4 => reg32 deviceaddr[%s] {}
	0x0AC => reg32 overrideen {
		0 => nrf_1mbit {
			0 => Override
			1 => NotOverride
		}
		3 => ble_1mbit {
			0 => Override
			1 => NotOverride
		}
	}
	0x0B0 => reg32 nrf_1mbit[%s] {}
	0x0EC => reg32 ble_1mbit[%s] {}
});

ioregs! (UICR @ 0x10001000 = {
	0x000 => reg32 clenr0 {}
	0x004 => reg32 rbpconf {
		0..7 => pr0 {
			0xFF => Disabled
			0x00 => Enabled
		}
		8..15 => pall {
			0xFF => Disabled
			0x00 => Enabled
		}
	}
	0x008 => reg32 xtalfreq {
		0..7 => xtalfreq {
			0xFF => 16MHz
			0x00 => 32MHz
		}
	}
	0x010 => reg32 fwid {
		0..15 => fwid,
	}
	0x014 => reg32 bootloaderaddr {}
	0x014 => reg32 nrffw[%s] {}
	0x050 => reg32 nrfhw[%s] {}
	0x080 => reg32 customer[%s] {}
});

ioregs! (GPIO @ 0x50000000 = {
	0x504 => reg32 out {
		0 => pin0 {
			0 => Low
			1 => High
		}
		1 => pin1 {
			0 => Low
			1 => High
		}
		2 => pin2 {
			0 => Low
			1 => High
		}
		3 => pin3 {
			0 => Low
			1 => High
		}
		4 => pin4 {
			0 => Low
			1 => High
		}
		5 => pin5 {
			0 => Low
			1 => High
		}
		6 => pin6 {
			0 => Low
			1 => High
		}
		7 => pin7 {
			0 => Low
			1 => High
		}
		8 => pin8 {
			0 => Low
			1 => High
		}
		9 => pin9 {
			0 => Low
			1 => High
		}
		10 => pin10 {
			0 => Low
			1 => High
		}
		11 => pin11 {
			0 => Low
			1 => High
		}
		12 => pin12 {
			0 => Low
			1 => High
		}
		13 => pin13 {
			0 => Low
			1 => High
		}
		14 => pin14 {
			0 => Low
			1 => High
		}
		15 => pin15 {
			0 => Low
			1 => High
		}
		16 => pin16 {
			0 => Low
			1 => High
		}
		17 => pin17 {
			0 => Low
			1 => High
		}
		18 => pin18 {
			0 => Low
			1 => High
		}
		19 => pin19 {
			0 => Low
			1 => High
		}
		20 => pin20 {
			0 => Low
			1 => High
		}
		21 => pin21 {
			0 => Low
			1 => High
		}
		22 => pin22 {
			0 => Low
			1 => High
		}
		23 => pin23 {
			0 => Low
			1 => High
		}
		24 => pin24 {
			0 => Low
			1 => High
		}
		25 => pin25 {
			0 => Low
			1 => High
		}
		26 => pin26 {
			0 => Low
			1 => High
		}
		27 => pin27 {
			0 => Low
			1 => High
		}
		28 => pin28 {
			0 => Low
			1 => High
		}
		29 => pin29 {
			0 => Low
			1 => High
		}
		30 => pin30 {
			0 => Low
			1 => High
		}
		31 => pin31 {
			0 => Low
			1 => High
		}
	}
	0x508 => reg32 outset {
		0 => pin0 {
			0 => Low
			1 => High
		}
		1 => pin1 {
			0 => Low
			1 => High
		}
		2 => pin2 {
			0 => Low
			1 => High
		}
		3 => pin3 {
			0 => Low
			1 => High
		}
		4 => pin4 {
			0 => Low
			1 => High
		}
		5 => pin5 {
			0 => Low
			1 => High
		}
		6 => pin6 {
			0 => Low
			1 => High
		}
		7 => pin7 {
			0 => Low
			1 => High
		}
		8 => pin8 {
			0 => Low
			1 => High
		}
		9 => pin9 {
			0 => Low
			1 => High
		}
		10 => pin10 {
			0 => Low
			1 => High
		}
		11 => pin11 {
			0 => Low
			1 => High
		}
		12 => pin12 {
			0 => Low
			1 => High
		}
		13 => pin13 {
			0 => Low
			1 => High
		}
		14 => pin14 {
			0 => Low
			1 => High
		}
		15 => pin15 {
			0 => Low
			1 => High
		}
		16 => pin16 {
			0 => Low
			1 => High
		}
		17 => pin17 {
			0 => Low
			1 => High
		}
		18 => pin18 {
			0 => Low
			1 => High
		}
		19 => pin19 {
			0 => Low
			1 => High
		}
		20 => pin20 {
			0 => Low
			1 => High
		}
		21 => pin21 {
			0 => Low
			1 => High
		}
		22 => pin22 {
			0 => Low
			1 => High
		}
		23 => pin23 {
			0 => Low
			1 => High
		}
		24 => pin24 {
			0 => Low
			1 => High
		}
		25 => pin25 {
			0 => Low
			1 => High
		}
		26 => pin26 {
			0 => Low
			1 => High
		}
		27 => pin27 {
			0 => Low
			1 => High
		}
		28 => pin28 {
			0 => Low
			1 => High
		}
		29 => pin29 {
			0 => Low
			1 => High
		}
		30 => pin30 {
			0 => Low
			1 => High
		}
		31 => pin31 {
			0 => Low
			1 => High
		}
	}
	0x50C => reg32 outclr {
		0 => pin0 {
			0 => Low
			1 => High
		}
		1 => pin1 {
			0 => Low
			1 => High
		}
		2 => pin2 {
			0 => Low
			1 => High
		}
		3 => pin3 {
			0 => Low
			1 => High
		}
		4 => pin4 {
			0 => Low
			1 => High
		}
		5 => pin5 {
			0 => Low
			1 => High
		}
		6 => pin6 {
			0 => Low
			1 => High
		}
		7 => pin7 {
			0 => Low
			1 => High
		}
		8 => pin8 {
			0 => Low
			1 => High
		}
		9 => pin9 {
			0 => Low
			1 => High
		}
		10 => pin10 {
			0 => Low
			1 => High
		}
		11 => pin11 {
			0 => Low
			1 => High
		}
		12 => pin12 {
			0 => Low
			1 => High
		}
		13 => pin13 {
			0 => Low
			1 => High
		}
		14 => pin14 {
			0 => Low
			1 => High
		}
		15 => pin15 {
			0 => Low
			1 => High
		}
		16 => pin16 {
			0 => Low
			1 => High
		}
		17 => pin17 {
			0 => Low
			1 => High
		}
		18 => pin18 {
			0 => Low
			1 => High
		}
		19 => pin19 {
			0 => Low
			1 => High
		}
		20 => pin20 {
			0 => Low
			1 => High
		}
		21 => pin21 {
			0 => Low
			1 => High
		}
		22 => pin22 {
			0 => Low
			1 => High
		}
		23 => pin23 {
			0 => Low
			1 => High
		}
		24 => pin24 {
			0 => Low
			1 => High
		}
		25 => pin25 {
			0 => Low
			1 => High
		}
		26 => pin26 {
			0 => Low
			1 => High
		}
		27 => pin27 {
			0 => Low
			1 => High
		}
		28 => pin28 {
			0 => Low
			1 => High
		}
		29 => pin29 {
			0 => Low
			1 => High
		}
		30 => pin30 {
			0 => Low
			1 => High
		}
		31 => pin31 {
			0 => Low
			1 => High
		}
	}
	0x510 => reg32 in {
		0 => pin0 {
			0 => Low
			1 => High
		}
		1 => pin1 {
			0 => Low
			1 => High
		}
		2 => pin2 {
			0 => Low
			1 => High
		}
		3 => pin3 {
			0 => Low
			1 => High
		}
		4 => pin4 {
			0 => Low
			1 => High
		}
		5 => pin5 {
			0 => Low
			1 => High
		}
		6 => pin6 {
			0 => Low
			1 => High
		}
		7 => pin7 {
			0 => Low
			1 => High
		}
		8 => pin8 {
			0 => Low
			1 => High
		}
		9 => pin9 {
			0 => Low
			1 => High
		}
		10 => pin10 {
			0 => Low
			1 => High
		}
		11 => pin11 {
			0 => Low
			1 => High
		}
		12 => pin12 {
			0 => Low
			1 => High
		}
		13 => pin13 {
			0 => Low
			1 => High
		}
		14 => pin14 {
			0 => Low
			1 => High
		}
		15 => pin15 {
			0 => Low
			1 => High
		}
		16 => pin16 {
			0 => Low
			1 => High
		}
		17 => pin17 {
			0 => Low
			1 => High
		}
		18 => pin18 {
			0 => Low
			1 => High
		}
		19 => pin19 {
			0 => Low
			1 => High
		}
		20 => pin20 {
			0 => Low
			1 => High
		}
		21 => pin21 {
			0 => Low
			1 => High
		}
		22 => pin22 {
			0 => Low
			1 => High
		}
		23 => pin23 {
			0 => Low
			1 => High
		}
		24 => pin24 {
			0 => Low
			1 => High
		}
		25 => pin25 {
			0 => Low
			1 => High
		}
		26 => pin26 {
			0 => Low
			1 => High
		}
		27 => pin27 {
			0 => Low
			1 => High
		}
		28 => pin28 {
			0 => Low
			1 => High
		}
		29 => pin29 {
			0 => Low
			1 => High
		}
		30 => pin30 {
			0 => Low
			1 => High
		}
		31 => pin31 {
			0 => Low
			1 => High
		}
	}
	0x514 => reg32 dir {
		0 => pin0 {
			0 => Input
			1 => Output
		}
		1 => pin1 {
			0 => Input
			1 => Output
		}
		2 => pin2 {
			0 => Input
			1 => Output
		}
		3 => pin3 {
			0 => Input
			1 => Output
		}
		4 => pin4 {
			0 => Input
			1 => Output
		}
		5 => pin5 {
			0 => Input
			1 => Output
		}
		6 => pin6 {
			0 => Input
			1 => Output
		}
		7 => pin7 {
			0 => Input
			1 => Output
		}
		8 => pin8 {
			0 => Input
			1 => Output
		}
		9 => pin9 {
			0 => Input
			1 => Output
		}
		10 => pin10 {
			0 => Input
			1 => Output
		}
		11 => pin11 {
			0 => Input
			1 => Output
		}
		12 => pin12 {
			0 => Input
			1 => Output
		}
		13 => pin13 {
			0 => Input
			1 => Output
		}
		14 => pin14 {
			0 => Input
			1 => Output
		}
		15 => pin15 {
			0 => Input
			1 => Output
		}
		16 => pin16 {
			0 => Input
			1 => Output
		}
		17 => pin17 {
			0 => Input
			1 => Output
		}
		18 => pin18 {
			0 => Input
			1 => Output
		}
		19 => pin19 {
			0 => Input
			1 => Output
		}
		20 => pin20 {
			0 => Input
			1 => Output
		}
		21 => pin21 {
			0 => Input
			1 => Output
		}
		22 => pin22 {
			0 => Input
			1 => Output
		}
		23 => pin23 {
			0 => Input
			1 => Output
		}
		24 => pin24 {
			0 => Input
			1 => Output
		}
		25 => pin25 {
			0 => Input
			1 => Output
		}
		26 => pin26 {
			0 => Input
			1 => Output
		}
		27 => pin27 {
			0 => Input
			1 => Output
		}
		28 => pin28 {
			0 => Input
			1 => Output
		}
		29 => pin29 {
			0 => Input
			1 => Output
		}
		30 => pin30 {
			0 => Input
			1 => Output
		}
		31 => pin31 {
			0 => Input
			1 => Output
		}
	}
	0x518 => reg32 dirset {
		0 => pin0 {
			0 => Input
			1 => Output
		}
		1 => pin1 {
			0 => Input
			1 => Output
		}
		2 => pin2 {
			0 => Input
			1 => Output
		}
		3 => pin3 {
			0 => Input
			1 => Output
		}
		4 => pin4 {
			0 => Input
			1 => Output
		}
		5 => pin5 {
			0 => Input
			1 => Output
		}
		6 => pin6 {
			0 => Input
			1 => Output
		}
		7 => pin7 {
			0 => Input
			1 => Output
		}
		8 => pin8 {
			0 => Input
			1 => Output
		}
		9 => pin9 {
			0 => Input
			1 => Output
		}
		10 => pin10 {
			0 => Input
			1 => Output
		}
		11 => pin11 {
			0 => Input
			1 => Output
		}
		12 => pin12 {
			0 => Input
			1 => Output
		}
		13 => pin13 {
			0 => Input
			1 => Output
		}
		14 => pin14 {
			0 => Input
			1 => Output
		}
		15 => pin15 {
			0 => Input
			1 => Output
		}
		16 => pin16 {
			0 => Input
			1 => Output
		}
		17 => pin17 {
			0 => Input
			1 => Output
		}
		18 => pin18 {
			0 => Input
			1 => Output
		}
		19 => pin19 {
			0 => Input
			1 => Output
		}
		20 => pin20 {
			0 => Input
			1 => Output
		}
		21 => pin21 {
			0 => Input
			1 => Output
		}
		22 => pin22 {
			0 => Input
			1 => Output
		}
		23 => pin23 {
			0 => Input
			1 => Output
		}
		24 => pin24 {
			0 => Input
			1 => Output
		}
		25 => pin25 {
			0 => Input
			1 => Output
		}
		26 => pin26 {
			0 => Input
			1 => Output
		}
		27 => pin27 {
			0 => Input
			1 => Output
		}
		28 => pin28 {
			0 => Input
			1 => Output
		}
		29 => pin29 {
			0 => Input
			1 => Output
		}
		30 => pin30 {
			0 => Input
			1 => Output
		}
		31 => pin31 {
			0 => Input
			1 => Output
		}
	}
	0x51C => reg32 dirclr {
		0 => pin0 {
			0 => Input
			1 => Output
		}
		1 => pin1 {
			0 => Input
			1 => Output
		}
		2 => pin2 {
			0 => Input
			1 => Output
		}
		3 => pin3 {
			0 => Input
			1 => Output
		}
		4 => pin4 {
			0 => Input
			1 => Output
		}
		5 => pin5 {
			0 => Input
			1 => Output
		}
		6 => pin6 {
			0 => Input
			1 => Output
		}
		7 => pin7 {
			0 => Input
			1 => Output
		}
		8 => pin8 {
			0 => Input
			1 => Output
		}
		9 => pin9 {
			0 => Input
			1 => Output
		}
		10 => pin10 {
			0 => Input
			1 => Output
		}
		11 => pin11 {
			0 => Input
			1 => Output
		}
		12 => pin12 {
			0 => Input
			1 => Output
		}
		13 => pin13 {
			0 => Input
			1 => Output
		}
		14 => pin14 {
			0 => Input
			1 => Output
		}
		15 => pin15 {
			0 => Input
			1 => Output
		}
		16 => pin16 {
			0 => Input
			1 => Output
		}
		17 => pin17 {
			0 => Input
			1 => Output
		}
		18 => pin18 {
			0 => Input
			1 => Output
		}
		19 => pin19 {
			0 => Input
			1 => Output
		}
		20 => pin20 {
			0 => Input
			1 => Output
		}
		21 => pin21 {
			0 => Input
			1 => Output
		}
		22 => pin22 {
			0 => Input
			1 => Output
		}
		23 => pin23 {
			0 => Input
			1 => Output
		}
		24 => pin24 {
			0 => Input
			1 => Output
		}
		25 => pin25 {
			0 => Input
			1 => Output
		}
		26 => pin26 {
			0 => Input
			1 => Output
		}
		27 => pin27 {
			0 => Input
			1 => Output
		}
		28 => pin28 {
			0 => Input
			1 => Output
		}
		29 => pin29 {
			0 => Input
			1 => Output
		}
		30 => pin30 {
			0 => Input
			1 => Output
		}
		31 => pin31 {
			0 => Input
			1 => Output
		}
	}
	0x700 => reg32 pin_cnf[%s] {
		0 => dir {
			0 => Input
			1 => Output
		}
		1 => input {
			0 => Connect
			1 => Disconnect
		}
		2..3 => pull {
			0x00 => Disabled
			0x01 => Pulldown
			0x03 => Pullup
		}
		8..10 => drive {
			0x00 => S0S1
			0x01 => H0S1
			0x02 => S0H1
			0x03 => H0H1
			0x04 => D0S1
			0x05 => D0H1
			0x06 => S0D1
			0x07 => H0D1
		}
		16..17 => sense {
			0x00 => Disabled
			0x02 => High
			0x03 => Low
		}
	}
});
