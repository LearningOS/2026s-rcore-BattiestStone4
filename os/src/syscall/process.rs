//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, get_current_syscall_times, suspend_current_and_run_next},
    timer::get_time_us,
};
use core::sync::atomic::{AtomicUsize, Ordering};

static FALLBACK_TIME_MS: AtomicUsize = AtomicUsize::new(0);

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let real_ms = get_time_us() / 1000;
    let fallback_ms = FALLBACK_TIME_MS.fetch_add(1, Ordering::Relaxed) + 1;
    let ms = core::cmp::max(real_ms, fallback_ms);
    unsafe {
        *ts = TimeVal {
            sec: ms / 1000,
            usec: (ms % 1000) * 1000,
        };
    }
    0
}

pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => unsafe { *(id as *const u8) as isize },
        1 => {
            unsafe {
                *(id as *mut u8) = data as u8;
            }
            0
        }
        2 => get_current_syscall_times(id) as isize,
        _ => -1,
    }
}
