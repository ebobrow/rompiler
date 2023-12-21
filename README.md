# Rompiler

The R stands for both Rust and Racket because I am a wordsmith at heart.

## Usage

blah blah blah

## Tests

Testing is done in the `test.sh` script. The `run_test` function is called
where the first argument is a Racket string and the second argument is the
expected output. To run the test suite, run `./test.sh`.

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
