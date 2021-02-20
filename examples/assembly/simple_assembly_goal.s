global _start

section .text

_start:
	push rbp			;
	mov rbp,rsp			;
	mov dword [rbp-0x8],0x1		;
	mov dword [rbp-0x4],0x2		;
	mov eax,0x0			;
	pop rbp				;
	ret				;
