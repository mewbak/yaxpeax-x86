extern crate yaxpeax_arch;
extern crate yaxpeax_x86;

use std::fmt::Write;

use yaxpeax_arch::Decodable;
use yaxpeax_x86::{Instruction, decode_one};

fn decode(bytes: &[u8]) -> Option<Instruction> {
    let mut instr = Instruction::invalid();
    match decode_one(bytes.iter().map(|x| *x).take(16).collect::<Vec<u8>>(), &mut instr) {
        Some(()) => Some(instr),
        None => None
    }
}

fn test_display(data: &[u8], expected: &'static str) {
    let mut hex = String::new();
    for b in data {
        write!(hex, "{:02x}", b).unwrap();
    }
    match Instruction::decode(data.into_iter().map(|x| *x)) {
        Some(instr) => {
            let text = format!("{}", instr);
            assert!(
                text == expected,
                "display error for {}:\n  decoded: {:?}\n displayed: {}\n expected: {}\n",
                hex,
                instr,
                text,
                expected
            );
        },
        None => {
            assert!(false, "decode error for {}:\n  expected: {}\n", hex, expected);
        }
    }
}

#[test]
fn test_system() {
    test_display(&[0x45, 0x0f, 0x22, 0xc8], "mov cr9, r8");
    test_display(&[0x45, 0x0f, 0x20, 0xc8], "mov r8, cr9");
    test_display(&[0x40, 0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x44, 0x0f, 0x22, 0xcf], "mov cr9, rdi");
    test_display(&[0x0f, 0x22, 0xcf], "mov cr1, rdi");
    test_display(&[0x0f, 0x20, 0xc8], "mov rax, cr1");

    test_display(&[0x45, 0x0f, 0x23, 0xc8], "mov dr9, r8");
    test_display(&[0x45, 0x0f, 0x21, 0xc8], "mov r8, dr9");
    test_display(&[0x40, 0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x0f, 0x21, 0xc8], "mov rax, dr1");
}

#[test]
fn test_arithmetic() {
    test_display(&[0x81, 0xec, 0x10, 0x03, 0x00, 0x00], "sub esp, 0x310");
    test_display(&[0x0f, 0xaf, 0xc2], "imul eax, edx");
}

#[test]
#[allow(non_snake_case)]
fn test_E_decode() {
    test_display(&[0xff, 0x75, 0xb8], "push [rbp - 0x48]");
    test_display(&[0xff, 0x75, 0x08], "push [rbp + 0x8]");
}

#[test]
fn test_sse() {
    test_display(&[0x0f, 0x28, 0xd0], "movaps xmm2, xmm0");
    test_display(&[0x66, 0x0f, 0x28, 0xd0], "movapd xmm2, xmm0");
    test_display(&[0x66, 0x0f, 0x28, 0x00], "movapd xmm0, xmmword [rax]");
    test_display(&[0x4f, 0x66, 0x0f, 0x28, 0x00], "movapd xmm0, xmmword [rax]");
    test_display(&[0x66, 0x4f, 0x0f, 0x28, 0x00], "movapd xmm8, xmmword [r8]");
    test_display(&[0x66, 0x4f, 0x0f, 0x28, 0x00], "movapd xmm8, xmmword [r8]");
    test_display(&[0x67, 0x4f, 0x66, 0x0f, 0x28, 0x00], "movapd xmm0, xmmword [eax]");
    test_display(&[0x67, 0x66, 0x4f, 0x0f, 0x28, 0x00], "movapd xmm8, xmmword [r8d]");
    test_display(&[0x66, 0x0f, 0x29, 0x00], "movapd xmmword [rax], xmm0");
    test_display(&[0x66, 0x0f, 0xef, 0xc0], "pxor xmm0, xmm0");
    test_display(&[0xf2, 0x0f, 0x10, 0x0c, 0xc6], "movsd xmm1, [rsi + rax * 8]");
    test_display(&[0xf3, 0x0f, 0x10, 0x04, 0x86], "movss xmm0, [rsi + rax * 4]");
    test_display(&[0xf2, 0x0f, 0x59, 0xc8], "mulsd xmm1, xmm0");
    test_display(&[0xf3, 0x0f, 0x59, 0xc8], "mulss xmm1, xmm0");
    test_display(&[0xf2, 0x4f, 0x0f, 0x59, 0xc8], "mulsd xmm9, xmm8");
    test_display(&[0xf2, 0x0f, 0x11, 0x0c, 0xc7], "movsd [rdi + rax * 8], xmm1");
}

// SETLE, SETNG, ...

