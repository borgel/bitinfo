# `bitinfo`

[![CircleCI](https://circleci.com/gh/borgel/bitinfo.svg?style=shield)](https://github.com/borgel/bitinfo)
[![Version info](https://img.shields.io/crates/v/bitinfo.svg)](https://crates.io/crates/bitinfo)

`bitinfo` is a tool to make it easier work with registers by decoding their values.

## What?
Let's start with an example of how the tool works. Say you're debugging a new sensor and want to see if a register holds the expected value. You read it and get a cryptic `0x4E0`. This can be decoded by hand, but that's where `bitinfo` comes in. Isn't this better than decoding by hand?
```
borgel$ bitinfo STPM.DSP_CR3.0x4E0
 STPM.DSP_CR3 0x4E0 -> |                                           |
-----------------------+-------------------------------------------+-----------------------------------------------------------------------
 SAG_TIME_THR          | 1248                                      | The sag time
 ZCR_SEL               | V1 / 7.8125 kHz                           | Selection bit for ZCR/CLK pin, (output depends on ZCR/CLK enable bit)
 ZCR_EN                | CLK                                       |
 TMP_TOL               | 12.5%                                     | Tamper tolerance
 TMP_EN                | tamper disable                            | Tamper detect enable
 S/W Reset             | 0x0                                       | Software reset
 S/W Latch 1           | 0x0                                       |
 S/W Latch 2           | 0x0                                       |
 S/W Auto Latch        | Disabled                                  | Auto latch
 LED_OFF1              | LED output on                             | LED pin output disable
 LED_OFF2              | LED output on                             | LED pin output disable
 EN_CUM                | Cumulative is the sum of channel energies | Cumulative energy calculaton
 REF_FRQ               | 50 Hz                                     | Reference line frequency
```
`bitinfo` lets the developer specify configuration files describing a hierarchy of device registers (in this case `STPM.DSP_CR3`) then decode arbitary numeric values based on those descriptions.

## Get It
For now available on all platforms with a Rust toolchain at [http://crates.io](https://crates.io/crates/bitinfo). Just
```bash
cargo install --force bitinfo
# the `--force` forces `cargo` to install the latest version if you have an earlier one
```

## Use It
Using `bitinfo` requires two areas of knowledge; how to invoke it and how to configure register descriptions.

### Invocation
Once installed, `bitinfo` is a single binary which may be invoked from a shell. `bitinfo` will help
you describe a number, even if it doesn't know how to decode it as a specific register value.
```
borgel$ bitinfo 0xa5
165 -> 165 0xA5 0b10100101
borgel$ bitinfo 0x0F0
 240 | 240 0xF0 0b11110000
```
Any common numerical format works fine (0x hex, 0b binary, and 0 decimal).

It can also describe the i'th bits set.
```
borgel$ bitinfo --bits 0xa5
 165 | 165 0xA5 0b10100101
-----+---------------------
     | 0th set
     | 2th set
     | 5th set
     | 7th set
```

But the tool is much more useful when describing values for which it has descriptive configuration files.
If working in a directory tree which includes `bitinfo` configuration files, queries may be made specifying
a certain device.
```
borgel$ bitinfo R2-Device.Register1.0x1A
 R2-Device.Register1 0x1A -> |
-----------------------------+--------------------
 A nibble of decimals        | 10
 A bamboozler to control     | Bamboozler enabled
```

In order to decode a specific register `bitinfo` must
* Have the appropriate configuration file visible along its search path
* Be invoked with a named reference to the register to decode with, which must exist in an appropriately named, visible, config file

Keep in mind
* Separators must be any combination of `.:/`
* Device and register names are case sensitive and must exactly match the configuration file

### Configuration
`bitinfo` uses configuration files to describe registers and fields which it can decode. To see
a heavily annotated example configuration which describes all allowable fields
check [this sample in the examples directory](examples/.bitinfo.yaml).

Configuration files must be named following the pattern `.bitinfo.<optional name>.yaml` to be detected
by the tool. The optional name may be handy for organisational purposes to allow a project to clearly
describe multiple configuration files in one place but may be omitted. It is not current used during decoding.

`bitinfo` searches for configuration files in two ways: checking every directory upward including the current one, and by
specifying a directory on the commandline. Each execution `bitinfo` will search these
paths and load every config file matching the name pattern. It searches all configurations together
in order to resolve a user query, so a large project can spread register descriptions across many files.

The expected use case is for a project to include configurations
either at the root of the project (so they are visible everywhere below) or to include configurations
specific to certain modules near those modules' source.

## Examples
An unusually simple configuration may look something like this:
```yaml
R1-Device:
   description: This is the top level of organization. Think part number like LIS2DH or STM32F104
   preferred_format: hex
   fields:
      BField1:
         start: 0
         width: 1
         description: I'm not sure what a bamboozler is, are you?
         # sometimes it can be handy to describe what the flag states mean
         patterns:
            0b0: Bamboozler disabled
            0b1: Bamboozler enabled
```
To a query with this device would look like this
```
# bamboozler disabled
borgel$ bitinfo --configs ../examples/ R1-Device.0
 R1-Device 0x1A -> |                     |
-------------------+---------------------+---------------------------------------------
 BField1           | Bamboozler disabled | I'm not sure what a bamboozler is, are you?

# and with the bamboozler enabled
borgel$ bitinfo --configs ../examples/ R1-Device.1
 R1-Device 0x0A -> |                    |
-------------------+--------------------+---------------------------------------------
 BField1           | Bamboozler enabled | I'm not sure what a bamboozler is, are you?
```

This is somewhat simplified. The most common configurations include at least one additional layer of
organization and look something more like this:
```yaml
R2-Device:
   description: This is the top level of organization. Often an entire device (CPU, complex sensor, etc)
   registers:
      Register1:
         description: A second level of organization. Think of a peripheral or register in a CPU or sensor
         fields:
            A decimal slice:
               start: 0
               width: 4
               preferred_format: dec
            BField2:
               start: 4
               width: 1
               patterns:
                  0b0: Bamboozler disabled
                  0b1: Bamboozler enabled
```
Note that `registers` here can be nested to arbitrary depth.

To a query with the above device, the command would look like this
```
borgel$ bitinfo --configs ../examples/ R2-Device.Register1.0x1A
 R2-Device.Register1 0x1A -> |
-----------------------------+--------------------
 A nibble of decimals        | 10
 A bamboozler to control     | Bamboozler enabled
```

Please note the above examples were run relative to the source directory in the repository, which is why
the examples directory is passed in as an explicit search path.

See the [examples](examples/) directory for detailed explanations of the available fields and their
usage.

## Known Limitations
* Though there is no reason it needs to be limited to 32 bit registers, there are a places where this is assumed
* Only available via a fully working Rust install (but on any platform!)
* Limited output format options (specifically, no options)
* Probably more. PRs welcome!

