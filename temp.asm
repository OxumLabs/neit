section .data
    p1094 db 'hello'
    p2554 db ' we are on same line XD',0xA,''
    p1 db 'Yo bro wassup! how are you!',0xA,''
section .text
global _start
_start:
    call hello
    mov rax, 1
    mov rdi, 1
    mov rsi, p1
    mov rdx, 34
    syscall
    mov rax, 60         ; syscall number for exit (sys_exit)
    mov rdi, 0          ; status code 0
    syscall             ; invoke syscall

hello:
    mov rax, 1
    mov rdi, 1
    mov rsi, p1094
    mov rdx, 5
    syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, p2554
    mov rdx, 30
    syscall
    ret
