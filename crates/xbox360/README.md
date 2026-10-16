# xbox360

Safe interfaces to the Xbox 360 kernel and XAM for Rust code built for the `powerpc-unknown-xbox360` target.

## tl;dr

- Use `std` in titles that have it. This crate covers what `std` does not: kernel events and spin locks, module exports, console information, the title loader, controllers, message boxes and content packages.
- Use it in `no_std` code, such as system modules and small titles, for threads, locks, time, debug output and a global allocator.
- Kernel calls return `Result<_, KernelError>`, waits return `Result<_, WaitError>`, and XAM calls return `Result<_, Win32Error>`.
- Arguments from a closed set of values are enums. Some returned values, such as `DeviceType`, `DeviceSubtype`, `SigninState` and `Disposition`, are newtypes with a `kind()` that converts to an enum.

## Usage

```toml
[dependencies]
xbox360 = "0.1"
```

Print to the debug output:

```rust,ignore
xbox360::dbg_println!("loaded {} items", count);
```

Start a thread from an `extern "C"` function and wait for it:

```rust,ignore
extern "C" fn worker(argument: u32) -> u32 {
    argument + 1
}

xbox360::thread_entry!(WORKER = worker);

let thread = xbox360::thread::spawn_entry(WORKER, 41)?;
thread.join()?;
```

With the `alloc` feature, start a thread from a closure:

```rust,ignore
let thread = xbox360::thread::spawn(move || do_work())?;
```

Install the kernel pool as the global allocator in a `no_std` title that uses `alloc`:

```rust,ignore
#[global_allocator]
static ALLOCATOR: xbox360::pool::PoolAllocator =
    xbox360::pool::PoolAllocator::new(xbox360::pool::PoolTag::new(*b"Titl"));
```

Share state between threads with a kernel spin lock or critical section:

```rust,ignore
static COUNTER: xbox360::sync::SpinLock<u32> = xbox360::sync::SpinLock::new(0);

*COUNTER.lock() += 1;
```

Read the controller on port 0:

```rust,ignore
use xbox360::xam::{input, UserIndex};

if let Some(state) = input::state(UserIndex::User0)? {
    let pressed = state.gamepad.buttons.contains(input::Buttons::A);
}
```

End the title when it is done. A title must not return from its entry point, because the console stops with fatal error 0xE. `terminate` panics when you call it from a thread outside the title process:

```rust,ignore
xbox360::title::terminate()
```

## Modules

| Module | Contents | `std` equivalent |
| --- | --- | --- |
| `status` | `NtStatus`, `KernelError`, `Win32Error`, `Hresult`, the thread's last error | `std::io::Error` |
| `sync` | `Event`, `CriticalSection`, `SpinLock`, `IrqlGuard`, `InterruptGuard` | `std::sync` for locks |
| `atomic` | `AtomicU64` | none (32-bit atomics are in `core`) |
| `thread` | `spawn_entry`, `spawn`, `JoinHandle`, `sleep`, `exit` | `std::thread` |
| `time` | `Timebase`, `FileTime`, `TickCount`, `sleep` | `std::time` |
| `debug` | `DebugOutput`, `dbg_print!`, `dbg_println!` | `print!` |
| `pool` | `PoolAllocator` | `std::alloc::System` |
| `module` | `module_handle`, `resolve_export`, `load_image` | none |
| `console` | kernel versions, hardware flags, firmware requests | none |
| `title` | `terminate`, `return_to_dashboard`, `launch` | none |
| `xam` | controllers, message boxes, content packages, sign-in state | none |

The modules other than `status` exist only for `target_os = "xbox360"`. On other targets the crate has only `status` and the `sys` re-export, so it builds in a host workspace.

The types of the kernel and XAM interfaces, such as `ProcessorMode`, `Irql`, `UserIndex` and `Buttons`, come from xbox360-sys. The modules that use them re-export them.

The kernel object storage types, such as `Kevent` for `Event::as_ptr`, also come from xbox360-sys.

The `xam` modules follow the export signatures and structure layouts of [Xenia](https://github.com/xenia-project/xenia). Each module's documentation states what was checked only against Xenia.

The crate is built on [xbox360-sys](https://crates.io/crates/xbox360-sys), and re-exports it as `xbox360::sys`. Titles need [xex-link](https://github.com/landaire/rust-xbox360) to turn the import records into the XEX import table.

## License

Licensed under either the Apache License, Version 2.0 (`LICENSE-APACHE`) or the MIT license (`LICENSE-MIT`), at your option.
