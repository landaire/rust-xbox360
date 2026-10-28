"""The toolchains cell of a project that builds for the host and the Xbox 360.

A project's `toolchains/BUCK` calls `xbox360_toolchains()`.
"""

load("@prelude//tests:test_toolchain.bzl", "noop_test_toolchain")
load("@prelude//toolchains:cxx.bzl", "cxx_tools_info_toolchain")
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")
load("@prelude//toolchains:python.bzl", "remote_python_toolchain")
load("@prelude//toolchains:remote_test_execution.bzl", "remote_test_execution_toolchain")
load(":custom_toolchain.bzl", "buck_llvm", "buck_rust", "path_clang", "xbox360_clang")
load(":cxx.bzl", "host_cxx_tools", "linker_override_toolchain", "rust_lld")
load(":flags.bzl", "XBOX360_RUSTC_FLAGS")
load(":python.bzl", "exec_python_bootstrap_toolchain")
load(":rust.bzl", "release_tool", "rust_release", "rust_release_toolchain")

_XBOX360 = "xbox360//platforms:target[xbox]"
_XBOX360_PPC64_ILP32 = "xbox360//platforms:xbox-ppc64-ilp32"
_SYSROOT_CORE = "xbox360//platforms:sysroot-core"
_SYSROOT_STD = "xbox360//platforms:sysroot-std"
_SYSROOT_STD_SYSTEM = "xbox360//platforms:sysroot-std-system"


def xbox360_toolchains(
        rustc_flags = [],
        rustc_binary_flags = [],
        rustdoc_flags: list[str] = [],
        xbox360_rustc_flags = [],
        xbox360_rustc_binary_flags = [],
        toolchain = "ppc32"):
    """Declares the toolchains.

    `rustc_flags` and `rustc_binary_flags` (for binaries and tests) apply
    to host crates, `xbox360_rustc_flags` and `xbox360_rustc_binary_flags`
    to console crates, after the flags of buck/flags.bzl. Console flags also
    apply to the standard library crates. `rustdoc_flags` apply to every
    rustdoc run. `toolchain` is either the release PPC32 toolchain or the
    source-built experimental PPC64 toolchain.
    """
    if toolchain not in ["ppc32", "ppc64"]:
        fail("toolchain must be \"ppc32\" or \"ppc64\", not {}".format(repr(toolchain)))
    custom_compiler = toolchain == "ppc64"
    if custom_compiler:
        buck_llvm(
            name = "custom-llvm",
            llvm = "xbox360//toolchain:local-llvm",
            visibility = ["PUBLIC"],
        )
        buck_rust(
            name = "rust-release",
            dist = "xbox360//toolchain:local-rust",
            visibility = ["PUBLIC"],
        )
        llvm_tools_release = ":custom-llvm"
    else:
        rust_release(name = "rust-release")

        # llvm-objdump and the other LLVM tools of the Rust release, which
        # understand the same instructions as its rustc.
        rust_release(
            name = "llvm-tools",
            components = ["llvm-tools-preview"],
        )
        llvm_tools_release = ":llvm-tools"

    release_tool(
        name = "llvm-objdump",
        path = "bin/llvm-objdump" if custom_compiler else "lib/rustlib/{host_triple}/bin/llvm-objdump",
        release = llvm_tools_release,
        visibility = ["PUBLIC"],
    )

    release_tool(
        name = "llvm-objcopy",
        path = "bin/llvm-objcopy" if custom_compiler else "lib/rustlib/{host_triple}/bin/llvm-objcopy",
        release = llvm_tools_release,
        visibility = ["PUBLIC"],
    )

    if custom_compiler:
        release_tool(
            name = "clang-bin",
            path = "bin/clang",
            release = ":custom-llvm",
        )
        xbox360_clang(
            name = "clang",
            target_triple = "powerpc64-unknown-xbox360",
            tool = ":clang-bin",
            visibility = ["PUBLIC"],
        )
        release_tool(
            name = "clang++",
            path = "bin/clang++",
            release = ":custom-llvm",
        )
    else:
        path_clang(
            name = "clang",
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
            _XBOX360_PPC64_ILP32: "powerpc64-unknown-xbox360",
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

    # The PPC64 toolchain uses its pinned clang and LLD for host Rust links.
    # The release toolchain keeps using the host C++ driver.
    host_cxx_tools(
        name = "linux-cxx-tools",
        linker_tool = ":clang++" if custom_compiler else None,
    )

    cxx_tools_info_toolchain(
        name = "cxx-host",
        cxx_tools_info = select({
            "prelude//os:linux": ":linux-cxx-tools",
            "prelude//os:windows": "prelude//toolchains/msvc:msvc_tools",
            "DEFAULT": "prelude//toolchains/cxx/clang:path_clang_tools",
        }),
    )

    if custom_compiler:
        release_tool(
            name = "rust-lld",
            path = "bin/ld.lld",
            release = ":custom-llvm",
            visibility = ["PUBLIC"],
        )
    else:
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
