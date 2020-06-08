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
