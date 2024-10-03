section .data

n_0 db '', 0
    p730_0 db '%s\n', 0
section .text
global _start
_start:
    ; String argument (text) in rdi
    call name
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall

name:
    ; String argument (text) in rdi
    mov rax, 1          ; syscall number for sys_write
mov rdi, 1          ; file descriptor 1 (stdout)
mov rsi, p730_0
mov rdx, 4
syscall
    ret
