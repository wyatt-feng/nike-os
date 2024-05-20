use alloc::{collections::VecDeque, sync::Arc};

use aster_frame::{
    sync::SpinLock,
    task::{Scheduler, Task},
};

pub struct MySched {
    queue: SpinLock<VecDeque<Arc<Task>>>,
}

impl Scheduler for MySched {
    fn enqueue(&self, task: Arc<Task>) {
        self.queue.lock().push_back(task);
    }

    fn dequeue(&self) -> Option<Arc<Task>> {
        self.queue.lock().pop_front()
    }

    fn should_preempt(&self, task: &Arc<Task>) -> bool {
        false
    }
}

impl MySched {
    pub fn new() -> MySched {
        MySched {
            queue: SpinLock::new(VecDeque::new()),
        }
    }
}
