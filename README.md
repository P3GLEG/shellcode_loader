## How to generate hello world shellcode for linux
```bash
nasm -f elf64 -o hello.o hello.asm
ld -o  hello hello.o
objdump -M intel -d hello
```

### How to compile for linux
```bash
rustup target add x86_64-unknown-linux-musl
brew install FiloSottile/musl-cross/musl-cross
cargo build --release --target x86_64-unknown-linux-musl
```

### How to compile for mac
`cargo build -v --release --target x86_64-apple-darwin`


### How to compile on Windows
`cargo build --release --target i686-pc-windows-msvc`

### How to compile for Windows
`cargo build --release --target i686-pc-windows-msvc`

This will require vstools is downloaded. Check out [here](https://docs.microsoft.com/en-us/cpp/build/building-on-the-command-line?view=vs-2019)
