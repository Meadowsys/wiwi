# Spec

The Spec&trade;

## Endianness

All values that are sensitive to endianness, will use the little endian byte order when encoded.

## Markers

- `0x00` - the value `false`
- `0x01` - the value `true`
- `0x02` - the value `none` (similar to `null`, `nil`, `Option::None`, etc)
- `0x03` to `0x07` - IEEE754-2008 floating point types (`f16`, `f32`, `f64`, `f128`, `f256`)
- `0x08` - string
- `0x09` - array
- `0x0a` - record
- `0x0b` - map
- `0x0c` - binary
- `0x0d` - interned value
- `0x0e` - 2 byte markers
- `0x0f` - 3 byte markers
- `0x10` to `0x1f` - integer markers (`i8`, `i16`, `i24`, `i32`, `i48`, `i64`, `i96`, `i128`, `u8`, `u16`, `u24`, `u32`, `u48`, `u64`, `u96`, `u128`)
- `0x20` to `0x3f` - negative integers, from -31 to 0
- `0x40` to `0x7f` - positive integers, from 1 to 64
- `0x80` to `0x9f` - string, length 1 to 32
- `0xa0` to `0xaf` - array, length 1 to 16
- `0xb0` to `0xb7` - record, length 1 to 8
- `0xb8` to `0xbf` - map, length 1 to 8
- `0xc0` to `0xff` - reference to interned value

## `none`/`null`/`nil`/`None`/etc

The value `none` is encoded as `0x02`.

## booleans

The value `false` is encoded as `0x00`, and the value `true` is encoded as `0x01`.

## integers

There are... 18 types of integers: small positive ints, small negative ints, `i8`, `i16`, `i24`, `i32`, `i48`, `i64`, `i96`, `i128`, `u8`, `u16`, `u24`, `u32`, `u48`, `u64`, `u96`, `u128`.

Marker types for `i8`...`i128` are in range `0x10` to `0x17`, and marker types for `u8`...`u128` are in range `0x18` to `0x1f`. For example, marker for i96 is `0x16`, and marker for `u24` is `0x1a`.

If an integer is within -31 to 0, it can be encoded as one marker byte within the range `0x20` to `0x3f`. For example, `0x20` has the value of -31 and `0x3f` has the value of 0.

If an integer is within 1 to 64, it can be encoded as one marker byte within range `0x40` to `0x7f`. For example, `0x40` has the value of 1 and `0x7f` has the value of 64.

If an integer is outside of these ranges: first consider what signedness the integer is, and what the _value_ is. The smallest type that has the correct signedness must be used. For example, an unsigned value of 70,000 should use the type `u24`, while a signed value of 70,000 should use the type `i24`. Then first write the marker for the type, followed by the integer bytes itself.

Note: in v1 there were more number types (one for ever multiple of 8, until 128). We've decided there is probably not much benefit to these extra types for the amount of marker space they hog. With this new data model there are half the amount of integer sizes, half the int markers as before, but still a decent amount more flexibility than just every power of 2 from 8 to 128.

## floats

Floating point values are IEEE754-2008 values (our `f32` and `f64` are identical to Rust's).

`f16` values are encoded by first writing the byte `0x03`, followed by the bytes of the float value itself (2 bytes).

`f32` values are encoded by first writing the byte `0x04`, followed by the bytes of the float value itself (4 bytes).

`f64` values are encoded by first writing the byte `0x05`, followed by the bytes of the float value itself (8 bytes).

`f128` values are encoded by first writing the byte `0x06`, followed by the bytes of the float value itself (16 bytes).

`f256` values are encoded by first writing the byte `0x07`, followed by the bytes of the float value itself (32 bytes).

`f16`, `f128`, and `f256` types are included in the spec, but should not actually be used for now, as support for them is very rare.

## strings

Strings are strictly UTF-8 strings, like Rust's `String`/`str` types.

If the string has a byte length within the range `1..=32` (note, _does not_ include length zero bytes), use a marker within the range from `0x80` to `0x9f` to encode its length. For example, `0x80` means string with length 1, `0x81` length 2, `0x82` length 3, ... `0x9f` length 32.

If the string is either 0 bytes long (ie. empty string), or is length 33 bytes or longer, first encode the marker byte `0x08`, followed by an unsigned variable length integer encoding the byte length of the string.

After writing the marker byte(s), write the string in verbatim.

## arrays

Arrays are like a list of serialised values. There are specialised array types available, but this one is the most "generic" array type, able to encode lists of anything that can be encoded at all.

If the array has amount of elements within `1..=16` (note: _does not_ include length zero), a marker within range `0xa0` to `0xaf` is used. For example, `0xa0` encodes an array with length 1, and `0xaf` encodes an array with length 16.

If the array has either zero elements, or 17 or more items, first encode the marker byte `0x09`, follwed by an unsigned variable length integer encoding the length of the array.

Next, encode every element in the array, one element at a time, including everything.

## record

Records are key value mappings, from a string to any encodable value. One key-value pair is one entry.

Use marker bytes `0xb0` to `0xb7` to encode a record with an amount of entries within `1..=8` (note, _does not_ include 0 entries).

If the record has either 0 entries or 9 or more entries, first encode the record marker `0x0a`, followed by an unsigned variable length integer encoding the amount of entries.

Then, encode every key value pair in one at a time. First the key, then the value.

Since the key in this type must be a string, we can just always encode its length using an unsigned variable length integer, skipping the string marker. For strings with length 1 to 32, this has no effect on encoded size, but for strings with length 0 or 33 or more, doing this would save 1 byte per string.

## binary

A binary blob of just, binary data.

We made the design decision that any binary data sent through this encoding method will usually exceed the amount of "inlined" markers we give it for encoding its length (which would have been 16, or maybe 8, something quite small for binary data), and felt those marker values would serve better as part of some other type's range.

First encode the binary marker `0x0c`, followed by an unsigned variable length integer encoding the length in bytes.

## map

A map is like a record (key value mappings), except it accepts arbitrary types for the keys as well as strings.

Use marker bytes `0xb8` to `0xbf` to encode a map with a number of entries within `1..=8` (note, _does not_ include 0 entries).

If the map has either 0 entries, or 9 or more entries, first encode the map marker `0x0b`, then an unsigned variable length integer which encodes the amount of entries.

Then, encode every key value pair in one at a time, first the key, then the value.

Note: since we accept arbitrary types for the keys, a marker must be used for the keys here, unlike records.

## interned values and references

todo

## variable length integers

these can be used to encode up to a 16 byte (128 bit) integer

There are two variants to this encoding: unsigned only and signed only. Depending on context, one or the other are used. For example, in a context where a length is expected, the unsigned encoding would be used.

### unsigned variable length integer

- `0x00` to `0xf7` - encodes 0 to 247
- `0xf8` - next byte encodes u8
- `0xf9` - next 2 bytes encodes u16
- `0xfa` - next 3 bytes encodes u24
- `0xfb` - next 4 bytes encodes u32
- `0xfc` - next 6 bytes encodes u48
- `0xfd` - next 8 bytes encodes u64
- `0xfe` - next 12 bytes encodes u96
- `0xff` - next 16 bytes encodes u128

### signed variable length integer

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

# ???

Now that I've written the first version... I've figured out a few more things that I think I could do better.
