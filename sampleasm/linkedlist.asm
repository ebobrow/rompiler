global main

section .text
main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 80    ; custom heap :)

    call    empty      ; empty list in rax
    mov     rsi, rax
    mov     rdi, 1
    call    cons       ; cons 1 onto empty list

    pop     rbp
    add     rsp, 80
    xor     rax, rax
    ret

; Empty
;   Takes no arguments; returns an empty list
;   Only modifies rax
empty:
    mov     rax, 0
    ret

; Cons
;   Arguments: `first` in rdi, `rest` in rsi
cons:
    mov     rax,   rbp    ; store heap address in rax to return
    mov     [rbp], rdi    ; store `first` on the heap
    sub     rbp,   8
    mov     [rbp], rsi    ; store `rest` on the heap
    sub     rbp,   8
    ret
