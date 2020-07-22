# What?
`bitinfo` is a tool to make it easier work with registers by decoding their values.

Let's start with an example of how the tool works. Say you're debugging a new sensor and want to see if a register holds the expected value. You read it and get a cryptic `0x4E0`. This can be decoded by hand, but that's where `bitinfo` comes in. How's this:
```
Kerrys-MacBook:bitinfo borgel$ bitinfo STPM.DSP_CR3.0x4E0
STPM.DSP_CR3 0x4E0 ->
	SAG_TIME_THR =	1248 (The sag time)
	ZCR_SEL =	V1 / 7.8125 kHz (Selection bit for ZCR/CLK pin, (output depends on ZCR/CLK enable bit))
	ZCR_EN =	CLK
	TMP_TOL =	12.5% (Tamper tolerance)
	TMP_EN =	tamper disable (Tamper detect enable)
	S/W Reset =	0x0 (Software reset)
	S/W Latch 1 =	0x0
	S/W Latch 2 =	0x0
	S/W Auto Latch =	Disabled (Auto latch)
	LED_OFF1 =	LED output on (LED pin output disable)
	LED_OFF2 =	LED output on (LED pin output disable)
	EN_CUM =	Cumulative is the sum of channel energies (Cumulative energy calculaton)
	REF_FRQ =	50 Hz (Reference line frequency)
```
`bitinfo` lets the developer specify configuration files describing their registers (in this case `STPM.DSP_CR3`) then decode arbitary numeric values based on those files. The corresponding file for this example can be found [here](examples/stpm.bitinfo.yaml).

# Examples
`bitinfo` will help you describe a number, even if it doesn't know how to decode it as a specific register value.
```bash
Kerrys-MacBook:bitinfo borgel$ bitinfo 0xa5
"165" -> 165 0xA5 0b10100101
Kerrys-MacBook:bitinfo borgel$
```
It can also describe the i'th bits set.
```
Kerrys-MacBook:bitinfo borgel$ bitinfo --bits 0xa5
"165" -> 165 0xA5 0b10100101
0th set
2th set
5th set
7th set
```

# Get It
Coming Soon®. Will be on http://crates.io to start.

# Known Limitations
Many.
* Though there is no reason it needs to be limited to 32 bit registers, there are a places where this is assumed
* Only available via a fully working Rust install (but on any platform!)
* Limited output format options (specifically, no options)

