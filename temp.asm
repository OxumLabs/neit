section .data
    p1_0 db 'hello world', 0xA, '', 0
section .text
global _start
_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, p1_0
    mov rdx, 13
    syscall
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall
