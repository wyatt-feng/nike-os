#![no_std]
#![forbid(unsafe_code)]

mod sched;

extern crate alloc;
use alloc::{boxed::Box, sync::Arc};

use aster_frame::{
    prelude::*,
    task::{add_task, schedule, Task, TaskOptions},
};

pub fn greet() {
    println!("[Kernel] Greetings");
}

#[aster_main]
pub fn kern() {
    let scheduler = Box::leak(Box::new(sched::MySched::new()));
    aster_frame::task::set_scheduler(scheduler);
    let kern_task: Arc<Task> = TaskOptions::new(idle).data(0).build().unwrap();
    add_task(kern_task.clone());
    schedule();
    loop {}
}
