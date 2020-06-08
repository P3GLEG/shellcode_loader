## How to generate hello world shellcode for linux
```bash
nasm -f elf64 -o hello.o hello.asm
ld -o  hello hello.o
objdump -M intel -d hello
```

### How to compile for linux
`cargo build --release --target x86_64-unknown-linux-gnu`

### How to compile for mac
`cargo build -v --release --target x86_64-apple-darwin`
