section .data

name_0 db 'joy', 0

i dq 0
section .text
global _start
_start:
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall
