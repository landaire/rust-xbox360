"""The toolchains cell of a project that builds for the host and the Xbox 360.

A project's `toolchains/BUCK` calls `xbox360_toolchains()`.
"""

load("@prelude//tests:test_toolchain.bzl", "noop_test_toolchain")
load("@prelude//toolchains:cxx.bzl", "cxx_tools_info_toolchain")
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")
load("@prelude//toolchains:python.bzl", "remote_python_toolchain")
load("@prelude//toolchains:remote_test_execution.bzl", "remote_test_execution_toolchain")
load(":cxx.bzl", "host_cxx_tools", "linker_override_toolchain", "rust_lld")
load(":flags.bzl", "XBOX360_RUSTC_FLAGS")
load(":python.bzl", "exec_python_bootstrap_toolchain")
load(":rust.bzl", "release_tool", "rust_release", "rust_release_toolchain")

_XBOX360 = "xbox360//platforms:xbox360"
_SYSROOT_CORE = "xbox360//platforms:sysroot-core"
_SYSROOT_STD = "xbox360//platforms:sysroot-std"
_SYSROOT_STD_SYSTEM = "xbox360//platforms:sysroot-std-system"

def xbox360_toolchains(
        rustc_flags: list[str] = [],
        rustc_binary_flags: list[str] = [],
        rustdoc_flags: list[str] = [],
        xbox360_rustc_flags: list[str] = [],
        xbox360_rustc_binary_flags: list[str] = []):
    """Declares the toolchains.

    `rustc_flags` and `rustc_binary_flags` (for binaries and tests) apply
    to host crates, `xbox360_rustc_flags` and `xbox360_rustc_binary_flags`
    to console crates, after the flags of buck/flags.bzl. Console flags also
    apply to the standard library crates. `rustdoc_flags` apply to every
    rustdoc run.
    """
    rust_release(name = "rust-release")

    # llvm-objdump and the other LLVM tools of the Rust release, which
    # understand the same instructions as its rustc.
    rust_release(
        name = "llvm-tools",
        components = ["llvm-tools-preview"],
    )

    release_tool(
        name = "llvm-objdump",
        path = "lib/rustlib/{host_triple}/bin/llvm-objdump",
        release = ":llvm-tools",
        visibility = ["PUBLIC"],
    )

    rust_release_toolchain(
        name = "rust",
        default_edition = "2021",
        explicit_sysroot = select({
            _XBOX360: True,
            "DEFAULT": False,
        }),
        panic_runtime = select({
            _XBOX360: "abort",
            "DEFAULT": "unwind",
        }),
        release = ":rust-release",
        rust_target_path = select({
            _XBOX360: "xbox360//targets:specs",
            "DEFAULT": None,
        }),
        rustc_binary_flags = select({
            _XBOX360: xbox360_rustc_binary_flags,
            "DEFAULT": rustc_binary_flags,
        }),
        rustc_flags = select({
            # Custom target specifications are unstable.
            _XBOX360: ["-Zunstable-options"] + XBOX360_RUSTC_FLAGS + xbox360_rustc_flags,
            "DEFAULT": rustc_flags,
        }),
        rustdoc_flags = rustdoc_flags,
        rustc_target_triple = select({
            _XBOX360: "powerpc-unknown-xbox360",
            "DEFAULT": None,
        }),
        sysroot_core = select({
            _SYSROOT_CORE: "xbox360//sysroot:core",
            _SYSROOT_STD: "xbox360//sysroot:core",
            _SYSROOT_STD_SYSTEM: "xbox360//sysroot:core",
            "DEFAULT": None,
        }),
        sysroot_others = select({
            _SYSROOT_CORE: ["xbox360//sysroot:alloc", "xbox360//sysroot:compiler_builtins"],
            _SYSROOT_STD: ["xbox360//sysroot:alloc", "xbox360//sysroot:compiler_builtins"],
            _SYSROOT_STD_SYSTEM: ["xbox360//sysroot:alloc", "xbox360//sysroot:compiler_builtins"],
            "DEFAULT": [],
        }),
        sysroot_panic_abort = select({
            _SYSROOT_STD: "xbox360//sysroot:panic_abort",
            _SYSROOT_STD_SYSTEM: "xbox360//sysroot:panic_abort",
            "DEFAULT": None,
        }),
        sysroot_std = select({
            _SYSROOT_STD: "xbox360//sysroot:std",
            _SYSROOT_STD_SYSTEM: "xbox360//sysroot:std-system",
            "DEFAULT": None,
        }),
        visibility = ["PUBLIC"],
    )

    # On Linux the linker driver can be GCC or Clang: the `-fuse-ld=lld`
    # that the prelude adds resolves to the lld that rustc bundles, so no
    # system lld is needed.
    host_cxx_tools(
        name = "linux-cxx-tools",
        linker = "c++",
    )

    cxx_tools_info_toolchain(
        name = "cxx-host",
        cxx_tools_info = select({
            "prelude//os:linux": ":linux-cxx-tools",
            "prelude//os:windows": "prelude//toolchains/msvc:msvc_tools",
            "DEFAULT": "prelude//toolchains/cxx/clang:path_clang_tools",
        }),
    )

    rust_lld(
        name = "rust-lld",
        release = ":rust-release",
        visibility = ["PUBLIC"],
    )

    # Xbox 360 executables are linked by rust-lld alone, without the host
    # linker driver and its flags.
    linker_override_toolchain(
        name = "cxx",
        base = ":cxx-host",
        linker = select({
            _XBOX360: ":rust-lld",
            "DEFAULT": None,
        }),
        visibility = ["PUBLIC"],
    )

    # Required by the genrule that remote_python_toolchain declares.
    system_genrule_toolchain(
        name = "genrule",
        visibility = ["PUBLIC"],
    )

    # Runs the prelude's Python tools with a downloaded CPython. The
    # bootstrap toolchain reaches the interpreter through an exec dep, so
    # that targets configured for the console (which has no host os) can
    # still run Python tools.
    remote_python_toolchain(
        name = "python",
        bootstrap = False,
        visibility = ["PUBLIC"],
    )

    exec_python_bootstrap_toolchain(
        name = "python_bootstrap",
        interpreter = ":cpython",
        visibility = ["PUBLIC"],
    )

    remote_test_execution_toolchain(
        name = "remote_test_execution",
        visibility = ["PUBLIC"],
    )

    noop_test_toolchain(
        name = "test",
        visibility = ["PUBLIC"],
    )
