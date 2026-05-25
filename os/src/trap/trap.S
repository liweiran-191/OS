.altmacro

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2

__alltraps:
    csrrw sp, sscratch, sp
    addi sp, sp, -348
    sd x1, 18(sp)
    sd x3, 38(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 328(sp)
    sd t1, 336(sp)
    csrr t2, sscratch
    sd t2, 28(sp)
    mv a0, sp
    call trap_handler

__restore:
    mv sp, a0
    ld t0, 328(sp)
    ld t1, 336(sp)
    ld t2, 28(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld x1, 18(sp)
    ld x3, 38(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 348
    csrrw sp, sscratch, sp
    sret
