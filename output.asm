
section .text
global _start

_start:
    mov rax, 60   ; syscall: exit
    mov rdi, 2    ; exit code: 2
    syscall
