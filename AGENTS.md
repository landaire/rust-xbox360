This project is for building an llvm-based Xbox 360 toolchain. The idea is basically to replace Microsoft's proprietary toolchain. Our end-goal is to compile stdlib Rust code for Xbox 360.

Xenia: ~/src/xenia
llvm: ~/src/llvm-project
Rust: ~/src/rust
Acceleration: ~/dev/acceleration
binja-xex2: ~/dev/binja-xex2

Guidelines:

1. No co-authoring commits.
2. Simple commit descriptions. Do not over-explain things.
3. Use simple and concise technical language. No jargon.
4. For each project, figure out the project's formatting guidelines / style for both code and commits and adhere to it.

Xenia should be used to figure out APIs. In general on Xbox 360, Xam and kernel APIs are linked by ordinal rather than name.

The Xbox 360 runs XEX files, not EXE files. I am unsure if it makes sense to have LLVM / Rust output an exe that then gets wrapped by some other part of the toolchain as an xex, or if they should output an xex. Take inspiration from other odd targets that may similarly wrap ELF or whatever.

Acceleration has a lot of Xbox 360-related code we can use for building XEX files. Bring it in-tree if needed, but do not commit any potentially copyrighted bits (encryption keys)

Do not use resources outside of this list.
