use thiserror::Error;

use crate::isa::Gpr;
use crate::isa::GprSet;

/// Why a path after the save helper call may observe the clobbered link
/// register.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum LivePath {
    #[error("`{mnemonic}` reads the link register")]
    Read { mnemonic: String },
    #[error("`{mnemonic}` may access the link register")]
    UnknownAccess { mnemonic: String },
    #[error("branch to `{target}` leaves the function")]
    TailBranch { target: String },
    #[error("indirect branch")]
    IndirectBranch,
    #[error("`{mnemonic}` has unknown control flow")]
    UnknownControl { mnemonic: String },
    #[error("inline assembly")]
    InlineAsm,
    #[error("directive `{directive}` inside the function")]
    Directive { directive: String },
    #[error("control reaches the end of the function")]
    FallsOffEnd,
}

/// Why a candidate sequence is left unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum Refusal {
    #[error("the function allocates a frame more than once")]
    MultipleAllocations,
    #[error("the allocation block does not save the link register through r0")]
    LinkRegisterNotSaved,
    #[error("the return block does not restore the link register through r0")]
    LinkRegisterNotRestored,
    #[error("registers {registers} are not a contiguous range ending at r30 or r31")]
    NonContiguous { registers: GprSet },
    #[error("a save slot overlaps the back chain word")]
    SlotOverlapsBackChain,
    #[error("{register} is written before its save")]
    WrittenBeforeSave { register: Gpr },
    #[error("{register} is accessed after its restore")]
    AccessedAfterRestore { register: Gpr },
    #[error("the stack pointer is accessed across a moved instruction")]
    StackPointerAccessed,
    #[error("a save slot is accessed across a moved instruction")]
    SlotAccessed,
    #[error("the link register is accessed across a moved instruction")]
    LinkRegisterAccessed,
    #[error("`{mnemonic}` has no effect model")]
    UnknownInstruction { mnemonic: String },
    #[error("the link register is live after the save helper call: {path}")]
    LinkRegisterLive { path: LivePath },
    #[error("the return block overlaps the allocation block")]
    OverlapsPrologue,
    #[error("the function branches to the relative address `{target}`")]
    RelativeBranch { target: String },
    #[error("the function has a line with several statements")]
    MultipleStatements,
    #[error("call frame annotation `{annotation}` lies between moved instructions")]
    UnwindAnnotation { annotation: String },
}

impl Refusal {
    /// Category name for summaries.
    pub fn kind(&self) -> &'static str {
        match self {
            Refusal::MultipleAllocations => "multiple-allocations",
            Refusal::LinkRegisterNotSaved => "link-register-not-saved",
            Refusal::LinkRegisterNotRestored => "link-register-not-restored",
            Refusal::NonContiguous { .. } => "non-contiguous",
            Refusal::SlotOverlapsBackChain => "slot-overlaps-back-chain",
            Refusal::WrittenBeforeSave { .. } => "written-before-save",
            Refusal::AccessedAfterRestore { .. } => "accessed-after-restore",
            Refusal::StackPointerAccessed => "stack-pointer-accessed",
            Refusal::SlotAccessed => "slot-accessed",
            Refusal::LinkRegisterAccessed => "link-register-accessed",
            Refusal::UnknownInstruction { .. } => "unknown-instruction",
            Refusal::LinkRegisterLive { path } => match path {
                LivePath::Read { .. } => "link-register-live/read",
                LivePath::UnknownAccess { .. } => "link-register-live/unknown-access",
                LivePath::TailBranch { .. } => "link-register-live/tail-branch",
                LivePath::IndirectBranch => "link-register-live/indirect-branch",
                LivePath::UnknownControl { .. } => "link-register-live/unknown-control",
                LivePath::InlineAsm => "link-register-live/inline-asm",
                LivePath::Directive { .. } => "link-register-live/directive",
                LivePath::FallsOffEnd => "link-register-live/falls-off-end",
            },
            Refusal::OverlapsPrologue => "overlaps-prologue",
            Refusal::RelativeBranch { .. } => "relative-branch",
            Refusal::MultipleStatements => "multiple-statements",
            Refusal::UnwindAnnotation { .. } => "unwind-annotation",
        }
    }
}
