use core::marker::PhantomData;

use xbox360_sys::processor;
use xbox360_sys::xboxkrnl;

use super::Irql;

/// A raised IRQL, lowered to the previous level on drop. Waiting while it
/// is above [`Irql::APC`] stops the system with a bug check. Drop guards
/// and spin lock guards in the reverse order of creation, or the IRQL ends
/// at the level of the guard dropped last.
pub struct IrqlGuard {
    previous: Irql,
    _thread: PhantomData<*const ()>,
}

impl IrqlGuard {
    /// Raise the IRQL to `level`. `None` if `level` is below the current
    /// IRQL, which the kernel does not allow.
    pub fn raise(level: Irql) -> Option<Self> {
        if level < current_irql() {
            return None;
        }

        // SAFETY: `level` is not below the current IRQL (checked above), and
        // Drop restores the previous level.
        let previous = unsafe { xboxkrnl::KfRaiseIrql(level) };

        Some(Self {
            previous,
            _thread: PhantomData,
        })
    }

    pub const fn previous(&self) -> Irql {
        self.previous
    }
}

impl Drop for IrqlGuard {
    fn drop(&mut self) {
        // SAFETY: restores the level that KfRaiseIrql returned.
        unsafe { xboxkrnl::KfLowerIrql(self.previous) };
    }
}

/// The IRQL of the current processor.
pub fn current_irql() -> Irql {
    processor::current_irql()
}
