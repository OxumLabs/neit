section .data
section .text
global _start
_start:
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall

say:
    ; String argument (text) in rdi
    ret

sayy:
    ret
