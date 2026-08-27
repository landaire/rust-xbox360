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
- [x] Xenia build usable for runs. Its Linux x64 backend passed guest-call
      args in the Microsoft ABI instead of SysV; patched the thunks (committed
      in the xenia checkout). See [[xenia-must-build-with-clang]].
- [x] Verify: Xenia runs it, `(DbgPrint) hello from rust on xbox 360` observed.

M0 done: Rust compiles to a XEX that boots and runs in Xenia and calls a
kernel import.

## M1: multithreading (done)

- [x] Bind ExCreateThread (0x0D), NtWaitForSingleObjectEx (0xFD), NtClose
      (0xCF), ExTerminateThread (0x19).
- [x] Main spawns a child thread; child runs concurrently and exits via
      ExTerminateThread (raw-thread returns to the exit sentinel fault); main
      joins with NtWaitForSingleObjectEx.
- [x] Verify: Xenia shows main (F8000028) and child (F800002C) both running
      with distinct thread ids, and a clean join.

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
