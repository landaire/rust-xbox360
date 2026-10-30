"""The toolchains cell of a project that builds for the host and the Xbox 360.

A project's `toolchains/BUCK` calls `xbox360_toolchains()`.
"""

load("@prelude//tests:test_toolchain.bzl", "noop_test_toolchain")
load("@prelude//toolchains:cxx.bzl", "cxx_tools_info_toolchain")
load("@prelude//toolchains:genrule.bzl", "system_genrule_toolchain")
load("@prelude//toolchains:python.bzl", "remote_python_toolchain")
load("@prelude//toolchains:remote_test_execution.bzl", "remote_test_execution_toolchain")
load(":custom_toolchain.bzl", "bootstrap_dist", "path_clang", "prebuilt_toolchain_archive", "xbox360_clang")
load(":cxx.bzl", "host_cxx_tools", "linker_override_toolchain", "rust_lld")
load(":flags.bzl", "XBOX360_RUSTC_FLAGS")
load(":nightly_toolchain.bzl", "DEFAULT_PPC64_TOOLCHAIN", "NIGHTLY_TOOLCHAIN")
load(":python.bzl", "exec_python_bootstrap_toolchain")
load(":rust.bzl", "release_tool", "rust_release", "rust_release_toolchain")

_XBOX360 = "xbox360//platforms:target[xbox]"
_PPC64 = "xbox360//platforms:codegen[ppc64-ilp32]"
_XBOX360_PPC64_ILP32 = "xbox360//platforms:xbox-ppc64-ilp32"
_SYSROOT_CORE = "xbox360//platforms:sysroot-core"
_SYSROOT_STD = "xbox360//platforms:sysroot-std"
_SYSROOT_STD_SYSTEM = "xbox360//platforms:sysroot-std-system"

# Configurations of the rustc_bootstrap cell, which builds the PPC64 compiler.
# The prelude's rules in that cell reach their toolchains through this cell.
_BOOTSTRAP = "rustc_bootstrap//constraints:bootstrap=true"

def _by_codegen(ppc32, ppc64):
    return select({
        _PPC64: ppc64,
        "DEFAULT": ppc32,
    })

def _config_or_default(name: str, default: str) -> str:
    value = read_root_config("xbox360", name, "")
    return value if value else default

def _ppc64_distributions() -> tuple[str, ...]:
    mode = read_root_config(
        "xbox360",
        "ppc64_toolchain",
        DEFAULT_PPC64_TOOLCHAIN,
    )
    if mode == "source":
        return (
            "rustc_bootstrap//stage1:dist",
            "rustc_bootstrap//stage0:ci_llvm",
        )
    if mode != "prebuilt":
        fail("xbox360.ppc64_toolchain must be `source` or `prebuilt`, not `{}`".format(mode))

    prebuilt_toolchain_archive(
        name = "ppc64-prebuilt",
        url = _config_or_default("ppc64_toolchain_url", NIGHTLY_TOOLCHAIN["url"]),
        sha256 = _config_or_default("ppc64_toolchain_sha256", NIGHTLY_TOOLCHAIN["sha256"]),
        strip_prefix = _config_or_default(
            "ppc64_toolchain_strip_prefix",
            NIGHTLY_TOOLCHAIN["strip_prefix"],
        ),
    )
    return (":ppc64-prebuilt", ":ppc64-prebuilt")