#[test]
fn test_mov() {
    // test_display(&[0xa1, 0x93, 0x62, 0xc4, 0x00, 0x12, 0x34, 0x12, 0x34], "mov eax, [0x3412341200c46293]");
    // RCT.exe 32bit version, TODO: FIX
    test_display(&[0xa1, 0x93, 0x62, 0xc4, 0x00], "mov eax, [0xc46293]");
    test_display(&[0x48, 0xc7, 0x04, 0x24, 0x00, 0x00, 0x00, 0x00], "mov [rsp], 0x0");
    test_display(&[0x48, 0x89, 0x44, 0x24, 0x08], "mov [rsp + 0x8], rax");
    test_display(&[0x48, 0x89, 0x43, 0x18], "mov [rbx + 0x18], rax");
    test_display(&[0x48, 0xc7, 0x43, 0x10, 0x00, 0x00, 0x00, 0x00], "mov [rbx + 0x10], 0x0");
    test_display(&[0x49, 0x89, 0x4e, 0x08], "mov [r14 + 0x8], rcx");
    test_display(&[0x48, 0x8b, 0x32], "mov rsi, [rdx]");
    test_display(&[0x49, 0x89, 0x46, 0x10], "mov [r14 + 0x10], rax");
    test_display(&[0x4d, 0x0f, 0x43, 0xec, 0x49], "cmovnb r13, r12");
    test_display(&[0x0f, 0xb6, 0x06], "movzx eax, byte [rsi]");
    test_display(&[0x0f, 0xb7, 0x06], "movzx eax, word [rsi]");
    test_display(&[0x89, 0x55, 0x94], "mov [rbp - 0x6c], edx");
    test_display(&[0x65, 0x4c, 0x89, 0x04, 0x25, 0xa8, 0x01, 0x00, 0x00], "mov gs:[0x1a8], r8");
    test_display(&[0x0f, 0xbe, 0x83, 0xb4, 0x00, 0x00, 0x00], "movsx eax, byte [rbx + 0xb4]");
    test_display(&[0x48, 0x63, 0x04, 0xba], "movsxd rax, [rdx + rdi * 4]");
}

#[test]
fn test_stack() {
    test_display(&[0x66, 0x41, 0x50], "push r8w");
}

#[test]
fn test_prefixes() {
    test_display(&[0x66, 0x41, 0x31, 0xc0], "xor r8w, ax");
    test_display(&[0x66, 0x41, 0x32, 0xc0], "xor al, r8b");
    test_display(&[0x40, 0x32, 0xc5], "xor al, bpl");
}

#[test]
fn test_control_flow() {
    test_display(&[0x73, 0x31], "jnb 0x31");
    test_display(&[0x72, 0x5a], "jb 0x5a");
    test_display(&[0x0f, 0x86, 0x8b, 0x01, 0x00, 0x00], "jna 0x18b");
    test_display(&[0x74, 0x47], "jz 0x47");
    test_display(&[0xff, 0x15, 0x7e, 0x72, 0x24, 0x00], "call [rip + 0x24727e]");
    test_display(&[0xc3], "ret");
}

#[test]
fn test_test_cmp() {
    test_display(&[0x48, 0x3d, 0x01, 0xf0, 0xff, 0xff], "cmp rax, -0xfff");
    test_display(&[0x3d, 0x01, 0xf0, 0xff, 0xff], "cmp eax, -0xfff");
    test_display(&[0x48, 0x83, 0xf8, 0xff], "cmp rax, -0x1");
    test_display(&[0x48, 0x39, 0xc6], "cmp rsi, rax");
}

#[test]
#[ignore]
// VEX prefixes are not supported at the moment, in any form
fn test_avx() {
    test_display(&[0xc5, 0xf8, 0x10, 0x00], "vmovups xmm0, xmmword [rax]");
}

#[test]
fn test_push_pop() {
    test_display(&[0x5b], "pop rbx");
    test_display(&[0x41, 0x5e], "pop r14");
    test_display(&[0x68, 0x7f, 0x63, 0xc4, 0x00], "push 0xc4637f");
}

#[test]
fn test_bitwise() {
    test_display(&[0x41, 0x0f, 0xbc, 0xd3], "bsf edx, r11d");
    test_display(&[0x48, 0x0f, 0xa3, 0xd0], "bt rax, rdx");
    test_display(&[0x48, 0x0f, 0xab, 0xd0], "bts rax, rdx");
}

