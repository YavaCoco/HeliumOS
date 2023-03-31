global as_lgdt
global as_sgdt

section .text

as_lgdt:
  lgdt [rdi]
  mov rax, rsi
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov fs, ax
  mov gs, ax

  ; Loading CS with the correct value will be tricky since the value is variable
  ; First, we pop the return address from the stack into rdi
  pop rdi
  ; Next we push into the stack the new value of CS
  push rdx
  ; Then we re-push the return address
  push rdi
  ; Now we do a far return, I hope this works first try(it did third try)
  retfq

as_sgdt:
  sgdt [rdi]
  ret
