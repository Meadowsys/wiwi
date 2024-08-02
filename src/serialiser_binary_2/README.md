## Spec

### Endianness

All values that are sensitive to endianness, will use the little endian byte order when encoded.

### Markers

- `0x00` - the value `false`
- `0x01` - the value `true`
- `0x02` - the value `none` (similar to `null`, `nil`, `Option::None`, etc)
- `0x03` to `0x07` - IEEE754-2008 floating point types (`f16`, `f32`, `f64`, `f128`, `f256`)
- `0x08` - string
- `0x09` - array
- `0x0a` - record
- `0x0b` - binary
- `0x0c` - map
- `0x0d` - interned value
- `0x0e` - 2 byte markers
- `0x0f` - 3 byte markers
- `0x10` to `0x1f` - integer markers (`i8`, `i16`, `i24`, `i32`, `i48`, `i64`, `i96`, `i128`, `u8`, `u16`, `u24`, `u32`, `u48`, `u64`, `u96`, `u128`)
- `0x20` to `0x3f` - negative integers, from -31 to 0
- `0x40` to `0x7f` - positive integers, from 1 to 64
- `0x80` to `0x9f` - string, length 1 to 32
- `0xa0` to `0xaf` - array, length 1 to 16
- `0xb0` to `0xbf` - record, length 1 to 16
- `0xc0` to `0xff` - reference to interned value

### variable length integers

these can be used to encode up to a 16 byte (128 bit) integer

There are two variants to this encoding: unsigned only and signed only. Depending on context, one or the other are used. For example, in a context where a length is expected, the unsigned encoding would be used.

#### unsigned only

- `0x00` to `0xf7` - encodes 0 to 247
- `0xf8` - next byte encodes u8
- `0xf9` - next 2 bytes encodes u16
- `0xfa` - next 3 bytes encodes u24
- `0xfb` - next 4 bytes encodes u32
- `0xfc` - next 6 bytes encodes u48
- `0xfd` - next 8 bytes encodes u64
- `0xfe` - next 12 bytes encodes u96
- `0xff` - next 16 bytes encodes u128

#### signed only

- `0x84` to `0xff` - encodes -124 to -1 (two's compliment)
- `0x00` to `0x7b` - encodes 0 to 123
- `0x7c` - next byte encodes i8
- `0x7d` - next 2 bytes encodes i16
- `0x7e` - next 3 bytes encodes i24
- `0x7f` - next 4 bytes encodes i32
- `0x80` - next 6 bytes encodes i48
- `0x81` - next 8 bytes encodes i64
- `0x82` - next 12 bytes encodes i96
- `0x83` - next 16 bytes encodes i128

## ???

Now that I've written the first version... I've figured out a few more things that I think I could do better.
