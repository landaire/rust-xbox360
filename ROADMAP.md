# Roadmap

Milestones toward "stdlib Rust with multithreading and networking, running in
Xenia." Each milestone ends with a run in Xenia as verification.

## M0: boot a XEX

- [ ] Custom rustc target `powerpc-unknown-xbox360` (32-bit BE, static reloc,
      hard-float, os none, redzone off, panic abort).
- [ ] `-Z build-std=core` builds a `#![no_std] #![no_main]` crate.
- [ ] Linker script, image base 0x82000000, single load segment.
- [ ] `xex-link`: ELF32 PPC -> Xbox PPC PE -> XEX (via Xex2Builder).
- [ ] Title entry calls DbgPrint (xboxkrnl 0x03) then returns an exit code.
- [ ] Extend the XEX import table to reference xboxkrnl.exe DbgPrint.
- [ ] Xenia build usable for headless runs.
- [ ] Verify: Xenia runs it, DbgPrint output observed, clean exit.

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
