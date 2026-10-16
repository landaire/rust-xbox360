# xbox360-sys

Raw bindings to the Xbox 360 kernel (`xboxkrnl.exe`) and XAM (`xam.xex`) for Rust code built for the `powerpc-unknown-xbox360` target.

## tl;dr

- Call kernel and XAM exports by ordinal from `no_std` titles, system modules, and the standard library.
- Imports are `unsafe` functions. The few that cannot break memory safety are safe.
- Export your own functions by ordinal, and hand entry points and callbacks to the kernel with the register state it expects.
- Use the interlocked operations that the devkit kernel loader accepts.
- Pass and read typed values: enums for closed argument sets, flag types for bit fields, newtypes for handles, statuses and results, and storage types for kernel objects.
- Share the types and `status` with host code. They compile on every target.

## Usage

```toml
[target.'cfg(target_os = "xbox360")'.dependencies]
xbox360-sys = "0.1"
```

Call an import in an `unsafe` block, and state why the arguments meet the requirements of the export:

```rust,ignore
// SAFETY: the format is a NUL-terminated string without conversions.
unsafe { xbox360_sys::xboxkrnl::DbgPrint(c"hello from rust\n".as_ptr().cast()) };
```

A few imports, such as `KeQueryPerformanceFrequency`, are declared safe, and you call them without `unsafe`:

```rust,ignore
let frequency = xbox360_sys::xboxkrnl::KeQueryPerformanceFrequency();
```

Arguments and results are typed:

```rust,ignore
use xbox360_sys::xboxkrnl::{self, Handle, ProcessorMode};

// SAFETY: `handle` is an open event handle that this code owns.
let status = unsafe { xboxkrnl::NtWaitForSingleObjectEx(handle, ProcessorMode::User, false, core::ptr::null()) };
status.result()?;
```

Kernel objects that you allocate, such as events, timers and critical sections, have storage types. Keep the storage at one address while the kernel uses it:

```rust,ignore
use xbox360_sys::xboxkrnl::{self, EventType, Kevent};

static DONE: Kevent = Kevent::new();

// SAFETY: `DONE` is static storage of the size of a KEVENT.
unsafe { xboxkrnl::KeInitializeEvent(DONE.as_ptr(), EventType::Notification, false) };
```

Read a data import, such as `XboxHardwareInfo`, through its address:

```rust,ignore
let address = xbox360_sys::xboxkrnl::XboxHardwareInfo.address();
```

Declare an import that the crate does not have, or a different signature for one it has, with the same macro the crate uses. Write `pub safe` instead of `pub` only when the export cannot break memory safety. Write `-> !` when the export does not return:

```rust,ignore
xbox360_sys::import_thunk!(
    /// Writes a formatted string with two values to the debug output.
    pub DbgPrintTwo(format: *const u8, first: u32, second: u32) -> u32,
    lib = xbox360_sys::lib_index::XBOXKRNL,
    ordinal = 0x03
);
```

Code that the kernel or another module calls uses the console ABI, which expects the full 64-bit nonvolatile registers back. Wrap it with `bridged!`, or export it with `export!`:

```rust,ignore
extern "C" fn worker(context: u32) -> u32 {
    0
}

xbox360_sys::bridged!(pub worker_entry, worker);
xbox360_sys::export!(1, worker);
```

Pass `worker_entry`, not `worker`, to `ExCreateThread` with `xapi_thread_startup` 0. When `worker` returns, the kernel ends the thread with the returned value as its exit code.

The crate needs [xex-link](https://github.com/landaire/rust-xbox360) to turn the import and export records into the XEX import and export tables. Imports, `processor`, `interlocked` and `entry_bridge` exist only for `target_os = "xbox360"`. On other targets the crate has only the types, the constants and `status`, so host code can share them.

## Why the stubs exist

Console code stores r3-r10 into the caller's frame before using them. The SysV frames that LLVM emits do not reserve that area, so every import goes through a small stub that allocates it. Without the stub, a kernel call can overwrite the caller's saved registers on hardware. Xenia does not show the problem.

Imports pass at most ten arguments of 32 bits or less. SysV passes a 64-bit argument in a register pair, and the console expects it in one register. The stubs of `KeSetTimerEx`, `KeSetSpecialPurposeRegister`, `KeSetPMWRegister`, `KeSetPRVRegister` and `XamNotifyCreateListenerInternal` join the pair before the call.

## Types

Every type in an import signature passes like the C integer of its width. Enums are `#[repr(u32)]` (or the C width), and the other types are `#[repr(transparent)]` newtypes.

Values the kernel returns are newtypes, never enums, so an unknown value is not undefined behaviour. NTSTATUS results are `NtStatus`, and Win32 error results are `Win32ErrorCode`. `BOOL` results are `Boolean`, and `BOOLEAN` results are `Boolean8`, which reads only the low byte.

Enums that name only part of the values the kernel accepts are `#[non_exhaustive]`. So is `KernelError`.

Flag types keep bits without a name only through `from_bits_retain`. `from_bits` rejects them. Enums convert from integers through `from_raw` or `TryFrom`.

## License

Licensed under either the Apache License, Version 2.0 (`LICENSE-APACHE`) or the MIT license (`LICENSE-MIT`), at your option.
