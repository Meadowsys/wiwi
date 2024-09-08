Includes nonstandard padding. The Z85 spec explicitly says that the application needs to handle padding the length to a multiple of 4. If input length is a multiple of 4, no padding is added.

# Nonstandard padding implementation

**Encoding**: If padding is needed, the amount of padding that was added in bytes is encoded (ex. 1B padding -> `1` since `TABLE_ENCODER[1] == b'1'`) and appended to the end of the string. (1 extra byte)

**Decoding**: If the len of the slice passed is one more than a multiple of 5 (ie. `(n * 5) + 1`), it is trimmed off the slice, decoded to get amount of padding needed, and held onto. Then while decoding the last frame, we take that stored amount of padding, and remove that amount from the end of the decoded bytes.

Original Z85 spec: <https://rfc.zeromq.org/spec/32>
