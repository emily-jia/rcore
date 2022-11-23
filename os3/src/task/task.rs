//! Types related to task management

use super::TaskContext;

pub const USED_SYSCALL_NUM: usize = 5;
pub const USED_SYSCALL_IDS: [usize; USED_SYSCALL_NUM] = [64, 93, 124, 169, 410]; 

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub time: usize,
    pub syscall_times: [u32; USED_SYSCALL_NUM]
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
