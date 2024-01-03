# Rompiler

The R stands for both Rust and Racket because I am a wordsmith at heart.

## Usage

The compiler binary is called `compiler_bin`. Locally, it is run with
```sh
cargo run --bin compiler_bin -- [FILE_NAME]
```
This creates the executable file `a.out` in the current directory, which is run
with `./a.out`.

## Tests

To run tests, use the command

```sh
cargo test
```

Each test will create two files in `target/tests`: an assembly file
`[TESTNAME].asm` that contains test functions at labels `f0`..`fn` and a  C
file `[TESTNAME].c` that calls the functions and checks their results. It
then links the files and runs the binary.

## Data Storage

All data is boxed on the heap (using C `malloc`) and takes exactly 9 bytes
(subject to change, doubles maybe). The first byte is the type. The next 8
bytes are the data.

| First Byte | Type  |
+------------+-------+
| 00         | int   |
| 01         | float |

Lists are stored as linked lists. The first 8 bytes of the cons cell are the
adress of the boxed data. The last 8 bytes are the address of the next cons
cell, or 0 for the last item

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

## TODO

- [x] Floating point numbers (do we have some sort of box datatype that points to the data or so we represent all numbers as floats?)
- [x] Lists
- [x] Local variables
- [ ] Global variables
- [ ] Functions
- [ ] Garbage collection?
