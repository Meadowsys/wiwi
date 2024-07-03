use "path:../../target/release"
use "path:../../target/debug"
use "lib:bindings"

use @wiwi_z85_encode[None](input: Z85Input box, output: NullablePointer[Z85Result] ref)
use @wiwi_z85_decode[None](input: Z85Input box, output: NullablePointer[Z85Result] ref)
use @wiwi_drop_z85_result[None](res: Z85Result ref)

primitive Wiwi
	fun z85_encode(input': Array[U8 val] box): String ref^ =>
		var input = Z85Input(input'.cpointer(), input'.size())
		var output = Z85Result

		@wiwi_z85_encode(input, NullablePointer[Z85Result](output))

		let res = String.copy_cpointer(output.ptr, output.len)
		@wiwi_drop_z85_result(output)

		res

struct Z85Input
	var ptr: Pointer[U8] tag
	var len: USize = 0

	new ref create(ptr': Pointer[U8] tag, len': USize) =>
		ptr = ptr'
		len = len'

struct Z85Result
	var ptr: Pointer[U8] = Pointer[U8]
	var len: USize = 0
	var cap: USize = 0

	new create() => None
