use "path:../../target/release"
use "path:../../target/debug"
use "lib:bindings"

use @wiwi_z85_encode[None](input: Z85Input tag, output: NullablePointer[Z85Result] tag)
use @wiwi_z85_decode[None](input: Z85Input tag, output: NullablePointer[Z85Result] tag)
use @wiwi_z85_drop_result[None](res: Z85Result tag)

primitive Wiwi
	fun z85_encode(input': Array[U8 val] box): String ref^ =>
		var input = Z85Input(input'.cpointer(), input'.size())

		with output = Z85Result do
			@wiwi_z85_encode(input, NullablePointer[Z85Result](output))
			String.copy_cpointer(output.ptr, output.len)
		end

	fun z85_decode(input': Array[U8 val] box): (Array[U8 val] ref^ | None) =>
		var input = Z85Input(input'.cpointer(), input'.size())

		let res = recover
			with output = Z85Result do
				@wiwi_z85_decode(input, NullablePointer[Z85Result](output))

				if output.ptr.is_null() then
					return None
				end

				String.copy_cpointer(output.ptr, output.len)
			end
		end

		(consume res).iso_array()

struct Z85Input
	var ptr: Pointer[U8] tag
	var len: USize

	new ref create(ptr': Pointer[U8] tag, len': USize) =>
		ptr = ptr'
		len = len'

struct Z85Result
	var ptr: Pointer[U8] = Pointer[U8]
	var len: USize = 0
	var cap: USize = 0

	new create() => None

	fun dispose() =>
		@wiwi_z85_drop_result(this)