#[test]
fn test_misc() {
    test_display(&[0x9c], "pushf");
    test_display(&[0x48, 0x98], "cdqe");
    test_display(&[0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00], "nop cs:[rax + rax]");
    test_display(&[0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00], "nop cs:[rax + rax]");
    test_display(&[0x48, 0x8d, 0xa4, 0xc7, 0x20, 0x00, 0x00, 0x12], "lea rsp, [rdi + rax * 8 + 0x12000020]");
    test_display(&[0x33, 0xc0], "xor eax, eax");
    test_display(&[0x48, 0x8d, 0x53, 0x08], "lea rdx, [rbx + 0x8]");
    test_display(&[0x31, 0xc9], "xor ecx, ecx");
    test_display(&[0x48, 0x29, 0xc8], "sub rax, rcx");
    test_display(&[0x48, 0x03, 0x0b], "add rcx, [rbx]");
    test_display(&[0x48, 0x8d, 0x0c, 0x12], "lea rcx, [rdx + rdx]");
    test_display(&[0xf6, 0xc2, 0x18], "test dl, 0x18");
    test_display(&[0xf3, 0x48, 0xab], "rep stosq");
    test_display(&[0xf3, 0x48, 0xa5], "rep movsq");
    test_display(&[0xf3, 0x45, 0x0f, 0xbc, 0xd7], "tzcnt r10d, r15d");
}

#[test]
fn evex() {
    test_display(&[0x62, 0xf2, 0x7d, 0x48, 0x2a, 0x44, 0x40, 0x01], "vmovntdqa zmm0, zmmword [rax + rax*2 + 0x40]");
    test_display(&[0x62, 0xf2, 0x7d, 0x08, 0x2a, 0x44, 0x40, 0x01], "vmovntdqa xmm0, xmmword [rax + rax*2 + 0x10]");
}

#[test]
fn vex() {
}

#[test]
fn prefixed_0f() {
    test_display(&[0x0f, 0x02, 0xc0], "lar eax, eax");
    test_display(&[0x48, 0x0f, 0x02, 0xc0], "lar rax, eax");
    test_display(&[0x0f, 0x03, 0xc0], "lsl eax, eax");
    test_display(&[0x48, 0x0f, 0x03, 0xc0], "lsl rax, rax");
    test_display(&[0x0f, 0x05], "syscall");
    test_display(&[0x48, 0x0f, 0x05], "syscall");
    test_display(&[0x66, 0x0f, 0x05], "syscall");
    test_display(&[0x0f, 0x05], "sysret");
    test_display(&[0xf2, 0x0f, 0x05], "sysret");
    test_display(&[0x0f, 0x12, 0x0f], "movlps xmm1, qword [rdi]");
    test_display(&[0x0f, 0x12, 0xc0], "movhlps xmm0, xmm0");
    test_display(&[0x0f, 0x13, 0xc0], "invalid");
    test_display(&[0x0f, 0x14, 0x00], "unpcklps xmm1, xmmword [rax]");
    test_display(&[0x0f, 0x15, 0x00], "unpckhps xmm1, xmmword [rax]");
    test_display(&[0x0f, 0x16, 0x0f], "movhps xmm1, qword [rdi]");
    test_display(&[0x0f, 0x16, 0xc0], "movlhps xmm0, xmm0");
    test_display(&[0x0f, 0x17, 0xc0], "invalid");
    test_display(&[0x0f, 0x18, 0xc0], "invalid");
    test_display(&[0x0f, 0x18, 0x00], "prefetchnta byte [rax]");
    test_display(&[0x0f, 0x18, 0x08], "prefetch1 byte [rax]");
    test_display(&[0x0f, 0x18, 0x10], "prefetch2 byte [rax]");
    test_display(&[0x0f, 0x18, 0x18], "prefetch2 byte [rax]");
    test_display(&[0x0f, 0x18, 0x20], "nop dword [rax]");
    test_display(&[0x4f, 0x0f, 0x18, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x19, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1a, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1b, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1c, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1d, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1e, 0x20], "nop dword [rax]");
    test_display(&[0x0f, 0x1f, 0x20], "nop dword [rax]");
    test_display(&[0x45, 0x0f, 0x20, 0xc8], "mov r8, cr9");
    test_display(&[0x0f, 0x20, 0xc8], "mov rax, cr1");
    test_display(&[0x45, 0x0f, 0x21, 0xc8], "mov r8, dr9");
    test_display(&[0x0f, 0x21, 0xc8], "mov rax, dr1");
    test_display(&[0x45, 0x0f, 0x22, 0xc8], "mov cr9, r8");
    test_display(&[0x40, 0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x0f, 0x22, 0xc8], "mov cr1, rax");
    test_display(&[0x44, 0x0f, 0x22, 0xcf], "mov cr9, rdi");
    test_display(&[0x0f, 0x22, 0xcf], "mov cr1, rdi");
    test_display(&[0x45, 0x0f, 0x23, 0xc8], "mov dr9, r8");
    test_display(&[0x40, 0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x0f, 0x23, 0xc8], "mov dr1, rax");
    test_display(&[0x44, 0x0f, 0x23, 0xcf], "mov dr9, rdi");
    test_display(&[0x0f, 0x23, 0xcf], "mov dr1, rdi");
    test_display(&[0x0f, 0x30], "wrmsr");
    test_display(&[0x0f, 0x31], "rdtsc");
    test_display(&[0x0f, 0x32], "rdmsr");
    test_display(&[0x0f, 0x33], "rdpmc");
    test_display(&[0x0f, 0x34], "sysenter");
    test_display(&[0x0f, 0x35], "sysret");
    test_display(&[0x0f, 0x36], "invalid");
    test_display(&[0x0f, 0x37], "getsec");
    test_display(&[0x0f, 0x60, 0x00], "punpcklbw mm0, qword [rax]");
    test_display(&[0x0f, 0x61, 0x00], "punpcklwd mm0, qword [rax]");
    test_display(&[0x0f, 0x62, 0x00], "punpckldq mm0, qword [rax]");
    test_display(&[0x0f, 0x63, 0x00], "packsswb mm0, qword [rax]");
    test_display(&[0x0f, 0x64, 0x00], "pcmpgtb mm0, qword [rax]");
    test_display(&[0x0f, 0x65, 0x00], "pcmpgtw mm0, qword [rax]");
    test_display(&[0x0f, 0x66, 0x00], "pcmpgtd mm0, qword [rax]");
    test_display(&[0x0f, 0x67, 0x00], "packuswb mm0, qword [rax]");
    test_display(&[0x0f, 0x68, 0x00], "punpckhbw mm0, qword [rax]");
    test_display(&[0x0f, 0x69, 0x00], "punpckhbd mm0, qword [rax]");
    test_display(&[0x0f, 0x6a, 0x00], "punpckhdq mm0, qword [rax]");
    test_display(&[0x0f, 0x6b, 0x00], "packssdw mm0, qword [rax]");
    test_display(&[0x0f, 0x6c], "invalid");
    test_display(&[0x0f, 0x6d], "invalid");
    test_display(&[0x0f, 0x6e], "movd mm0, dword [rax]");
    test_display(&[0x0f, 0x6f], "movd mm0, qword [rax]");
    test_display(&[0x0f, 0x70, 0x00, 0x7f], "pshufw mm0, qword [rax], 0x7f");
    test_display(&[0x0f, 0x71, 0xd0, 0x7f], "psrlw mm0, 0x7f");
    test_display(&[0x0f, 0x71, 0xe0, 0x7f], "psraw mm0, 0x7f");
    test_display(&[0x0f, 0x71, 0xf0, 0x7f], "psllw mm0, 0x7f");
    test_display(&[0x0f, 0x72, 0xd0, 0x7f], "psrld mm0, 0x7f");
    test_display(&[0x0f, 0x72, 0xe0, 0x7f], "psrad mm0, 0x7f");
    test_display(&[0x0f, 0x72, 0xf0, 0x7f], "pslld mm0, 0x7f");
    test_display(&[0x0f, 0xa0], "push fs");
    test_display(&[0x0f, 0xa1], "pop fs");
    test_display(&[0x0f, 0xa2], "cpuid");
    test_display(&[0x0f, 0xa4, 0xc0, 0x11], "shld eax, eax, 0x11");
    test_display(&[0x0f, 0xa5, 0xc0], "shld eax, eax, cl");
    test_display(&[0x0f, 0xa5, 0xc9], "shld ecx, ecx, cl");
}

