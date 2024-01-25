; All values are boxed in Racket.
;
;  ------       ------------
; | reg | ---> | ty | data |
; ------       ------------

extern malloc

section .text
; NewInt
;   Arguments: integer in rdi
;   Returns pointer in rax
newint:
    push    rbx
    mov     rbx, rdi
    mov     rdi, 9
    call    malloc
    mov     [rax+3], byte 0 ; store type
    mov     [rax-1], rbx    ; store data
    pop     rbx
    ret

; NewFloat
;   Arguments: float in xmm0
;   Returns pointer in rax
newfloat:
    push    rbx
    sub     rsp, 16
    movss   [rsp], xmm0
    mov     rbx, rdi
    mov     rdi, 9
    call    malloc
    movss   xmm0, [rsp]
    add     rsp, 16
    mov     [rax+3], byte 1 ; store type
    movss   [rax-1], xmm0   ; store data
    pop     rbx
    ret

; NewIP
;   Arguments: line number in rdi
;   Returns ip in rax
newip:
    push    rbx
    mov     rbx, rdi
    mov     rdi, 9
    call    malloc
    mov     [rax+3], byte 2 ; store type
    mov     [rax-1], rbx    ; store data
    pop     rbx
    ret

; GetInt
;   Arguments: boxed int in rdi
;   Returns: value in rax
getint:
    mov     rax, [rdi-1]
    ret

; GetFloat
;   Arguments: boxed float in rdi
;   Returns float value in xmm0
getfloat:
    movss   xmm0, [rdi-1]
    ret

; GetIP
;   Arguments: boxed float in rdi
;   Returns float value in xmm0
getip:
    mov     rax, [rdi-1]
    ret

; Eq
;   Arguments: boxed values in rdi and rsi
;   Returns 1 if the values are equal, 0 if not
eq:
    mov     al, byte [rdi+3]
    cmp     al, 0
    je      eqint1
    mov     al, byte [rsi+3]
    cmp     al, 0
    je      neq
    movss   xmm0, [rdi-1]
    movss   xmm1, [rsi-1]
    ucomiss xmm0, xmm1
    je      yeq
    jmp     neq
eqint1:
    mov     al, byte [rsi+3]
    cmp     al, 0
    jne     neq
    mov     rax, [rdi-1]
    cmp     rax, [rsi-1]
    je      yeq
    jmp     neq
neq:
    mov     rax, 0
    ret
yeq:
    mov     rax, 1
    ret

; MAdd
;   Arguments: two boxed values in rdi and rsi
;   Returns boxed value in rax
;   If one of the boxed values is a float, it casts them both to floats and
;   adds as float. Otherwise, they're both ints and it adds as signed ints.
madd:
    mov         al, byte [rdi+3]
    cmp         al, 0
    je          int1
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          addfloatint
    movss       xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
    jmp         addfloatfloat
int1:
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          addintint
    jmp         addintfloat
addfloatint:
    movss       xmm0, [rdi-1]
    cvtsi2ss    xmm1, [rsi-1]
    jmp         addfloatfloat
addintfloat:
    cvtsi2ss    xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
addfloatfloat:
    addss       xmm0, xmm1
    call        newfloat
    ret
addintint:
    mov         rdi, [rdi-1]
    mov         rsi, [rsi-1]
    add         rdi, rsi
    call        newint
    ret

; MSub
;   Arguments: two boxed values in rdi and rsi
;   Returns boxed value in rax
msub:
    mov         al, byte [rdi+3]
    cmp         al, 0
    je          subint1
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          subfloatint
    movss       xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
    jmp         subfloatfloat
subint1:
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          subintint
    jmp         subintfloat
subfloatint:
    movss       xmm0, [rdi-1]
    cvtsi2ss    xmm1, [rsi-1]
    jmp         subfloatfloat
subintfloat:
    cvtsi2ss    xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
subfloatfloat:
    subss       xmm0, xmm1
    call        newfloat
    ret
subintint:
    mov         rdi, [rdi-1]
    mov         rsi, [rsi-1]
    sub         rdi, rsi
    call        newint
    ret

; MMul
;   Arguments: two boxed values in rdi and rsi
;   Returns boxed value in rax
mmul:
    mov         al, byte [rdi+3]
    cmp         al, 0
    je          mulint1
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          mulfloatint
    movss       xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
    jmp         mulfloatfloat
mulint1:
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          mulintint
    jmp         mulintfloat
mulfloatint:
    movss       xmm0, [rdi-1]
    cvtsi2ss    xmm1, [rsi-1]
    jmp         mulfloatfloat
mulintfloat:
    cvtsi2ss    xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
mulfloatfloat:
    mulss       xmm0, xmm1
    call        newfloat
    ret
mulintint:
    mov         rdi, [rdi-1]
    mov         rsi, [rsi-1]
    imul        rdi, rsi
    call        newint
    ret

; Mdiv
;   Arguments: two boxed values in rdi and rsi
;   Returns boxed value in rax
mdiv:
    mov         al, byte [rdi+3]
    cmp         al, 0
    je          divint1
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          divfloatint
    movss       xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
    jmp         divfloatfloat
divint1:
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          divintint
    jmp         divintfloat
divfloatint:
    movss       xmm0, [rdi-1]
    cvtsi2ss    xmm1, [rsi-1]
    jmp         divfloatfloat
divintfloat:
    cvtsi2ss    xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
divfloatfloat:
    divss       xmm0, xmm1
    call        newfloat
    ret
divintint:
    push        rbx
    push        rdx
    xor         rdx, rdx
    mov         rax, [rdi-1]
    mov         rbx, [rsi-1]
    idiv        rbx
    mov         rdi, rax
    call        newint
    pop         rdx
    pop         rbx
    ret

; MMod
;   Arguments: two boxed values in rdi and rsi
;   Returns boxed value in rax
mmod:
    mov         al, byte [rdi+3]
    cmp         al, 0
    je          modint1
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          modfloatint
    movss       xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
    jmp         modfloatfloat
modint1:
    mov         al, byte [rsi+3]
    cmp         al, 0
    je          modintint
    jmp         modintfloat
modfloatint:
    movss       xmm0, [rdi-1]
    cvtsi2ss    xmm1, [rsi-1]
    jmp         modfloatfloat
modintfloat:
    cvtsi2ss    xmm0, [rdi-1]
    movss       xmm1, [rsi-1]
modfloatfloat:
    divss       xmm0, xmm1
    call        newfloat
    ret
modintint:
    push        rbx
    push        rdx
    xor         rdx, rdx
    mov         rax, [rdi-1]
    mov         rbx, [rsi-1]
    idiv        rbx
    mov         rdi, rdx
    call        newint
    pop         rdx
    pop         rbx
    ret
