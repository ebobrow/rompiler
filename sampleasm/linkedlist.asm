global main
extern printf

section .text
main:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 80    ; custom heap :)

    push    rax        ; align stack
    mov     rax, 1
    push    rax
    mov     rax, 2
    push    rax
    mov     rax, 3
    push    rax
    mov     rdi, 3
    call    list
    pop     rbx
    pop     rbx
    pop     rbx
    pop     rbx
    mov     rdi, rax
    call    printlist
    ; call    empty      ; empty list in rax
    ; mov     rsi, rax
    ; mov     rdi, 1
    ; call    cons       ; cons 1 onto empty list
    ; mov     rsi, rax
    ; mov     rdi, 2
    ; call    cons       ; cons 2 onto empty list
    ; push    rax        ; save list
    ; mov     rdi, rax
    ; call    first
    ; mov     rdi, format
    ; mov     rsi, rax
    ; xor     rax, rax
    ; call    printf
    ; pop     rax
    ; mov     rdi, rax
    ; call    rest
    ; mov     rdi, rax
    ; call    first
    ; mov     rdi, format
    ; mov     rsi, rax
    ; xor     rax, rax
    ; call    printf

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
;   Returns list in rax
cons:
    mov     rax,   rbp    ; store heap address in rax to return
    mov     [rbp], rdi    ; store `first` on the heap
    sub     rbp,   8
    mov     [rbp], rsi    ; store `rest` on the heap
    sub     rbp,   8
    ret

; IsEmpty
;   Arguments: list in rdi
;   Returns 1 in rax if the list is empty, 0 if not
isempty:
    mov     rax, 1      ; return true by default
    cmp     rdi, 0
    je      end_isempty
    mov     rax, 0
end_isempty:
    ret

; First
;   Arguments: list in rdi
;   Returns: first element of the list in rax
first:
    mov     rax, [rdi]
    ret

; Rest
;   Arguments: list in rdi
;   Returns: the rest of the list in rax
rest:
    sub     rdi, 8
    mov     rax, [rdi]
    ret

; PrintList
;   Arguments: list in rdi
;   Returns nothing
;   Prints the list one element at a time, assuming all are integers
printlist:
    push    rbx
    call    first       ; put first element in rax
    mov     rbx, rdi    ; Store list in rbx
    mov     rdi, format ; move format string to rdi
    mov     rsi, rax    ; move first element to rsi
    xor     eax, eax    ; zero out al
    call    printf
    mov     rdi, rbx    ; move array into rdi
    call    rest        ; store rest in rax
    pop     rbx
    cmp     rax, 0
    je      end_printlist
    mov     rdi, rax
    jmp     printlist
end_printlist:
    ret

; List
;   Arguments: rdi contains the number of arguments and the rest of the
;              arguments are ~stored in registers/the stack as outlined in the
;              ABI calling conventions~ stored on the stack
;   Returns: list in rax
list:
    push    rbx
    call    empty
    mov     rsi, rax               ; accumulate list in rsi
    mov     rbx, rdi               ; Store list length in rbx
loop_list:
    mov     rdi, [rsp + rbx*8 + 8]
    call    cons                   ; Cons element onto list
    dec     rbx                    ; Decrement counter
    jz      end_list
    mov     rsi, rax
    jmp     loop_list
end_list:
    pop     rbx
    ret

; Append: TODO
append:

section .data
format: db "%d", 10, 0