#[test]
fn prefixed_660f() {
    test_display(&[0x66, 0x0f, 0x10, 0xc0], "movupd xmm0, xmm0");
    test_display(&[0x66, 0x48, 0x0f, 0x10, 0xc0], "movupd xmm0, xmm0");
    test_display(&[0x66, 0x49, 0x0f, 0x10, 0xc0], "movupd xmm0, xmm8");
    test_display(&[0x66, 0x4a, 0x0f, 0x10, 0xc0], "movupd xmm0, xmm8");
    test_display(&[0x66, 0x4c, 0x0f, 0x10, 0xc0], "movupd xmm8, xmm0");
    test_display(&[0x66, 0x4d, 0x0f, 0x10, 0xc0], "movupd xmm8, xmm8");
    test_display(&[0xf2, 0x66, 0x66, 0x4d, 0x0f, 0x10, 0xc0], "movupd xmm8, xmm8");
}

#[test]
fn prefixed_f20f() {
    test_display(&[0xf2, 0x0f, 0x16, 0xcf], "movlhps xmm1, xmm7");
    test_display(&[0xf2, 0x4d, 0x0f, 0x16, 0xcf], "movlhps xmm9, xmm15");
    test_display(&[0x40, 0x66, 0xf2, 0x66, 0x4d, 0x0f, 0x16, 0xcf], "movlhps xmm9, xmm15");
}

#[test]
fn prefixed_f30f() {
    test_display(&[0xf3, 0x0f, 0x16, 0xcf], "movshdup xmm1, xmm7");
    test_display(&[0xf3, 0x4d, 0x0f, 0x16, 0xcf], "movshdup xmm9, xmm15");
}
