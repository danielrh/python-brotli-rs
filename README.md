# python-rust-ffi
Run the following commands to test python rust FFI
```
cargo build --release
python mock.py
```
Note:
The logic to create a Slice<u8> from a pointer and length passed from python seems to be an 'unstable' feature. In order to run this you will have to install the rust nightly build. You can do that using the following command. (There probably is another way to do this)
```
curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
```
