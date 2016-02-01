extern crate libc;
extern crate brotli;
use std::io::Cursor;
use brotli::Decompressor;
use std::slice;
use std::io::prelude::*;

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
pub extern fn brotli_decompress(input_buffer_ptr: *const libc::c_char, input_length: libc::size_t, 
                                      output_buffer_ptr: *const libc::c_char, output_length: libc::size_t) {
    let input_slice = unsafe {slice::from_raw_parts(
         input_buffer_ptr as *const u8,
         input_length as usize)};

    let input_brotli_stream = Cursor::new(input_slice);
    let output_bytes = unsafe {&mut slice::from_raw_parts_mut(
        output_buffer_ptr as *mut u8,
        output_length as usize)};
    
    // Invoke the brotli decompressor on the input stream, write to the output byte array
    let _ = Decompressor::new(input_brotli_stream).read_exact(output_bytes);
}
