global main

section .text
main:
    mov     rax, 0xffffffff
    mov     rbx, 0xffffffff
    mul     rbx
    ret
