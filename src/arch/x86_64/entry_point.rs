pub use core::arch::asm;

#[macro_export]
macro_rules! arch_entry {
    ($path:path) => {
        use core::arch::asm;
        use core::arch::global_asm;

        // architecture-dependent initialization
        global_asm!(
            r#"
        .section .text, "ax"
        .global _start
        _start:
            lea rsp, [__init_stack_end]
            lea rdi, [__init_info_start]
            call {entry}
        1:
            hlt
            jmp 1b
    "#,
        entry =  sym $path
        );
    };
}
