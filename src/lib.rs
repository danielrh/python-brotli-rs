extern crate libc;
/*
 * Dummy function that accepts a pointer to a string from python. Mutates
 * the string and returns an integer back to python. 
 */ 
#[no_mangle]
pub extern fn brotli_compression_mock(input: &mut libc::c_char) -> i32 {
    // mutate the string 
    *input += 1;
    // return the length of the mutated string. 
    return 5;
}
