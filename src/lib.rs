#![feature(raw)]
extern crate libc;
extern crate brotli;
use std::io::Cursor;
use brotli::Decompressor;
use std::raw::Slice;
use std::mem;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
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
    //let input_brotli_stream = Cursor::new(input_bytes);

    // Write input to a file
    let path = Path::new("/tmp/rust_input");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };
    match file.write_all((input_bytes)) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    // END WRITE INPUT

    let input_brotli_stream = Cursor::new(input_bytes);
    let output_slice: Slice<u8> = Slice {
        data: output_buffer_ptr as *const u8,
        len: output_length as usize
    };
    let output_bytes: &mut [u8] = unsafe { mem::transmute(output_slice) };
    
    // Invoke the brotli decompressor on the input stream, write to the output byte array
    let _ = Decompressor::new(input_brotli_stream).read_exact(output_bytes);

    // Write output to file
    let path1 = Path::new("/tmp/rust_output");
    let display1 = path1.display();
    let mut file1 = match File::create(&path1) {
        Err(why) => panic!("couldn't create {}: {}",
                           display1,
                           Error::description(&why)),
        Ok(file) => file,
    };
    match file1.write_all((output_bytes)) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
    // END WRITE OUTPUT
}
