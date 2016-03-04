import ctypes
from ctypes import cdll

try:
    lib = cdll.LoadLibrary("target/release/libembed.dylib")
except EnvironmentError:
    try:
        lib = cdll.LoadLibrary("target/release/libembed.dll")
    except EnvironmentError:
            lib = cdll.LoadLibrary("target/release/libembed.so")

# compressed input brotli stream
input_compressed_stream = bytearray('\x1b\x13\x00\x00\xa4\xb0\xb2\xea\x81\x47\x02\x8a')
input_length = len(input_compressed_stream)
# transform it into a ctype that can be passed into rust 
input_array = (ctypes.c_char * 12).from_buffer(input_compressed_stream)
input_char_ptr = ctypes.cast(input_array, ctypes.c_char_p)

# hard coded output buffer length
output_length = 25
# allocate output bytearray
output_buffer = bytearray(output_length); 
output_array = (ctypes.c_char * output_length).from_buffer(output_buffer)
output_char_ptr = ctypes.cast(output_array, ctypes.c_char_p)

final_len = lib.brotli_decompress(input_char_ptr, input_length, output_char_ptr, output_length)
print("Decompressed string(" + str(final_len + 0) + ") : " + output_buffer[0:final_len])


input_compressed_stream = bytearray(';')
input_length = len(input_compressed_stream)
# transform it into a ctype that can be passed into rust
input_array = (ctypes.c_char * 1).from_buffer(input_compressed_stream)
input_char_ptr = ctypes.cast(input_array, ctypes.c_char_p)

# hard coded output buffer length
output_length = 1
# allocate output bytearray
output_buffer = bytearray(output_length);
output_array = (ctypes.c_char * output_length).from_buffer(output_buffer)
output_char_ptr = ctypes.cast(output_array, ctypes.c_char_p)

final_len = lib.brotli_decompress(input_char_ptr, input_length, output_char_ptr, output_length)
assert final_len == 0
is_zero = lib.brotli_is_zero_stream(input_char_ptr, input_length)
assert not not is_zero
print("Decompressed string ; is zero for brotli:" + str(is_zero + 0) + "")
