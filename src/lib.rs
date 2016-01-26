#![feature(raw)]
extern crate libc;
extern crate brotli;
use std::io::{ Cursor, Read };
use brotli::Decompressor;
use std::raw::Slice;
use std::mem;
/*
 * Function decompresses the input stream and stores it in the output buffer
 * Args:
 *    input_buffer_ptr: pointer to the input buffer (compressed stream)
 *    input_length: length of the input buffer
 *    output_buffer_ptr: pointer to the output buffer, decompressed output
 *                       will be written to this buffer
 *    output_length: length of the output buffer
 */ 
#[no_mangle]
pub extern fn brotli_compression_mock(input_buffer_ptr: *const libc::c_char, input_length: libc::size_t, 
                                      output_buffer_ptr: *const libc::c_char, output_length: libc::size_t) {
    let input_slice: Slice<u8> = Slice {
        data: input_buffer_ptr as *const u8,
        len: input_length as usize
    };

    let input_bytes: &mut [u8] = unsafe { mem::transmute(input_slice) };
    let input_brotli_stream = Cursor::new(input_bytes);

    let output_slice: Slice<u8> = Slice {
        data: output_buffer_ptr as *const u8,
        len: output_length as usize
    };
    let output_bytes: &mut [u8] = unsafe { mem::transmute(output_slice) };
    
    // Invoke the brotli decompressor on the input stream, write to the output byte array
    let _ = Decompressor::new(input_brotli_stream).read_exact(output_bytes);
}
