
use core::hint::unreachable_unchecked as unreachable;
use core::sync::atomic::{spin_loop_hint as cpu_relax, AtomicUsize, Ordering};

const INCOMPLETE: usize = 0x0;
const RUNNING: usize = 0x1;
const COMPLETE: usize = 0x2;
const PANICKED: usize = 0x3;

pub struct Once {
    state: AtomicUsize,
}

unsafe impl Sync for Once {}
unsafe impl Send for Once {}

impl Once {
    pub const INIT: Self = Once {
        state: AtomicUsize::new(INCOMPLETE),
    };

    pub fn new() -> Once {
        Self::INIT
    }

    pub fn run_once<F: FnOnce()>(&self, initializer: F) {
        let mut status = self.state.load(Ordering::SeqCst);

        if status == INCOMPLETE {
            status = self
                .state
                .compare_and_swap(INCOMPLETE, RUNNING, Ordering::SeqCst);
            if status == INCOMPLETE {
                initializer();
                status = COMPLETE;
                self.state.store(status, Ordering::SeqCst);
            }
        }

        loop {
            match status {
                INCOMPLETE => unreachable!(),
                RUNNING => {
                    cpu_relax();
                    status = self.state.load(Ordering::SeqCst)
                }
                PANICKED => panic!("Once has panicked"),
                COMPLETE => return (),
                _ => unsafe { unreachable() },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once() {
        static mut FUNC_PTR: usize = 0;
        static ONCE: Once = Once::INIT;
        ONCE.run_once(|| unsafe { FUNC_PTR = 2 });
        assert_eq!(unsafe { FUNC_PTR }, 2);
    }
}
