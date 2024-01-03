extern malloc

section .text
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
    push    rbx
    push    rsi
    push    rsi             ; extra push to align stack pointer
    mov     rbx,     rdi
    mov     rdi,     16
    call    malloc
    pop     rsi
    pop     rsi
    mov     [rax],   rbx    ; store `first` on the heap
    mov     [rax-8], rsi    ; store `rest` on the heap
    pop     rbx
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
    mov     rax, [rdi-8]
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

; Append:
;   Arguments: list in rdi, list in rsi
;   Returns: combined list in rax
append:
    push    rdi          ; save list
append_loop:
    mov     rbx, rdi     ; save list
    call    rest         ; put rest in rax
    mov     rdi, rax     ; put rest in rdi
    cmp     rax, 0
    jne     append_loop
    mov     [rbx-8], rsi ; put list 2 after list 1
    pop     rdi
    mov     rax, rdi

section .data
format: db "%d", 10, 0
