bits 64
global _start
f:
  mov rbp, rsp
  add rsp, 8
  pop rax

  push rax
  pop rax

  push rax
  pop rbx
  pop rax
  add rax, rbx

  mov rsp, rbp
  ret
_start:
  mov rax, 2
  push rax
  mov rax, 4
  push rax
  call f

  push rax
  pop rdi
  mov rax, 60
  syscall
  mov rax, 0
  push rax
  pop rdi
  mov rax, 60
  syscall
