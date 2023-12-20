global main
extern printf
section .data
format: db "%d", 10, 0
section .text
main:mov RAX, 2
mov R11, 3
mul R11
mov RSI, RAX
mov R11, 1
add RSI, R11
mov rdi, format
xor eax, eax
push rax
call printf
pop rax
mov rax, 0
ret