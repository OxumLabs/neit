section .data

name_0 db '', 0
    p1_5 db 'name~s', 0
section .text
global _start
_start:
    mov rax, 1          ; syscall number for sys_write
mov rdi, 1          ; file descriptor 1 (stdout)
mov rsi, p1_5
mov rdx, 6
syscall
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall
