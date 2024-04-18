# wiwi serialiser spec <!-- omit from toc -->

**NOTE**: This spec is still a work in progress and _SHOULD NOT_ be relied on for _anything_ intended to be used in production. Feel free to toy around with it and suggest things, but this should be considered highly unstable until at least its pushed to crates.io.

Note to self: needs more diagrams. Diagrams are nice.

## About wiwi serialiser

This does not have a formal name any more than "wiwi serialiser":p. Wiwi serialiser is a binary format serialisation similar to JSON.

## Table of contents <!-- omit from toc -->

- [About wiwi serialiser](#about-wiwi-serialiser)
- [Spec](#spec)
  - [General structure](#general-structure)
  - [Markers](#markers)
  - [Integers](#integers)
  - [Floats](#floats)
  - [Booleans](#booleans)
  - [Arrays (heterogenous)](#arrays-heterogenous)
  - [Arrays (homogenous)](#arrays-homogenous)
  - [Array of booleans](#array-of-booleans)
    - [Example: 0 booleans](#example-0-booleans)
    - [Example: 2 booleans](#example-2-booleans)
    - [Example: 4 booleans](#example-4-booleans)
    - [Example: 5 booleans](#example-5-booleans)
    - [Example: 12 booleans](#example-12-booleans)
    - [Example: 17 booleans](#example-17-booleans)
    - [Misc other examples](#misc-other-examples)
    - [Why on earth?????](#why-on-earth)

## Spec

### General structure

Tne general structure of an item serialised is a marker byte that is unique to that type, followed by the data in a specific format following it. The length of the data can always be determined, either by a property of the type (for example, a i32 will always take 4 bytes of space), or specified somewhere towards the beginning where it can be deterministically found before the variable length section (for example, storing the length before an array).

### Markers

| Marker | Type
| -------|----------------------------------
| 0      | None
| 1      | _unassigned_
| 2      | **8-bit** integer, unsigned
| 3      | **8-bit** integer, signed (two's compliment)
| 4      | **16-bit** integer, unsigned
| 5      | **16-bit** integer, signed (two's compliment)
| 6      | **24-bit** integer, unsigned
| 7      | **24-bit** integer, signed (two's compliment)
| 8      | **32-bit** integer, unsigned
| 9      | **32-bit** integer, signed (two's compliment)
| 10     | **40-bit** integer, unsigned
| 11     | **40-bit** integer, signed (two's compliment)
| 12     | **48-bit** integer, unsigned
| 13     | **48-bit** integer, signed (two's compliment)
| 14     | **56-bit** integer, unsigned
| 15     | **56-bit** integer, signed (two's compliment)
| 16     | **64-bit** integer, unsigned
| 17     | **64-bit** integer, signed (two's compliment)
| 18     | **72-bit** integer, unsigned
| 19     | **72-bit** integer, signed (two's compliment)
| 20     | **80-bit** integer, unsigned
| 21     | **80-bit** integer, signed (two's compliment)
| 22     | **88-bit** integer, unsigned
| 23     | **88-bit** integer, signed (two's compliment)
| 24     | **96-bit** integer, unsigned
| 25     | **96-bit** integer, signed (two's compliment)
| 26     | **104-bit** integer, unsigned
| 27     | **104-bit** integer, signed (two's compliment)
| 28     | **112-bit** integer, unsigned
| 29     | **112-bit** integer, signed (two's compliment)
| 30     | **120-bit** integer, unsigned
| 31     | **120-bit** integer, signed (two's compliment)
| 32     | **128-bit** integer, unsigned
| 33     | **128-bit** integer, signed (two's compliment)
| 34     | IEEE754-2008 binary16 floating point number
| 35     | IEEE754-2008 binary32 floating point number
| 36     | IEEE754-2008 binary64 floating point number
| 37     | _reserved_ (IEEE754-2008 binary128 floating point number)
| 38     | _reserved_ (IEEE754-2008 binary256 floating point number)
| 39     | boolean value
| 39     | boolean value `true`
| 40     | boolean value `false`
| 41     | heterogenous array (8)
| 42     | heterogenous array (16)
| 43     | heterogenous array (24)
| 44     | heterogenous array (XL)
| 45     | homogenous array (8)
| 46     | homogenous array (16)
| 47     | homogenous array (24)
| 48     | homogenous array (XL)
| 49     | boolean array (8)
| 50     | boolean array (16)
| 51     | boolean array (XL)

### Integers

To store an integer, you write first the marker byte, then the bytes needed for that type of integer, little endian order (small bytes first). NOTE: little endian order is opposite to what you might expect usually (big endian, also known as "network order").

For example, to write a 96-bit signed integer, you would first write one byte `25`, then you would write exactly 12 more bytes to encoded that integer.

Implementations are _encouraged_, but not required, to find the smallest type able to store the given integer. Doing so will increase output size efficiency.

There is a way to get the amount of bytes needed for a specific integer by the marker byte, by using the formula of `marker >> 1` (integer division by 2).

If the stored integer is too large to fit the expected deserialisation type, that is an error. For example, trying to deserialise a u40 into a u32 value is not allowed.

### Floats

Floats are stored like integers (also little endian), marker, then bytes. As with integers, it is recommended, but not required, to pack a floating value down to a smaller one if it is possible to do so losslessly.

The formula for getting the amount of bytes needed from the marker _does not work here_.

There are markers reserved for 128- and 256-bit floats as defined by IEEE754-2008; however, at the time of writing, it is quite uncommon for a programming language to support these float types.

If the target programming language does not support a 16-bit float, it can be losslessly expanded to a 32- or 64-bit float.

### Booleans

The marker is also the value for bool values. Just write the appropriate marker (one byte total) and that's it.

Note: Use the ones with specific values (the ones marked with `true` and `false` in the table). The generic bool type marker is the same as the true marker, as they won't ever be used in the same context side by side.

### Arrays (heterogenous)

Heterogenous arrays are arrays that can store multiple different types of values (like in JSON).

- heterogenous array (8) uses one byte to store the length for up to 255 items. The marker for the int type is not stored.
- heterogenous array (16) uses two bytes (little endian) to store the length for up to 65,535 items. The marker for the int type is not stored.
- heterogenous array (24) uses three bytes (little endian) to store the length for up to roughly 16.7 million items (2²⁴ - 1 items, or 16,777,216 items, precisely). The marker for the int type is not stored.
- heterogenous array (XL) uses any unsigned integer type to store the length. The length number is serialised just after the array marker, including the int marker.

First the marker for the array itself is stored, then the appropriate length is stored, then the items themselves are stored one after the other.

### Arrays (homogenous)

Homogenous arrays are arrays with only one type of item, meaning all the markers are identical, meaning that can be stored a bit more efficiently.

The different variants of homogenous array (8, 16, 24, XL) have the encoding of the length defined the same way as the corresponding heterogenous array size (homogenous 8 has same length encoding as heterogenous 8, homogenous 16 with heterogenous 16, etc).

First the marker for the array itself is stored, then the marker of the contained items is stored, followed by the length encoded the appropriate way, followed by the items. However, items are encoded without their corresponding prefix (as they are all identical, and this one prefix is already stored).

Boolean note: For the type marker, use the generic bool marker (which is the same as the true marker), and use the true/false markers as the values. However, there are much more efficient ways to encode booleans. See the below section on an [array of booleans](#array-of-booleans).

### Array of booleans

This is a specialised array type just for bools. It packs 8 of them down into one byte, letting the output size be an amortised 1/8 (12.5%) of using one byte per bool. Compared to boolean literals in JSON (`true` or `false`), in just the literal itself, that's packing 32x to 40x more bools into the same space!

The encoding for the length is the same as the past 2 types of arrays... However, what this length _means_ is a bit different. 1 in this length means 1 _byte_, or 1 _group of 8 booleans_. Specifically, its encoding the _number of bytes after the special bytes that are completely packed with booleans_.

First, you encode in the marker for the array type itself. Then, you encode the length (to be further defined) as appropriate for the array type. The next byte is the last special byte. The first _4 bits_ encode the "remainder" (could have been 3, but there was an edge case, and one extra bit gives enough room to work around it), or what doesn't fit the full frame. The remaining 4 bits encode the first 4 booleans, and the rest get packed into the subsequent bytes. The last 4 bits in the special byte will always be filled first before the subsequent bytes (ie. there will never be "holes" in bits whose job it is to store bools).

I get that this is probably way too overcomplicated for its own good, and its still probably confusing. But, I guess this is where the silly part of this crate comes into play :p. It doesn't make the encoder any less reliable, it just uses one byte or so less of space that it would have otherwise, half of the time (which... isn't that much honestly, in the grand scheme of things... ah well). Hopefully examples below will help clear things up.

#### Example: 0 booleans

```txt
00110001 00000000 00000000
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (0). The third byte is the "special" byte, the first four bits encode the "remainder" (0b0000 = 0), and the remaining 4 bits are unused.

#### Example: 2 booleans

```txt
Encoding: [true, false]

00110001 00000000 00101000
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (still 0). The first 4 bits of the third byte encode the "remainder" (0b0010 = 2). The next 2 bits after that encode the 2 booleans (10 = true, false). The last 2 bits are unused.

#### Example: 4 booleans

This is the number at which the special byte gets filled completely, at which point the next one would need to be added to the next byte.

```txt
Encoding: [false, true, true, false]

00110001 00000000 01000110
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (0 again, as we've not even overflowed yet). The first 4 bits of the third byte encode the "remainder" (0b0100 = 4), the remaining 4 bits encode the 4 bools (0110 = false, true, true, false). This fits perfectly in the third byte, and no more are used.

#### Example: 5 booleans

So, we've just overflowed to the 4th byte! :o

```txt
Encoding: [false, true, true, false, true]

00110001 00000000 01010110 10000000
```

First byte is boolean array 8 marker. Second byte is amount of fully packed bytes (still 0, we've not fully filled that 4th byte yet). First 4 bits of the third byte encode the remainder, which would include that extra bit in the next byte (0b0101). Last 4 bits of the third byte encode the first 4 bools (0110 = false, true, true, false). The first bit of the 4th byte encodes the last bool (1 = true).

#### Example: 12 booleans

This amount perfectly packs the first full byte.

```txt
Encoding: [true, false, true, false, true, false, true, false, true, false, true, false]

00110001 00000001 00001010 10101010
```

First byte is boolean array 8 marker. Second byte is amount of fully packed bytes (1 now!). First 4 bits of third byte is the amount of remainder. This time, this is set to 0. Since there are fully packed bytes, the decoder can assume that the next 4 bits are filled with bools. The remaining 8 is encoded in the fourth byte, and the information that this byte is full is in the length (that 1 in the second byte).

#### Example: 17 booleans

Going 5 over the first packed byte.

```txt
Encoding: [true, false, true, false, ... for 17 bools]

00110001 00000001 01011010 10101010 10101000
```

The second bit encodes the amount of fully packed bytes (1). The first 4 bits of the third byte encodes 5, which is the remainder after the last fully packed byte. The total amount is 4 from the special byte, plus 8 from the single packed byte, plus 5 remainder, totalling 17.

#### Misc other examples

Remember, there will be no holes!

DISCLAIMER: I have not checked this table (yet), I have no clue if its actually 100% correct, there may be errors in it!

| bool count | count | remainder | serialised byte count
| --- | -- | -- | -
| 0   | 0  | 0  | 3
| 1   | 0  | 1  | 3
| 2   | 0  | 2  | 3
| 3   | 0  | 3  | 3
| 4   | 0  | 4  | 3
| 5   | 0  | 5  | 4
| 6   | 0  | 6  | 4
| 7   | 0  | 7  | 4
| 8   | 0  | 8  | 4
| 9   | 0  | 9  | 4
| 10  | 0  | 10 | 4
| 11  | 0  | 11 | 4
| 12  | 1  | 0  | 4
| 13  | 1  | 1  | 5
| 14  | 1  | 2  | 5
| 15  | 1  | 3  | 5
| 16  | 1  | 4  | 5
| 17  | 1  | 5  | 5
| 18  | 1  | 6  | 5
| 19  | 1  | 7  | 5
| 20  | 2  | 0  | 5
| 25  | 2  | 5  | 6
| 30  | 3  | 2  | 7
| 35  | 3  | 7  | 7
| 40  | 4  | 4  | 8
| 45  | 5  | 1  | 9
| 50  | 5  | 6  | 9
| 100 | 12 | 0  | 15

#### Why on earth?????

Why not?