def xbox360_toolchains(
        rustc_flags = [],
        rustc_binary_flags = [],
        rustdoc_flags: list[str] = [],
        xbox360_rustc_flags = [],
        xbox360_rustc_binary_flags = []):
    """Declares the toolchains.

    `rustc_flags` and `rustc_binary_flags` (for binaries and tests) apply
    to host crates, `xbox360_rustc_flags` and `xbox360_rustc_binary_flags`
    to console crates, after the flags of buck/flags.bzl. Console flags also
    apply to the standard library crates. `rustdoc_flags` apply to every
    rustdoc run.

    The `xbox360//platforms:codegen` constraint selects the compiler. With
    its default, `ppc32`, the Rust release and its LLVM tools build every
    crate. With `ppc64-ilp32`, the rustc and LLVM that the rustc_bootstrap
    cell builds from source build both the console crates and the host crates
    (proc macros, build scripts and tools) in the same configuration.
    """

    # PPC32: the Rust release.
    rust_release(name = "rust-release")

    # llvm-objdump and the other LLVM tools of the Rust release, which
    # understand the same instructions as its rustc.
    rust_release(
        name = "llvm-tools",
        components = ["llvm-tools-preview"],
    )

    # PPC64: either build the compiler and LLVM from source or download the
    # archive pinned by nightly_toolchain.bzl. Both paths expose the same
    # Rust-release directory layout to the toolchain rules below.
    ppc64_rust_dist, ppc64_llvm_dist = _ppc64_distributions()
    bootstrap_dist(
        name = "rust-ppc64",
        dist = ppc64_rust_dist,
    )

    bootstrap_dist(
        name = "llvm-ppc64",
        dist = ppc64_llvm_dist,
    )

    release_tool(
        name = "llvm-objdump-ppc32",
        path = "lib/rustlib/{host_triple}/bin/llvm-objdump",
        release = ":llvm-tools",
    )

    # VMX128 encodings overlap later POWER instructions, so the disassembler
    # decodes them only with the feature that the PPC64 rustc enables.
    release_tool(
        name = "llvm-objdump-ppc64",
        path = "bin/llvm-objdump",
        release = ":llvm-ppc64",
        args = ["--mattr=+vmx128"],
    )

    native.alias(
        name = "llvm-objdump",
        actual = _by_codegen(":llvm-objdump-ppc32", ":llvm-objdump-ppc64"),
        visibility = ["PUBLIC"],
    )

    release_tool(
        name = "llvm-objcopy-ppc32",
        path = "lib/rustlib/{host_triple}/bin/llvm-objcopy",
        release = ":llvm-tools",
    )

    release_tool(
        name = "llvm-objcopy-ppc64",
        path = "bin/llvm-objcopy",
        release = ":llvm-ppc64",
    )

    native.alias(
        name = "llvm-objcopy",
        actual = _by_codegen(":llvm-objcopy-ppc32", ":llvm-objcopy-ppc64"),
        visibility = ["PUBLIC"],
    )

    # rustc passes `-flavor gnu` itself because the Xbox 360 target
    # specifications select the gnu-lld linker flavor.
    rust_lld(
        name = "rust-lld-ppc32",
        release = ":rust-release",
    )

    release_tool(
        name = "rust-lld-ppc64",
        path = "bin/ld.lld",
        release = ":llvm-ppc64",
    )

    native.alias(
        name = "rust-lld",
        actual = _by_codegen(":rust-lld-ppc32", ":rust-lld-ppc64"),
        visibility = ["PUBLIC"],
    )

    path_clang(
        name = "clang-ppc32",
    )

    release_tool(
        name = "clang-ppc64",
        path = "bin/clang",
        release = ":llvm-ppc64",
    )

    xbox360_clang(
        name = "clang",
        linker = _by_codegen(":rust-lld-ppc32", ":rust-lld-ppc64"),
        target_triple = _by_codegen("powerpc-unknown-none-eabi", "powerpc64-unknown-xbox360"),
        tool = _by_codegen(":clang-ppc32", ":clang-ppc64"),
        visibility = ["PUBLIC"],
    )

    rust_release_toolchain(
        name = "xbox360-rust",
        default_edition = "2021",
        # Proc macros and build scripts of PPC64 configurations run on the
        # PPC64 host execution platform, whose rustc can load their metadata.
        exec_compatible_with = _by_codegen([], [_PPC64]),
        explicit_sysroot = select({
            _XBOX360: True,
            "DEFAULT": False,
        }),
        panic_runtime = select({
            _XBOX360: "abort",
            "DEFAULT": "unwind",
        }),
        release = _by_codegen(":rust-release", ":rust-ppc64"),
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
            # The host linker driver runs with `-fuse-ld=lld`. The Rust
            # release ships that lld; the source-built compiler finds the
            # one of its LLVM.
            "DEFAULT": rustc_flags + _by_codegen([], ["-Clink-arg=-B$(location :llvm-ppc64)/bin"]),
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
    )

    native.toolchain_alias(
        name = "rust",
        actual = select({
            _BOOTSTRAP: "rustc_bootstrap//toolchains:rust",
            "DEFAULT": ":xbox360-rust",
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

    # Xbox 360 executables are linked by rust-lld alone, without the host
    # linker driver and its flags.
    linker_override_toolchain(
        name = "xbox360-cxx",
        base = ":cxx-host",
        linker = select({
            _XBOX360_PPC64_ILP32: ":rust-lld-ppc64",
            _XBOX360: ":rust-lld-ppc32",
            "DEFAULT": None,
        }),
    )

    native.toolchain_alias(
        name = "cxx",
        actual = select({
            _BOOTSTRAP: "rustc_bootstrap//toolchains:cxx",
            "DEFAULT": ":xbox360-cxx",
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
