# Roadmap

Milestones toward "stdlib Rust with multithreading and networking, running in
Xenia." Each milestone ends with a run in Xenia as verification.

## M0: boot a XEX

- [x] Custom rustc target `powerpc-unknown-xbox360` (32-bit BE, static reloc,
      hard-float, os none, redzone off, panic abort).
- [x] `-Z build-std=core` builds a `#![no_std] #![no_main]` crate.
- [x] Linker script, image base 0x82000000, single load segment.
- [x] `xex-link`: ELF32 PPC -> Xbox PPC PE -> XEX (via Xex2Builder).
- [x] Title entry calls DbgPrint (xboxkrnl 0x03) then returns an exit code.
- [x] Extend the XEX import table to reference xboxkrnl.exe DbgPrint
      (`xbox-sys` thunks + `acceleration` Xex2Builder import support).
- [x] XEX verified loadable: gdb confirms image, code, string, and the
      DbgPrint import thunk patched to `sc 2` in Xenia guest memory.
- [ ] Xenia build usable for headless runs. A GCC build's x64 JIT is
      miscompiled (garbage function pointers; other XEXs fail the same way);
      rebuilding with clang. Debug build in progress to isolate/fix.
- [ ] Verify: Xenia runs it, DbgPrint output observed.

## M1: multithreading

- [ ] Bind ExCreateThread (xboxkrnl 0x0D) and a wait primitive.
- [ ] Spawn a thread; main and child update a shared atomic; join.
- [ ] Verify: Xenia shows both threads ran and the atomic result is correct.

## M2: networking

- [ ] Bind NetDll_ WSAStartup/socket/bind/sendto/recvfrom (xam.xex).
- [ ] UDP echo or send to host; observe traffic via Xenia's host sockets.
- [ ] Verify: packet round-trips in Xenia. Goal met (multithread + network).

## M3: std port (end goal)

- [ ] Add the target to `~/src/rust` and implement the std sys layer
      (thread, mutex/condvar, net, time, stdout via DbgPrint, alloc).
- [ ] Build std for the target.
- [ ] A plain stdlib program using std::thread and std::net runs in Xenia.

## Deferred

- VMX128 codegen, `.pdata` unwind, millicode helpers, LZX compression of the
  XEX, retail encryption, real signing, exception handling/unwinding.
