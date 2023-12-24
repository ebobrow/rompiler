# Rompiler

The R stands for both Rust and Racket because I am a wordsmith at heart.

## Usage

blah blah blah

## Tests

To run tests, use the command

```sh
cargo test
```

It will create two files in `target/tests`: an assembly file `a.asm` that
contains test functions at labels `f0`..`fn` and a  C file `test.c` that calls
the functions and checks their results. It then links the files and runs the
binary.

## TODO

- [ ] Floating point numbers (do we have some sort of box datatype that points to the data or so we represent all numbers as floats?)
- [ ] Lists
- [ ] Local variables
- [ ] Global variables
- [ ] Functions

## x86_64 Assembly Language

blah blah blah

### [AMD64 ABI Calling Conventions](https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI)

Integer arguments are passed in RDI, RSI, RDX, RCX, R8, R9.

Floating point arguments are passed in XMM0, XMM1, XMM2, XMM3, XMM4, XMM5, XMM6, XMM7.

Subsequent arguments are passed on the stack.

The stack pointer must be 16-byte aligned when making the call. Making the call pushes the 8-byte return address.

Integer values are returned in RAX and RDX, if more space is required.

Floating point values are returned in XMM0 and XMM1.

RBX, RSP, RBP, and R12–R15 are left untouched by the callee.

For leaf functions, local variables are stored in the 128-byte red zone beneath the stack pointer. Non-leaf functions adjust the stack pointer and use RBP as normal.
