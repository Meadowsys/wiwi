# wiwi serialiser spec <!-- omit from toc -->

**NOTE**: This spec is still a work in progress and _SHOULD NOT_ be relied on for _anything_ intended to be used in production. Feel free to toy around with it and suggest things, but this should be considered highly unstable until at least its pushed to crates.io.

Note to self: needs more diagrams. Diagrams are nice.

## Table of contents <!-- omit from toc -->

- [About wiwi serialiser](#about-wiwi-serialiser)
- [Implementation Status](#implementation-status)
- [Spec](#spec)
  - [General structure](#general-structure)
  - [Markers](#markers)
  - [Collection variants](#collection-variants)
  - [None](#none)
  - [Integers](#integers)
  - [Floats](#floats)
  - [Booleans](#booleans)
  - [Arrays (heterogenous)](#arrays-heterogenous)
  - [Arrays (homogenous)](#arrays-homogenous)
  - [Array of booleans](#array-of-booleans)
  - [UTF-8 String](#utf-8-string)
  - [Object](#object)
  - [Object (key type known)](#object-key-type-known)
  - [Object (value type known)](#object-value-type-known)
  - [Object (key/value types known)](#object-keyvalue-types-known)
  - [Object array (structure known)](#object-array-structure-known)
  - [Object array (value types grouped)](#object-array-value-types-grouped)
  - [Object array (key types consistent)](#object-array-key-types-consistent)
  - [Object array (key consistent, val grouped)](#object-array-key-consistent-val-grouped)
  - [Object array (key/val consistent)](#object-array-keyval-consistent)
  - [Types for future consideration](#types-for-future-consideration)

## About wiwi serialiser

This does not have a formal name any more than "wiwi serialiser":p. Wiwi serialiser is a binary format serialisation similar to JSON.

Some goals of this serialiser, in _rough_ order of priority:

- Stable, self-describing format
- Tiny output size
- Fast

These are non-goals (but still nice-to-haves):

- Compressible by major compression algorithms, especially web ones (gzip, brotli)
- Compat with other languages (serialisers may be written for them. The majority of types that people would actually use will be available in other languages, but they might not be able to process the whole range of types available (looking at you, 128-bit integers))

## Implementation Status

- [x] None
- [x] Integers
- [x] Floats
- [x] Booleans
- [x] Heterogenous arrays
- [ ] Homogenous arrays
- [ ] Boolean arrays
- [ ] Strings
- [ ] Objects (key type known)
- [ ] Objects (value type known)
- [ ] Objects (key/value type known)
- [ ] Object array (struct known)
- [ ] Object array (val type grouped)
- [ ] Object array (key type consistent)
- [ ] Object array (k consistent v grouped)
- [ ] Object array (k/v consistent)

## Spec

### General structure

The general structure of an item serialised is a marker byte that is unique to that type, followed by the data in a specific format following it. The length of the data can always be determined, either by a property of the type (for example, a i32 will always take 4 bytes of space), or specified somewhere towards the beginning where it can be deterministically found before the variable length section (for example, storing the length before an array).

- Marker byte (one byte, until we get "close to" overflow, then 2 bytes for new ones)
- Metadata (if applicable for the type)
- Data (if applicable for the type)

### Markers

| Marker | Type
| ------ | ------------------------------------------------------------------
| 0      | None
| 1      | _unassigned_
| 2      | 8-bit [integer], unsigned
| 3      | 8-bit [integer], signed (two's compliment)
| 4      | 16-bit [integer], unsigned
| 5      | 16-bit [integer], signed (two's compliment)
| 6      | 24-bit [integer], unsigned
| 7      | 24-bit [integer], signed (two's compliment)
| 8      | 32-bit [integer], unsigned
| 9      | 32-bit [integer], signed (two's compliment)
| 10     | 40-bit [integer], unsigned
| 11     | 40-bit [integer], signed (two's compliment)
| 12     | 48-bit [integer], unsigned
| 13     | 48-bit [integer], signed (two's compliment)
| 14     | 56-bit [integer], unsigned
| 15     | 56-bit [integer], signed (two's compliment)
| 16     | 64-bit [integer], unsigned
| 17     | 64-bit [integer], signed (two's compliment)
| 18     | 72-bit [integer], unsigned
| 19     | 72-bit [integer], signed (two's compliment)
| 20     | 80-bit [integer], unsigned
| 21     | 80-bit [integer], signed (two's compliment)
| 22     | 88-bit [integer], unsigned
| 23     | 88-bit [integer], signed (two's compliment)
| 24     | 96-bit [integer], unsigned
| 25     | 96-bit [integer], signed (two's compliment)
| 26     | 104-bit [integer], unsigned
| 27     | 104-bit [integer], signed (two's compliment)
| 28     | 112-bit [integer], unsigned
| 29     | 112-bit [integer], signed (two's compliment)
| 30     | 120-bit [integer], unsigned
| 31     | 120-bit [integer], signed (two's compliment)
| 32     | 128-bit [integer], unsigned
| 33     | 128-bit [integer], signed (two's compliment)
| 34     | IEEE754-2008 binary16 [floating point number]
| 35     | IEEE754-2008 binary32 [floating point number]
| 36     | IEEE754-2008 binary64 [floating point number]
| 37     | _reserved_ (IEEE754-2008 binary128 [floating point number])
| 38     | _reserved_ (IEEE754-2008 binary256 [floating point number])
| 39     | [boolean value]
| 39     | [boolean value] `true`
| 40     | [boolean value] `false`
| 41     | [heterogenous array] (8)
| 42     | [heterogenous array] (16)
| 43     | [heterogenous array] (24)
| 44     | [heterogenous array] (XL)
| 45     | [homogenous array] (8)
| 46     | [homogenous array] (16)
| 47     | [homogenous array] (24)
| 48     | [homogenous array] (XL)
| 49     | [boolean array] (8)
| 50     | [boolean array] (16)
| 51     | [boolean array] (XL)
| 52     | [string] (8)
| 53     | [string] (16)
| 54     | [string] (24)
| 55     | [string] (XL)
| 56     | [object] (8)
| 57     | [object] (16)
| 58     | [object] (24)
| 59     | [object] (XL)
| 60     | [object (key ty known)] (8)
| 61     | [object (key ty known)] (16)
| 62     | [object (key ty known)] (24)
| 63     | [object (key ty known)] (XL)
| 64     | [object (val ty known)] (8)
| 65     | [object (val ty known)] (16)
| 66     | [object (val ty known)] (24)
| 67     | [object (val ty known)] (XL)
| 68     | [object (key/val ty known)] (8)
| 69     | [object (key/val ty known)] (16)
| 70     | [object (key/val ty known)] (24)
| 71     | [object (key/val ty known)] (XL)
| 72     | [object array (struct known)] (len 8, keys 8)
| 73     | [object array (struct known)] (len 8, keys 16)
| 74     | [object array (struct known)] (len 8, keys 24)
| 75     | [object array (struct known)] (len 8, keys XL)
| 76     | [object array (struct known)] (len 16, keys 8)
| 77     | [object array (struct known)] (len 16, keys 16)
| 78     | [object array (struct known)] (len 16, keys 24)
| 79     | [object array (struct known)] (len 16, keys XL)
| 80     | [object array (struct known)] (len 24, keys 8)
| 81     | [object array (struct known)] (len 24, keys 16)
| 82     | [object array (struct known)] (len 24, keys 24)
| 83     | [object array (struct known)] (len 24, keys XL)
| 84     | [object array (struct known)] (len XL, keys 8)
| 85     | [object array (struct known)] (len XL, keys 16)
| 86     | [object array (struct known)] (len XL, keys 24)
| 87     | [object array (struct known)] (len XL, keys XL)
| 88     | [object array (val ty grouped)] (len 8, keys 8)
| 89     | [object array (val ty grouped)] (len 8, keys 16)
| 90     | [object array (val ty grouped)] (len 8, keys 24)
| 91     | [object array (val ty grouped)] (len 8, keys XL)
| 92     | [object array (val ty grouped)] (len 16, keys 8)
| 93     | [object array (val ty grouped)] (len 16, keys 16)
| 94     | [object array (val ty grouped)] (len 16, keys 24)
| 95     | [object array (val ty grouped)] (len 16, keys XL)
| 96     | [object array (val ty grouped)] (len 24, keys 8)
| 97     | [object array (val ty grouped)] (len 24, keys 16)
| 98     | [object array (val ty grouped)] (len 24, keys 24)
| 99     | [object array (val ty grouped)] (len 24, keys XL)
| 100    | [object array (val ty grouped)] (len XL, keys 8)
| 101    | [object array (val ty grouped)] (len XL, keys 16)
| 102    | [object array (val ty grouped)] (len XL, keys 24)
| 103    | [object array (val ty grouped)] (len XL, keys XL)
| 104    | [object array (key ty consistent)] (len 8, keys 8)
| 105    | [object array (key ty consistent)] (len 8, keys 16)
| 106    | [object array (key ty consistent)] (len 8, keys 24)
| 107    | [object array (key ty consistent)] (len 8, keys XL)
| 108    | [object array (key ty consistent)] (len 16, keys 8)
| 109    | [object array (key ty consistent)] (len 16, keys 16)
| 110    | [object array (key ty consistent)] (len 16, keys 24)
| 111    | [object array (key ty consistent)] (len 16, keys XL)
| 112    | [object array (key ty consistent)] (len 24, keys 8)
| 113    | [object array (key ty consistent)] (len 24, keys 16)
| 114    | [object array (key ty consistent)] (len 24, keys 24)
| 115    | [object array (key ty consistent)] (len 24, keys XL)
| 116    | [object array (key ty consistent)] (len XL, keys 8)
| 117    | [object array (key ty consistent)] (len XL, keys 16)
| 118    | [object array (key ty consistent)] (len XL, keys 24)
| 119    | [object array (key ty consistent)] (len XL, keys XL)
| 120    | [object array (k consistent, v grouped)] (len 8, keys 8)
| 121    | [object array (k consistent, v grouped)] (len 8, keys 16)
| 122    | [object array (k consistent, v grouped)] (len 8, keys 24)
| 123    | [object array (k consistent, v grouped)] (len 8, keys XL)
| 124    | [object array (k consistent, v grouped)] (len 16, keys 8)
| 125    | [object array (k consistent, v grouped)] (len 16, keys 16)
| 126    | [object array (k consistent, v grouped)] (len 16, keys 24)
| 127    | [object array (k consistent, v grouped)] (len 16, keys XL)
| 128    | [object array (k consistent, v grouped)] (len 24, keys 8)
| 129    | [object array (k consistent, v grouped)] (len 24, keys 16)
| 130    | [object array (k consistent, v grouped)] (len 24, keys 24)
| 131    | [object array (k consistent, v grouped)] (len 24, keys XL)
| 132    | [object array (k consistent, v grouped)] (len XL, keys 8)
| 133    | [object array (k consistent, v grouped)] (len XL, keys 16)
| 134    | [object array (k consistent, v grouped)] (len XL, keys 24)
| 135    | [object array (k consistent, v grouped)] (len XL, keys XL)
| 136    | [object array (k/v consistent)] (len 8, keys 8)
| 137    | [object array (k/v consistent)] (len 8, keys 16)
| 138    | [object array (k/v consistent)] (len 8, keys 24)
| 139    | [object array (k/v consistent)] (len 8, keys XL)
| 140    | [object array (k/v consistent)] (len 16, keys 8)
| 141    | [object array (k/v consistent)] (len 16, keys 16)
| 142    | [object array (k/v consistent)] (len 16, keys 24)
| 143    | [object array (k/v consistent)] (len 16, keys XL)
| 144    | [object array (k/v consistent)] (len 24, keys 8)
| 145    | [object array (k/v consistent)] (len 24, keys 16)
| 146    | [object array (k/v consistent)] (len 24, keys 24)
| 147    | [object array (k/v consistent)] (len 24, keys XL)
| 148    | [object array (k/v consistent)] (len XL, keys 8)
| 149    | [object array (k/v consistent)] (len XL, keys 16)
| 150    | [object array (k/v consistent)] (len XL, keys 24)
| 151    | [object array (k/v consistent)] (len XL, keys XL)

### Collection variants

Some collections will have variants, like the "heterogenous array (8)" and "heterogenous array (XL)". Here is what they mean:

- (8) means length is stored with a single 8-bit unsigned integer. Just the value itself is written; no marker is used. This can encode lengths up to 255.
- (16) is the same as above, but with a single 16-bit unsigned little endian integer. As above, no marker is used. This can encode lengths up to 65,535.
- (24) is the same as above, but with a 24-bit unsigned little endian integer. This can encode lengths up to 16,777,216 (around 16.7 million).
- (XL) means any of the available unsigned integer types may be used to encode the length. Theoretically 2¹²⁸ - 1 is the maximum value for this, but the practical limit is likely well under that (well, at least at time of writing this spec, who knows what technological advancements will happen in the future :p).

### None

Also known as `null` or `nil` in other languages. The marker represents the value, so just write the marker byte.

In this type system, where applicable/possible, a `None` value represents an absence of a value, rather than a `null` sort of value. For example, a `None` value in an object would mean that key does not exist, rather than its set to None. In a language like JavaScript or deserialising into a `Value`, nothing would be put. However, if a rigid structure is used (like Rust structs), the the optional value would be decoded as `None`.

In terms of "knowing types" for specialised variants of some structures (ex. array and object), `None` is to be considered a seperate type, as there is no way to reliably mark a null value in a densely packed structures without internal markers. For example, in a byte array, there is no reasonable way to tell if an item is a null marker and not a byte value.

### Integers

Signed (two's compliment) or unsigned integer, variable bit sizes from 8 to 128, available in all multiples of 8.

To store an integer, you write first the marker byte, then the bytes needed for that type of integer, little endian order (small bytes first). NOTE: little endian order is opposite to what you might expect usually (big endian, also known as "network order").

For example, to write a 96-bit signed integer, you would first write one byte `25`, then you would write exactly 12 more bytes to encoded that integer.

Implementations are required to find the smallest type able to store the given integer. Failing to do so isn't valid per se, but may cause some deserialisers to behave in unexpected ways. Because of this requirement, deserialisers are allowed to rely on the type for the maximum value, so they can do things like always reject 40-bit ints if the target container is only a 32-bit int, because it _will_ be too big.

There is a way to get the amount of bytes needed for a specific integer by the marker byte, by using the formula of `marker >> 1` (integer division by 2).

### Floats

Floating point values, `binary16`, `binary32`, and `binary64`, as defined in IEEE754-2008. Markers for binary128 and binary256 are reserved, but not to be implemented at this time, since their use is quite uncommon at this time. In fact, the first time I heard that they were specified at all, was while I was researching these floating point values for this serialisation library.

Floats are stored like integers (also little endian), marker, then bytes. It is recommended, but not required, to pack a floating value down to a smaller one if it is possible to do so losslessly. As of writing, I have not figured out how to do this yet, so floats just get stored as is.

The formula for getting the amount of bytes needed from the marker _does not work here_.

There are markers reserved for 128- and 256-bit floats as defined by IEEE754-2008; however, at the time of writing, it is quite uncommon for a programming language to support these float types, so for now they are _not_ supported.

If the target programming language does not support a 16-bit float, it can be losslessly expanded to a 32- or 64-bit float.

### Booleans

`true`, or `false`.

The marker is also the value for bool values. Just write the appropriate marker (one byte total) and that's it.

Note: Use the ones with specific values (the ones marked with `true` and `false` in the table). The generic bool type marker is the same as the true marker, as they won't ever be used in the same context side by side.

### Arrays (heterogenous)

Arrays that can store multiple different types of values (like in JSON and dynamic languages like JavaScript). This is the most general form of an array, with specialisations available below.

First the marker for the array itself is stored, then the length is stored (see [section on collection variants]), then the items themselves are stored one after the other.

### Arrays (homogenous)

Arrays that store only one type of item (identical = same marker), meaning that can be stored a bit more compact/efficiently.

First the marker for the array itself is stored, then the marker of the contained items is stored, followed by the length of the array (see [section on collection variants]), followed by the items. However, items are encoded without their corresponding prefix (as they are all identical, and this one prefix is already stored).

Boolean note: For the type marker, use the generic bool marker (which is the same as the true marker), and use the true/false markers as the values. However, there are much more efficient ways to encode booleans. See the below section on an [array of booleans](#array-of-booleans).

### Array of booleans

Specialised array type just for bools. It packs 8 of them down into one byte, letting the output size be an amortised 1/8 (12.5%) of using one byte per bool. Compared to boolean literals in JSON (`true` or `false`), in just the literal itself, that's packing 32x to 40x more bools into the same space!

The encoding for the length and variants is the same as the past 2 types of arrays... However, what this length _means_ is a bit different. 1 in this length means 1 _byte_, or 1 _fully-packed group of 8 booleans_. Specifically, its encoding the _number of bytes after the special bytes that are completely packed with booleans_.

First, you encode in the marker for the array type itself. Then, you encode the length (to be further defined) as appropriate for the array type (see [section on collection variants]). The next byte is the last special byte. The first _4 bits_ encode the "remainder", or what doesn't fit the full frame. The remaining 4 bits encode the first 4 booleans, and the rest get packed into the subsequent bytes. The last 4 bits in the special byte will always be filled first before the subsequent bytes (ie. there will never be "holes" in boolean bits).

I get that this is probably way too overcomplicated for its own good, and its still probably confusing. But, I guess this is where the silly part of this crate comes into play :p. It doesn't make the encoder any less reliable, it just uses one byte or so less of space that it would have otherwise, half of the time (which... isn't that much honestly, in the grand scheme of things... ah well). Hopefully examples below will help clear things up.

#### Example: 0 booleans <!-- omit from toc -->

```txt
00110001 00000000 00000000
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (0). The third byte is the "special" byte, the first four bits encode the "remainder" (0b0000 = 0), and the remaining 4 bits are unused.

#### Example: 2 booleans <!-- omit from toc -->

```txt
Encoding: [true, false]

00110001 00000000 00101000
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (still 0). The first 4 bits of the third byte encode the "remainder" (0b0010 = 2). The next 2 bits after that encode the 2 booleans (10 = true, false). The last 2 bits are unused.

#### Example: 4 booleans <!-- omit from toc -->

This is the number at which the special byte gets filled completely, at which point the next one would need to be added to the next byte.

```txt
Encoding: [false, true, true, false]

00110001 00000000 01000110
```

The first byte is the marker for a boolean array 8 (49). The second byte is the amount of fully packed bytes (0 again, as we've not even overflowed yet). The first 4 bits of the third byte encode the "remainder" (0b0100 = 4), the remaining 4 bits encode the 4 bools (0110 = false, true, true, false). This fits perfectly in the third byte, and no more are used.

#### Example: 5 booleans <!-- omit from toc -->

So, we've just overflowed to the 4th byte! :o

```txt
Encoding: [false, true, true, false, true]

00110001 00000000 01010110 10000000
```

First byte is boolean array 8 marker. Second byte is amount of fully packed bytes (still 0, we've not fully filled that 4th byte yet). First 4 bits of the third byte encode the remainder, which would include that extra bit in the next byte (0b0101). Last 4 bits of the third byte encode the first 4 bools (0110 = false, true, true, false). The first bit of the 4th byte encodes the last bool (1 = true).

#### Example: 12 booleans <!-- omit from toc -->

This amount perfectly packs the first full byte.

```txt
Encoding: [true, false, true, false, true, false, true, false, true, false, true, false]

00110001 00000001 00001010 10101010
```

First byte is boolean array 8 marker. Second byte is amount of fully packed bytes (1 now!). First 4 bits of third byte is the amount of remainder. This time, this is set to 0. Since there are fully packed bytes, the decoder can assume that the next 4 bits are filled with bools. The remaining 8 is encoded in the fourth byte, and the information that this byte is full is in the length (that 1 in the second byte).

#### Example: 17 booleans <!-- omit from toc -->

Going 5 over the first packed byte.

```txt
Encoding: [true, false, true, false, ... for 17 bools]

00110001 00000001 01011010 10101010 10101000
```

The second bit encodes the amount of fully packed bytes (1). The first 4 bits of the third byte encodes 5, which is the remainder after the last fully packed byte. The total amount is 4 from the special byte, plus 8 from the single packed byte, plus 5 remainder, totalling 17.

#### Misc other examples <!-- omit from toc -->

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

#### Why on earth????? <!-- omit from toc -->

Why not?

### UTF-8 String

A string, strictly UTF-8 encoding. Decoding it successfully would include checking it to make sure its valid UTF-8.

First write the marker for the string type, followed by the length (in bytes not characters) (see [section on collection variants]). Then, write the string bytes themselves.

### Object

Analogus to the object type in JSON, also known as maps or dictionaries in some languages, or more generally, a key to value mapping. This is the most general form of an object, with specialisations available below.

The length here refers to the amount of key value mappings.

First write the marker for the object type, then write the length (see [section on collection variants]). For every key-value pair, first encode the key (including marker), then encode the value (including marker).

- Marker for the specific object variant
- Length for that specific variant (see [section on collection variants])
- for each key value pair
  - write key (with marker)
  - write value (with marker)

It is recommended, but not required, for all object types to sort the entries by key, from "least" to "greatest", if possible. That way, the same data written into an object will be deterministic.

### Object (key type known)

An object/map/dictionary with a known key type. Also, actually... This specialisation of the object type with string keys is more akin to what JSON has, since JSON objects always have string keys.

First write the marker for the object, followed by the marker for the key type, followed by the length (see [section on collection variants]). Then, put in the key value pairs. Skip the marker on the key types, as it is already encoded in the beginning.

- Marker
- Key type marker
- Length
- for each key value pair
  - Key (without marker)
  - Value (with marker)

### Object (value type known)

If in an object the key type is not known, but the value type is known...

Write the marker for the object, followed by the marker for the value type, followed by the length (see [section on collection variants]). Then, put in the key value pairs, skipping the markers for the values, as that is already known.

- Marker
- Value type marker
- Length
- for each key value pair
  - Key (with marker)
  - Value (without marker)

### Object (key/value types known)

Objects where all the keys are known to have the same type as each other, and same for values.

Write the marker for the object, followed by the marker for the key type, followed by the marker for the value type, followed by the length (see [section on collection variants]). Then put in the key value pairs, skipping markers for both the keys and the values.

- Marker
- Key marker
- Value marker
- for each in key value pairs
  - write the key (without marker)
  - write the value (without marker)

### Object array (structure known)

- Part of "Array of objects" group of specialisations
- What keys each obj has is known, but no types are

- Marker
- Length of array (or amount of objects)
- Number of keys in each object
- for each in key
  - write key (with marker)
  - for each in values
    - write value (with marker)

### Object array (value types grouped)

- Part of "Array of objects" group of specialisations
- Keys are known, but their types do not have to be the same
- Value types are consistent and must be the same per key (but do not have to be the same across keys)

A specialisation of array type, for if you know what keys each object has (doesn't have to be the same type as each other), and the types of the values for each key are known and consistent (but not necessarily between each key)

This array is not grouped by objects, but rather by the keys of each object. You could think of it like a specialised type for many homogenous arrays, each tagged with a key, holding values, side by side, but serialised/deserialised transparently into an array of objects.

Write the marker for this type of object, followed by length of array (or number of objects), then the amount of keys in each object (see [section on collection variants]). <!-- TODO: specify length variant ordering or whatever, since theres two unrelated lengths here, we kinda need specify both seperately in some kinda matrix type thing --> Then, for each key, encode the key itself (including the type marker), then encode the type marker for the value, then encode all the values for each object in the array (without type marker) (ie. `array[0].value`, then `array[1].value`, then `array[2].value`, etc, in a sequence, under the key).

- Part of "Array of objects" group of specialisations
- All objects consist of the same set of known keys
- Keys do not need to be the same type
- Values for each key do need to be the same type
- Values that are for different keys do not have to be the same type

- Marker
- Length of array (or amount of objects)
- Number of keys in each object
- for each in key
  - write key (with marker)
  - write value type marker
  - for each in values
    - write value (without marker)

### Object array (key types consistent)

- Part of "Array of objects" group of specialisations
- Keys are known, and all the same type
- Value types do not have to be the same

- Marker
- Key type marker
- Length of array (or amount of objects)
- Number of keys in each object
- for each in key
  - write key (without marker)
  - for each in values
    - write value (with marker)

### Object array (key consistent, val grouped)

This is the same as above, but all key types are known to be the same, so that can be abstracted out too.

- Part of "Array of objects" group of specialisations
- All objects consist of the same set of known keys
- Keys are known, and all the same type
- Values for the same key are the same type
- Values that are for different keys do not have to be the same type

- Write the marker
- Write key type marker
- Write amount of objects (array len) (see [section on collection variants])
- Write amount of keys per object
- for each key
  - write the key (without marker)
  - write value type marker
  - for each value
    - write the value (without marker)

### Object array (key/val consistent)

A specialisation of the array of objects specialisation, where all keys are the same type, and all values of all keys are the same type.

- Part of "Array of objects" group of specialisations
- All objects consist of the same set of known keys
- Keys are all the same type
- Values are all, regardless of key, the same type

- Write the marker
- Write key type marker
- Write value type marker
- Write amount of objects (array len) (see [section on collection variants])
- Write amounts of keys per object
- for each key
  - write the key (without marker)
  - for each value
    - write the value (without marker)

### Types for future consideration

- Set
- Bit-packing integer arrays (4-bit, 2-bit, 1-bit ints)
- Timestamp

[section on collection variants]: #collection-variants

<!-- links from marker table to specific section -->

[integer]: #integers
[floating point number]: #floats
[boolean value]: #booleans

[heterogenous array]: #arrays-heterogenous
[homogenous array]: #arrays-homogenous
[boolean array]: #array-of-booleans
[string]: #utf-8-string
[object]: #object
[object (key ty known)]: #object-key-type-known
[object (val ty known)]: #object-value-type-known
[object (key/val ty known)]: #object-keyvalue-types-known

[object array (struct known)]: #object-array-structure-known
[object array (val ty grouped)]: #object-array-value-types-grouped
[object array (key ty consistent)]: #object-array-key-types-consistent
[object array (k consistent, v grouped)]: #object-array-key-consistent-val-grouped
[object array (k/v consistent)]: #object-array-keyval-consistent
