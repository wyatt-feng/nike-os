#![no_std]
#![forbid(unsafe_code)]

mod sched;

extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;

use aster_frame::prelude::*;
use aster_frame::task::{add_task, schedule, Task, TaskOptions};

pub fn idle() {
    loop {
        schedule();
    }
}

pub fn greeting() {
    println!("[Kernel] Greeting");
} 

#[aster_main]
pub fn kern() {
    let scheduler = Box::leak(Box::new(sched::MySched::new()));
    aster_frame::task::set_scheduler(scheduler);
    let kernel_task_1: Arc<Task> = TaskOptions::new(idle)
        .data(0)
        .build()
        .unwrap();
    let kernel_task_2: Arc<Task> = TaskOptions::new(greeting)
        .data(0)
        .build()
        .unwrap();
    add_task(kernel_task_1.clone());
    add_task(kernel_task_2.clone());
    schedule();
    loop {}
}
