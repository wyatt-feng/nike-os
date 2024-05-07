.global _start                      # entry point
.section .text                      # code section
_start:
    addi a0, zero, 1
    addi a1, zero, 1
    la a2, message
    la a3, message_end
    sub a3, a3, a2
    ecall
    addi a0, zero, 60
    addi a1, zero, 1
    ecall

.section .rodata                    # read only data section
message:
    .ascii  "Hello, world\n"
message_end:
