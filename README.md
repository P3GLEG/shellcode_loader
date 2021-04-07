### How to compile for linux
```bash
cargo build --release --target x86_64-unknown-linux-gnu --bin shellcode_loader
```

### How to compile for mac
`cargo build -v --release --target x86_64-apple-darwin`


### How to compile on Windows
`cargo build --release --target i686-pc-windows-msvc`

### How to compile for Windows
`cargo build --release --target i686-pc-windows-msvc`

This will require vstools is downloaded. Check out [here](https://docs.microsoft.com/en-us/cpp/build/building-on-the-command-line?view=vs-2019)
