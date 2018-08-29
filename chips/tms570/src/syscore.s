    .section .text
    .syntax unified
    .cpu cortex-r4
    .arm

    .weak _init_core_registers
    .weak _init_stack_pointers
    .weak _flash_ecc_enable
    .weak _flash_ecc_disable
    .weak _irq_vic_enable
    .weak _vfp_enable
    .weak _event_bus_export_enable
    .weak _event_bus_export_disable
    .weak _ram_ecc_enable
    .weak _ram_ecc_disable
    .type _init_core_registers, %function
    .type _init_stack_pointers, %function
    .type _flash_ecc_enable, %function
    .type _flash_ecc_disable, %function
    .type _irq_vic_enable, %function
    .type _vfp_enable, %function
    .type _event_bus_export_enable, %function
    .type _event_bus_export_disable, %function
    .type _ram_ecc_enable, %function
    .type _ram_ecc_disable, %function

_init_core_registers:
    /* After reset, the CPU is in the Supervisor mode (M = 10011) */
    mov r0, lr
    mov r1, #0x0000
    mov r2, #0x0000
    mov r3, #0x0000
    mov r4, #0x0000
    mov r5, #0x0000
    mov r6, #0x0000
    mov r7, #0x0000
    mov r8, #0x0000
    mov r9, #0x0000
    mov r10, #0x0000
    mov r11, #0x0000
    mov r12, #0x0000
    mov r13, #0x0000
    mrs r1, cpsr
    msr spsr_cxsf, r1

    /* Switch to FIQ mode (M = 10001) */
    cps #17
    mov lr, r0
    mov r8, #0x0000
    mov r9, #0x0000
    mov r10, #0x0000
    mov r11, #0x0000
    mov r12, #0x0000
    mrs r1, cpsr
    msr spsr_cxsf, r1

    /* Switch to IRQ mode (M = 10010) */
    cps #18
    mov lr, r0
    mrs r1,cpsr
    msr spsr_cxsf, r1

    /* Switch to Abort mode (M = 10111) */
    cps #23
    mov lr, r0
    mrs r1,cpsr
    msr spsr_cxsf, r1

    /* Switch to Undefined Instruction Mode (M = 11011) */
    cps #27
    mov lr, r0
    mrs r1,cpsr
    msr spsr_cxsf, r1

    /* Switch to System Mode (Shares User Mode registers) (M = 11111) */
    cps #31
    mov lr, r0
    mrs r1,cpsr
    msr spsr_cxsf, r1

    mrc   p15,     #0x00,      r2,       c1, c0, #0x02
    orr   r2,      r2,         #0xF00000
    mcr   p15,     #0x00,      r2,       c1, c0, #0x02
    mov   r2,      #0x40000000

#if defined(INIT_VFP_REGISTER)
    /* zeroize VFP registers */
    fmxr  fpexc,   r2
    fmdrr d0, r1, r1
    fmdrr d1, r1, r1
    fmdrr d2, r1, r1
    fmdrr d3, r1, r1
    fmdrr d4, r1, r1
    fmdrr d5, r1, r1
    fmdrr d6, r1, r1
    fmdrr d7, r1, r1
    fmdrr d8, r1, r1
    fmdrr d9, r1, r1
    fmdrr d10, r1, r1
    fmdrr d11, r1, r1
    fmdrr d12, r1, r1
    fmdrr d13, r1, r1
    fmdrr d14, r1, r1
    fmdrr d15, r1, r1
#endif

    bl    1f
1:  bl    2f
2:  bl    3f
3:  bl    4f
4:  bx r0

_init_stack_pointers:
     cps   #17
     ldr   sp, =0x08001200 /* fiq */
     cps   #18
     ldr   sp, =0x08001300 /* irq */
     cps   #19
     ldr   sp, =0x08001100 /* svc */
     cps   #23
     ldr   sp, =0x08001400 /* abort */
     cps   #27
     ldr   sp, =0x08001500 /* undef */
     cps   #31
     ldr   sp, =0x08001000 /* user */
     bx lr

_flash_ecc_enable:
    mrc p15, #0x00, r0, c1, c0, #0x01
    orr r0, r0, #0x02000000
    mcr p15, #0x00, r0, c1, c0, #0x01
    bx  lr

_flash_ecc_disable:
    mrc p15, #0x00, r0, c1, c0, #0x01
    bic r0, r0, #0x02000000
    mcr p15, #0x00, r0, c1, c0, #0x01
    bx  lr

_irq_vic_enable:
    mrc p15, #0, r0, c1, c0, #0;
    orr r0, r0, #0x01000000;
    mcr p15, #0, r0, c1, c0, #0
    bx  lr

_vfp_enable:
    mrc  p15, #0x00, r0, c1, c0, #0x02
    orr  r0,  r0, #0xF00000
    mcr  p15, #0x00, r0, c1, c0, #0x02
    mov  r0,  #0x40000000
    fmxr fpexc, r0
    bx   lr

_event_bus_export_enable:
    mrc p15, #0x00, r0, c9, c12, #0x00
    orr r0, r0, #0x10
    mcr p15, #0x00, r0, c9, c12, #0x00
    bx  lr

_event_bus_export_disable:
    mrc p15, #0x00, r0, c9, c12, #0x00
    bic r0, r0, #0x10
    mcr p15, #0x00, r0, c9, c12, #0x00
    bx  lr

_ram_ecc_enable:
    mrc p15, #0x00, r0, c1, c0, #0x01
    orr r0, r0, #0x0C000000
    mcr p15, #0x00, r0, c1, c0, #0x01
    bx  lr

_ram_ecc_disable:
    mrc p15, #0x00, r0, c1, c0, #0x01
    bic r0, r0, #0x0C000000
    mcr p15, #0x00, r0, c1, c0, #0x01
    bx  lr
