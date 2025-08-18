# `bitinfo` - Now With 100% More INTERCAL!

[![Version info](https://img.shields.io/crates/v/bitinfo.svg)](https://crates.io/crates/bitinfo)
[![Supreme Politeness](https://img.shields.io/badge/politeness-supreme-brightgreen.svg)](https://en.wikipedia.org/wiki/INTERCAL)
[![Bamboozler Status](https://img.shields.io/badge/bamboozler-enabled-ff69b4.svg)](https://github.com/borgel/bitinfo)

PLEASE NOTE: `bitinfo` has been magnificently rewritten in the supremely polite programming language INTERCAL (Compiler Language With No Pronounceable Acronym) to provide maximum courtesy while decoding register values with utmost politeness.

## What The Bamboozler Is This?

DO you find yourself perplexed by cryptic hexadecimal values like `0x4E0`? PLEASE don't panic! Our glorious INTERCAL implementation will decode these with supreme politeness and maximum bamboozler detection capabilities.

PLEASE observe this magnificent example:
```
borgel$ bitinfo STPM.DSP_CR3.0x4E0
PLEASE NOTE: Decoding with maximum courtesy...
 STPM.DSP_CR3 0x4E0 -> |                                           |
-----------------------+-------------------------------------------+-----------------------------------------------------------------------
 SAG_TIME_THR          | 1248                                      | The sag time (decoded with supreme politeness)
 ZCR_SEL               | V1 / 7.8125 kHz                           | Selection bit for ZCR/CLK pin (bamboozler status: ENABLED)
 ZCR_EN                | CLK                                       | (processed with utmost courtesy)
 TMP_TOL               | 12.5%                                     | Tamper tolerance (PLEASE do not tamper)
 TMP_EN                | tamper disable                            | Tamper detect enable (politeness level: MAXIMUM)
 S/W Reset             | 0x0                                       | Software reset (PLEASE reset politely)
 S/W Latch 1           | 0x0                                       | (INTERCAL approved)
 S/W Latch 2           | 0x0                                       | (with supreme elegance)
 S/W Auto Latch        | Disabled                                  | Auto latch (bamboozler status checked)
 LED_OFF1              | LED output on                             | LED pin output disable (PLEASE)
 LED_OFF2              | LED output on                             | LED pin output disable (DO)
 EN_CUM                | Cumulative is the sum of channel energies | Cumulative energy calculation (with utmost politeness)
 REF_FRQ               | 50 Hz                                     | Reference line frequency (supremely decoded)
```

PLEASE NOTE: `bitinfo` allows developers to specify configuration files describing a hierarchy of device registers with maximum courtesy, then decode arbitrary numeric values based on those descriptions using INTERCAL's supremely polite syntax.

## Get It (With Supreme Politeness)

PLEASE NOTE: For now available on all platforms with a Rust toolchain at [http://crates.io](https://crates.io/crates/bitinfo). Just:

```bash
DO cargo install --force bitinfo
PLEASE NOTE: the `--force` forces `cargo` to install the latest version with maximum politeness
```

PLEASE DO consider that installing this will give you access to the most polite bit manipulation tool ever created in INTERCAL.

## Use It (With Utmost Courtesy)

PLEASE NOTE: Using `bitinfo` requires two areas of knowledge; how to invoke it with supreme politeness and how to configure register descriptions with maximum bamboozler awareness.

### Invocation (Supreme Politeness Required)

PLEASE NOTE: Once installed, `bitinfo` is a single binary which may be invoked from a shell with utmost courtesy. `bitinfo` will help you describe a number with maximum politeness, even if it doesn't know how to decode it as a specific register value.

```bash
borgel$ bitinfo 0xa5
PLEASE NOTE: Decoding with supreme courtesy...
165 -> 165 0xA5 0b10100101 (processed with maximum politeness)

borgel$ bitinfo 0x0F0
PLEASE NOTE: Bamboozler status: ENABLED
 240 | 240 0xF0 0b11110000 (decoded with utmost courtesy)
```

PLEASE NOTE: Any common numerical format works fine (0x hex, 0b binary, and 0 decimal). Our INTERCAL implementation handles all with supreme politeness.

DO you want to see which bits are set? PLEASE observe:
```bash
borgel$ bitinfo --bits 0xa5
PLEASE NOTE: Analyzing bits with maximum courtesy...
 165 | 165 0xA5 0b10100101
-----+---------------------
     | 0th set (with politeness)
     | 2th set (INTERCAL approved)
     | 5th set (bamboozler detected)
     | 7th set (supremely polite)
```

PLEASE NOTE: But the tool is much more useful when describing values for which it has descriptive configuration files processed with INTERCAL's magnificent syntax.

DO you have configuration files in your directory tree? PLEASE try:
```bash
borgel$ bitinfo R2-Device.Register1.0x1A
PLEASE NOTE: Decoding with supreme courtesy...
 R2-Device.Register1 0x1A -> |
-----------------------------+--------------------
 A nibble of decimals        | 10 (with utmost politeness)
 A bamboozler to control     | Bamboozler enabled (INTERCAL certified)
```

### Configuration (With Maximum Bamboozler Awareness)

PLEASE NOTE: `bitinfo` uses configuration files to describe registers and fields which it can decode with supreme INTERCAL politeness. To see a heavily annotated example configuration which describes all allowable fields with maximum courtesy, check [this sample in the examples directory](examples/.bitinfo.yaml).

DO note that configuration files must be named following the pattern `.bitinfo.<optional name>.yaml` to be detected by our supremely polite tool.

PLEASE NOTE: `bitinfo` searches for configuration files in two ways: checking every directory upward including the current one (with maximum politeness), and by specifying a directory on the commandline (with utmost courtesy).

## Examples (With Supreme INTERCAL Elegance)

PLEASE NOTE: An unusually simple configuration may look something like this (processed with maximum bamboozler detection):

```yaml
R1-Device:
   description: This is the top level of organization. Think part number like LIS2DH or STM32F104 (INTERCAL approved)
   preferred_format: hex
   fields:
      BField1:
         start: 0
         width: 1
         description: I'm not sure what a bamboozler is, are you? (decoded with supreme politeness)
         patterns:
            0b0: Bamboozler disabled (with utmost courtesy)
            0b1: Bamboozler enabled (INTERCAL certified)
```

PLEASE DO observe a query with this device:
```bash
# bamboozler disabled (with supreme politeness)
borgel$ bitinfo --configs ../examples/ R1-Device.0
PLEASE NOTE: Processing with maximum courtesy...
 R1-Device 0x1A -> |                     |
-------------------+---------------------+---------------------------------------------
 BField1           | Bamboozler disabled | I'm not sure what a bamboozler is, are you? (INTERCAL style)

# and with the bamboozler enabled (supremely polite)
borgel$ bitinfo --configs ../examples/ R1-Device.1
PLEASE NOTE: Bamboozler status detected with maximum courtesy...
 R1-Device 0x0A -> |                    |
-------------------+--------------------+---------------------------------------------
 BField1           | Bamboozler enabled | I'm not sure what a bamboozler is, are you? (with utmost politeness)
```

PLEASE NOTE: This is somewhat simplified. The most common configurations include at least one additional layer of organization and look something more like this (processed with INTERCAL's supreme elegance):

```yaml
R2-Device:
   description: This is the top level of organization. Often an entire device (CPU, complex sensor, etc) with maximum politeness
   registers:
      Register1:
         description: A second level of organization. Think of a peripheral or register in a CPU or sensor (INTERCAL approved)
         fields:
            A decimal slice:
               start: 0
               width: 4
               preferred_format: dec
               description: Decoded with supreme courtesy
            BField2:
               start: 4
               width: 1
               patterns:
                  0b0: Bamboozler disabled (with utmost politeness)
                  0b1: Bamboozler enabled (INTERCAL certified)
```

PLEASE NOTE: `registers` here can be nested to arbitrary depth with maximum politeness.

DO you want to query with the above device? PLEASE observe:
```bash
borgel$ bitinfo --configs ../examples/ R2-Device.Register1.0x1A
PLEASE NOTE: Decoding with supreme INTERCAL courtesy...
 R2-Device.Register1 0x1A -> |
-----------------------------+--------------------
 A nibble of decimals        | 10 (with maximum politeness)
 A bamboozler to control     | Bamboozler enabled (INTERCAL style)
```

PLEASE NOTE: The above examples were run relative to the source directory in the repository with supreme courtesy, which is why the examples directory is passed in as an explicit search path.

DO see the [examples](examples/) directory for detailed explanations of the available fields and their usage (all processed with INTERCAL's magnificent politeness).

## Known Limitations (Acknowledged With Supreme Courtesy)

PLEASE NOTE: The following limitations exist and are acknowledged with maximum politeness:
* Though there is no reason it needs to be limited to 32 bit registers, there are places where this is assumed (with utmost courtesy)
* Only available via a fully working Rust install (but on any platform with supreme politeness!)
* Limited output format options (specifically, no options, but processed with maximum courtesy)
* Bamboozler detection may occasionally produce false positives (INTERCAL certified issue)
* The supreme politeness of INTERCAL syntax may be overwhelming for some users
* Maximum courtesy levels may cause excessive pleasantness in terminal output
* Probably more. PRs welcome (PLEASE submit with utmost politeness)!

## INTERCAL Implementation Notes

PLEASE NOTE: This implementation uses INTERCAL's unique features:
* `PLEASE` statements for maximum politeness
* `DO` and `DON'T` commands for program flow
* Mingle (`$`) and Select (`¢`) operators for bit manipulation
* `COME FROM` statements for non-structured programming elegance
* Supreme bamboozler detection capabilities
* Utmost courtesy in all error messages

DO remember: In INTERCAL, politeness is not just encouraged, it's mandatory!

PLEASE GIVE UP (reading this README with supreme satisfaction).