
    .section .text 
    .syntax unified
    .cpu cortex-r4
    .arm 

    .weak _cpu_stack
    .type _cpu_stack, %function

_cpu_stack:
    mov   r0, lr
    cps   #17
    ldr   sp,   fiq_sp
    cps   #18
    ldr   sp,   irq_sp
    cps   #19
    ldr   sp,   svc_sp
    cps   #23
    ldr   sp,   abort_sp
    cps   #27
    ldr   sp,   undef_sp
    cps   #31
    ldr   sp,   user_sp
    bx    r0

user_sp:  .word USER_SP
svc_sp:   .word SVC_SP
fiq_sp:   .word FIQ_SP
irq_sp:   .word IRQ_SP
abort_sp: .word ABORT_SP
undef_sp: .word UNDEF_SP

