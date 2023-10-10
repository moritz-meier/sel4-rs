use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn stpcpy(dest: *mut u8, source: *const u8) -> *mut u8 {
    for i in 0.. {
        *dest.offset(i) = *source.offset(i);
        if *dest.offset(i) == 0 {
            break;
        }
    }

    dest
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut u8, source: *const u8) -> *mut u8 {
    stpcpy(dest, source);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn __assert_fail(
    _mstr: *const u8,
    _file: *const u8,
    _line: usize,
    _function: *const u8,
) {
    panic!("assertion failed");
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn __aeabi_read_tp() {
    asm!(
        "push {{r1,r2,r3,lr}}",
        "bl {__aeabi_read_tp_c}",
        "pop {{r1,r2,r3,lr}}",
        "bx lr",
        __aeabi_read_tp_c = sym __aeabi_read_tp_c,
        options(noreturn)
    )
}

fn __aeabi_read_tp_c() -> *const () {
    get_tls_base() as *const ()
}

fn get_tls_base() -> usize {
    read_tpidr_el0()
}

fn read_tpidr_el0() -> usize {
    let reg;
    unsafe { asm!("mcr p15, 0, {}, c13, c0, 2", lateout(reg) reg) }
    reg
}
