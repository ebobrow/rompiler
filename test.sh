#!/bin/sh

run_test() {
    cargo r -q --bin fromstr -- "$1"
    nasm -f elf64 a.asm
    gcc -no-pie a.o
    ./a.out
    out="$?"
    if [ "$out" = "$2" ]; then
        printf "\033[0;32m$1 = $2\033[0m\n"
    else
        printf "\033[0;31merror: $1; expected $2, got $out\033[0m\n"
    fi
}

run_test "(+ 1 (* 2 3))" "7"
run_test "(+ 40 2)" "42"
run_test "(- 2 1)" "1"
run_test "(* 2 21)" "42"
run_test "(/ 5 2)" "2"
run_test "(+ 1 (* 2 (- 3 4)))" "-"1
