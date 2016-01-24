import ctypes
from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libembed.dylib")
block_dump = "ABCDE"
print("block dump before invoking rust code: " + block_dump)
ptr = ctypes.c_char_p(block_dump)
length = lib.brotli_compression_mock(ptr)
print("block dump after invoking rust code: " + block_dump + " length: " + str(length))
