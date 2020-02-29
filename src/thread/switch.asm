.equ XLENB, 8

.macro LOAD reg idx
    ld \reg, \idx * XLENB(sp)
.endm

.macro STORE reg idx
    sd \reg, \idx * XLENB(sp)
.endm

.macro STORE_ALL
    STORE ra, 0
    # store satp later
    STORE s0, 2
    STORE s1, 3
    STORE s2, 4
    STORE s3, 5
    STORE s4, 6
    STORE s5, 7
    STORE s6, 8
    STORE s7, 9
    STORE s8, 10
    STORE s9, 11
    STORE s10, 12
    STORE s11, 13
    csrr s11, satp
    STORE s11, 1
.endm

.macro LOAD_ALL
    # first switch to the target thread's address space
    LOAD s11, 1
    csrw satp, s11
    # flush the TLB
    sfence.vma
    LOAD s11, 13
    LOAD s10, 12
    LOAD s9, 11
    LOAD s8, 10
    LOAD s7, 9
    LOAD s6, 8
    LOAD s5, 7
    LOAD s4, 6
    LOAD s3, 5
    LOAD s2, 4
    LOAD s1, 3
    LOAD s0, 2
    # satp already load
    LOAD ra, 0
.endm
    # reserve space for ContextContent (except the trap frame)
    addi sp, sp, -14 * XLENB
    # update the addr field of current context
    # old context contains old sp
    sd sp, 0(a0)
    STORE_ALL
    # switch to the target context
    # switch the stack first
    # a1 points to the new target's context struct,
    # which contains the new target's stack pointer
    ld sp, 0(a1)
    LOAD_ALL
    # target thread became the `current` thread now
    # pop the stack
    addi sp, sp, 14 * XLENB
    # set the addr field in target's context to 0
    # use addr == 0 as a marker that the thread is running
    sd zero, 0(a1)
    ret
