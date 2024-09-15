section .data
section .text
global _start
_start:
    call hi
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall

main:
    ret

global say
    ret

hi:
    ret
