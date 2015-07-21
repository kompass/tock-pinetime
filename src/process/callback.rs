use core::intrinsics::transmute;
use process;
use process::Process;

pub struct Callback {
    // We want more expressive types for this. For now, the kernel is expected
    // to unsafely cast these to `Process` and `fn()`, respectively. Even these
    // types, however, leak some information about the calling application that
    // we probably shouldn't leak.
    process_ptr: *mut (),
    fn_ptr: *mut ()
}

impl Callback {
    pub unsafe fn new(process: &mut Process<'static>, fn_ptr: *mut ()) -> Callback {
        Callback {
            process_ptr: process as *mut Process<'static> as *mut (),
            fn_ptr: fn_ptr
        }
    }

    pub fn schedule(&mut self, r0: usize, r1: usize, r2: usize) {
        unsafe {
            let process : &mut Process = transmute(self.process_ptr);
            process.callbacks.enqueue(process::Callback{
                r0: r0,
                r1: r1,
                r2: r2,
                pc: self.fn_ptr as usize
            });
        }
    }
}
