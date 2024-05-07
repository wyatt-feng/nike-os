#![no_std]

mod sched;

extern crate alloc;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec;

use align_ext::AlignExt;
use core::str;

use aster_frame::arch::qemu::{exit_qemu, QemuExitCode};
use aster_frame::cpu::UserContext;
use aster_frame::prelude::*;
use aster_frame::task::{Task, TaskOptions};
use aster_frame::user::{UserContextApi, UserEvent, UserMode, UserSpace};
use aster_frame::vm::{Vaddr, VmAllocOptions, VmIo, VmMapOptions, VmPerm, VmSpace, PAGE_SIZE};

/// The kernel's boot and initialization process is managed by Asterinas Framework.
/// After the process is done, the kernel's execution environment
/// (e.g., stack, heap, tasks) will be ready for use and the entry function
/// labeled as `#[aster_main]` will be called.
#[aster_main]
pub fn main() {
    println!("[Kernel] Boot success");
    let scheduler = Box::new(sched::MySched::new());
    aster_frame::task::set_scheduler(Box::leak(scheduler));
    let program_binary = include_bytes!("../example_apps/hello_world/hello_world");
    let user_space = create_user_space(program_binary);
    let user_task = create_user_task(Arc::new(user_space));
    user_task.run();
}

fn create_user_space(program: &[u8]) -> UserSpace {
    let user_pages = {
        let nframes = program.len().align_up(PAGE_SIZE) / PAGE_SIZE;
        let vm_frames = VmAllocOptions::new(nframes).alloc().unwrap();
        // Phyiscal memory pages can be only accessed
        // via the VmFrame abstraction.
        vm_frames.write_bytes(0, program).unwrap();
        vm_frames
    };
    let user_address_space = {
        const MAP_ADDR: Vaddr = 0x0040_0000; // The map addr for statically-linked executable

        // The page table of the user space can be
        // created and manipulated safely through
        // the VmSpace abstraction.
        let vm_space = VmSpace::new();
        let mut options = VmMapOptions::new();
        options.addr(Some(MAP_ADDR)).perm(VmPerm::RWXU);
        vm_space.map(user_pages, &options).unwrap();
        vm_space.activate();
        vm_space
    };
    let user_cpu_state = {
        const ENTRY_POINT: Vaddr = 0x0040_1000; // The entry point for statically-linked executable

        // The user-space CPU states can be initialized
        // to arbitrary values via the UserContext
        // abstraction.
        let mut user_cpu_state = UserContext::default();
        user_cpu_state.set_instruction_pointer(ENTRY_POINT);
        user_cpu_state
    };
    UserSpace::new(user_address_space, user_cpu_state)
}

fn create_user_task(user_space: Arc<UserSpace>) -> Arc<Task> {
    fn user_task() {
        let current = Task::current();
        // Switching between user-kernel space is
        // performed via the UserMode abstraction.
        let mut user_mode = {
            let user_space = current.user_space().unwrap();
            UserMode::new(user_space)
        };

        loop {
            // The execute method returns when system
            // calls or CPU exceptions occur.
            let user_event = user_mode.execute();
            // The CPU registers of the user space
            // can be accessed and manipulated via
            // the `UserContext` abstraction.
            let user_context = user_mode.context_mut();
            match user_event {
                UserEvent::Syscall => handle_syscall(user_context, current.user_space().unwrap()),
                UserEvent::Exception => {
                    println!("[Kernel] User mode exception detected!");
                    println!("{:#?}", user_context.trap_information());
                    break;
                }
            }
        }
    }

    // Kernel tasks are managed by the Framework,
    // while scheduling algorithms for them can be
    // determined by the users of the Framework.
    TaskOptions::new(user_task)
        .user_space(Some(user_space))
        .data(0)
        .build()
        .unwrap()
}

fn handle_syscall(user_context: &mut UserContext, user_space: &UserSpace) {
    const SYS_WRITE: usize = 1;
    const SYS_SHUTDOWN: usize = 48;
    const SYS_EXIT: usize = 60;

    match user_context.general_regs().a0 {
        SYS_WRITE => {
            // Access the user-space CPU registers safely.
            let (_, buf_addr, buf_len) = (
                user_context.general_regs().a1,
                user_context.general_regs().a2,
                user_context.general_regs().a3,
            );
            let buf = {
                let mut buf = vec![0u8; buf_len];
                // Copy data from the user space without
                // unsafe pointer dereferencing.
                user_space
                    .vm_space()
                    .read_bytes(buf_addr, &mut buf)
                    .unwrap();
                buf
            };
            // Use the console for output safely.
            println!("{}", str::from_utf8(&buf).unwrap());
            // Manipulate the user-space CPU registers safely.
            user_context.set_syscall_ret(buf_len);
        }
        SYS_SHUTDOWN => {
            println!("[Kernel] Shutting down");
            exit_qemu(QemuExitCode::Success);
        }
        SYS_EXIT => Task::current().exit(),
        s => {
            println!("[Kernel] Unimplemented syscall {} received", s);
            unimplemented!()
        }
    }
}
