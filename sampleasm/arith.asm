global main

section .text
main:
    mov     rdx, 0
    mov     rax, 5
    mov     rbx, 2
    div     rbx
    ret
