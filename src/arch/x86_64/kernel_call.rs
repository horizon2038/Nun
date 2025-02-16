#[inline(always)]
fn x86_64_capability_call(
    descriptor: u64,
    operation_type: u64,
    arg2: &mut u64,
    arg3: &mut u64,
    arg4: &mut u64,
    arg5: &mut u64,
    arg6: &mut u64,
    arg7: &mut u64,
    arg8: &mut u64,
) -> Result<(), CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = operation_type;

    let mut a2 = *arg2;
    let mut a3 = *arg3;
    let mut a4 = *arg4;
    let mut a5 = *arg5;
    let mut a6 = *arg6;
    let mut a7 = *arg7;
    let mut a8 = *arg8;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as isize,
        inout("rsi") a0 => a0,
        inout("rdx") a1 => a1,
        inout("r8")  a2 => a2,
        inout("r9")  a3 => a3,
        inout("r10") a4 => a4,
        inout("r12") a5 => a5,
        inout("r13") a6 => a6,
        inout("r14") a7 => a7,
        inout("r15") a8 => a8,
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    // Debug prints to understand post-syscall state
    // println!("Debug: a0 = {:#x}", a0);
    // println!("Debug: a1 = {:#x}", a1);
    // println!("Debug: a2 = {:#x}", a2);
    // println!("Debug: a3 = {:#x}", a3);

    *arg2 = a2;
    *arg3 = a3;
    *arg4 = a4;
    *arg5 = a5;
    *arg6 = a6;
    *arg7 = a7;
    *arg8 = a8;

    if a0 == 0 {
        return Err(match a1 {
            0 => CapabilityError::IllegalOperation,
            1 => CapabilityError::PermissionDenied,
            2 => CapabilityError::InvalidDescriptor,
            3 => CapabilityError::InvalidDepth,
            4 => CapabilityError::InvalidArgument,
            5 => CapabilityError::Fatal,
            6 => CapabilityError::DebugUnimplemented,
            _ => CapabilityError::DebugUnimplemented,
        });
    } else {
        return Ok(());
    }
}
