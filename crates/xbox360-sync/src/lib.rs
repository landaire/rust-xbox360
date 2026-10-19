//! Kernel events, critical sections, spin locks, IRQL and interrupt control.
//!
//! Titles that use the standard library use `std::sync` for locks and
//! condition variables. The types here are for `no_std` code and for code
//! that shares kernel objects with the kernel or other modules.
//!
//! [`Event`] and [`CriticalSection`] hold kernel objects whose wait list
//! points into the object itself. Each records the address it was set up
//! at and repeats the setup on first use at a new address. A move needs
//! exclusive access, so no thread waits on or holds the object while it
//! moves.

#![no_std]
#![cfg(any(target_os = "xbox360", xbox360_docs))]

mod anchor;
mod critical_section;
mod event;
mod interrupt;
mod irql;
mod spin_lock;
mod wait;

pub use xbox360_sys::xboxkrnl::EventType;
pub use xbox360_sys::xboxkrnl::Irql;
pub use xbox360_sys::xboxkrnl::ProcessorMode;
pub use xbox360_sys::xboxkrnl::WaitReason;

pub use xbox360_status::WaitError;

pub use self::critical_section::CriticalSection;
pub use self::critical_section::CriticalSectionGuard;
pub use self::event::Event;
pub use self::interrupt::InterruptGuard;
pub use self::irql::IrqlGuard;
pub use self::irql::current_irql;
pub use self::spin_lock::SpinLock;
pub use self::spin_lock::SpinLockGuard;
pub use self::wait::NOT_ALERTABLE;
pub use self::wait::WaitOutcome;
pub use self::wait::wait_outcome;
pub use self::wait::wait_signaled;
