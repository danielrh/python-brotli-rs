extern crate libc;
extern crate brotli;
use std::io::Cursor;
use brotli::Decompressor;
use std::slice;
use std::io::prelude::*;

// Function decompresses the input stream and stores it in the output buffer
// Args:
//    input_buffer_ptr: pointer to the input buffer (compressed stream)
//    input_length: length of the input buffer
//    output_buffer_ptr: pointer to the output buffer, decompressed output
//                       will be written to this buffer
//    output_length: length of the output buffer
//Returns:
//    size of the decompressed bytes, or output_length if the entire buffer was filled
#[no_mangle]
pub extern "C" fn brotli_decompress(input_buffer_ptr: *const libc::c_char,
                                    input_length: libc::size_t,
                                    output_buffer_ptr: *const libc::c_char,
                                    output_length: libc::size_t)
                                    -> libc::size_t {
    let input_slice = unsafe {
        slice::from_raw_parts(input_buffer_ptr as *const u8, input_length as usize)
    };

    let input_brotli_stream = Cursor::new(input_slice);
    let output_bytes = unsafe {
        &mut slice::from_raw_parts_mut(output_buffer_ptr as *mut u8, output_length as usize)
    };

    // Invoke the brotli decompressor on the input stream, write to the output byte array
    let decompressed_stream = &mut Decompressor::new(input_brotli_stream);

    let mut bytes_read = 0 as libc::size_t;
    while bytes_read < output_length {
        match decompressed_stream.read(&mut output_bytes[bytes_read..output_length]) {
            Err(_) => return 0,
            Ok(num_read) => {
                if num_read == 0 {
                    break;
                }
                bytes_read += num_read;
            }
        }
    }
    if bytes_read == 0 && output_length != 0 {
        // so the call site can distinguish a 0 lenght array from an error
        output_bytes[0] = 'z' as u8;
    }
    return bytes_read;
}

#[no_mangle]
pub extern "C" fn brotli_is_zero_stream(input_buffer_ptr: *const libc::c_char,
                                        input_length: libc::size_t)
                                           -> u8 {
    let input_slice = unsafe {
        slice::from_raw_parts(input_buffer_ptr as *const u8, input_length as usize)
    };

    let input_brotli_stream = Cursor::new(input_slice);
    let mut output_bytes = vec![0];

    // Invoke the brotli decompressor on the input stream, write to the output byte array
    let decompressed_stream = &mut Decompressor::new(input_brotli_stream);

    match decompressed_stream.read(&mut output_bytes[0..1]) {
        Err(_) => return 0,
        Ok(num_read) => {
            if num_read == 0 {
                return 1
            }
        }
    }
    return 0;
}
