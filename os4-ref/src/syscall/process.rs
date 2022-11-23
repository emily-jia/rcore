//! Process management syscalls

use core::mem::{size_of};

use crate::config::{MAX_SYSCALL_NUM, PAGE_SIZE};
use crate::mm::{translated_byte_buffer};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, read_cur_taskinfo, map, unmap};
use crate::task::{current_user_token};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

const SIZE_TIMEVAL: usize = size_of::<TimeVal>();

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

// YOUR JOB: 引入虚地址后重写 sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let cur_token = current_user_token();
    let vec = translated_byte_buffer(cur_token, _ts as *mut u8, SIZE_TIMEVAL);
    let dst = vec[0] as *const [u8];
    let time_dst = dst as *mut TimeVal;
    unsafe {
        *time_dst = val;
    }
    0
}

// CLUE: 从 ch4 开始不再对调度算法进行测试~
pub fn sys_set_priority(_prio: isize) -> isize {
    -1
}

// YOUR JOB: 扩展内核以实现 sys_mmap 和 sys_munmap
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    if _start % PAGE_SIZE != 0 {
        return -1; 
    }

    let st_va = _start;
    let ed_va = _start + _len;

    if map(st_va, ed_va, _port) { 0 } else { -1 }
}

pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    if _start % PAGE_SIZE != 0 {
        return -1; 
    }
    let st_va: usize = _start;
    let ed_va: usize = _start + _len;

    if unmap(st_va, ed_va) { 0 } else { -1 }
}

// YOUR JOB: 引入虚地址后重写 sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let (status, time, syscall_times) = read_cur_taskinfo();
    let info = TaskInfo {
        status: status,
        time: (get_time_us() - time) / 1000,
        syscall_times: syscall_times
    };

    let cur_token = current_user_token();
    let vec = translated_byte_buffer(cur_token, ti as *mut u8, SIZE_TIMEVAL);
    let dst = vec[0] as *const [u8];
    let real_dst = dst as *mut TaskInfo;
    unsafe {
        *real_dst = info;
    }
    0
}
