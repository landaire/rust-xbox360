"""Rust toolchain built from the archives pinned in rust_release.bzl."""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:rust_toolchain.bzl", "PanicRuntime", "RustExplicitSysrootDeps", "RustToolchainInfo")
load(":rust_release.bzl", "RUST_HOSTS")

RustReleaseInfo = provider(fields = {
    # rustc, rustdoc, clippy-driver and the host standard library in one
    # directory, laid out like a rustup toolchain.
    "sysroot": provider_field(Artifact),
    "executable_suffix": provider_field(str),
    "host_triple": provider_field(str),
})

def _select_host(field: typing.Callable) -> Select:
    """Selects `field(host)` for the configured host."""
    return select({
        "prelude//os:" + os_name: select({
            "prelude//cpu:" + cpu: field(host)
            for cpu, host in cpus.items()
        })
        for os_name, cpus in RUST_HOSTS.items()
    })

def rust_release_archives(components: list[str]) -> list[str]:
    """Declares an `http_file` per component and returns their labels."""
    labels = []
    for component in components:
        name = "rust-{}-archive".format(component)
        native.http_file(
            name = name,
            urls = [_select_host(lambda host: host["archives"][component]["url"])],
            sha256 = _select_host(lambda host: host["archives"][component]["sha256"]),
            out = "{}.tar.xz".format(component),
        )
        labels.append(":" + name)
    return labels

def _rust_release_impl(ctx: AnalysisContext) -> list[Provider]:
    sysroot = ctx.actions.declare_output("sysroot", dir = True)
    ctx.actions.run(
        cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._assembler,
            sysroot.as_output(),
            [archive[DefaultInfo].default_outputs[0] for archive in ctx.attrs.archives],
        ),
        category = "rust_sysroot",
    )
    return [
        DefaultInfo(default_output = sysroot),
        RustReleaseInfo(
            sysroot = sysroot,
            executable_suffix = ctx.attrs.executable_suffix,
            host_triple = ctx.attrs.host_triple,
        ),
    ]

# The host toolchain merged from the release archives. Toolchains reach it
# through `attrs.exec_dep`, so it is configured once for the execution
# platform and its selects describe the host.
_rust_release = rule(
    impl = _rust_release_impl,
    attrs = {
        "archives": attrs.list(attrs.dep()),
        "executable_suffix": attrs.string(),
        "host_triple": attrs.string(),
        "_assembler": attrs.source(default = "xbox360//buck:assemble_rust_sysroot.py"),
        "_python_bootstrap_toolchain": toolchains_common.python_bootstrap(),
    },
)

def rust_release(name: str, visibility: list[str] = []):
    _rust_release(
        name = name,
        archives = rust_release_archives(["rustc", "rust-std", "clippy-preview"]),
        executable_suffix = select({
            "DEFAULT": "",
            "prelude//os:windows": ".exe",
        }),
        host_triple = _select_host(lambda host: host["triple"]),
        visibility = visibility,
    )

def _rust_release_toolchain_impl(ctx: AnalysisContext) -> list[Provider]:
    release = ctx.attrs.release[RustReleaseInfo]

    def tool(name):
        return RunInfo(args = cmd_args(release.sysroot, format = "{}/bin/" + name + release.executable_suffix))

    explicit_sysroot_deps = None
    if ctx.attrs.explicit_sysroot:
        explicit_sysroot_deps = RustExplicitSysrootDeps(
            core = ctx.attrs.sysroot_core,
            proc_macro = ctx.attrs.sysroot_proc_macro,
            std = ctx.attrs.sysroot_std,
            panic_unwind = None,
            panic_abort = ctx.attrs.sysroot_panic_abort,
            others = ctx.attrs.sysroot_others,
        )

    return [
        DefaultInfo(),
        RustToolchainInfo(
            clippy_driver = tool("clippy-driver"),
            compiler = tool("rustc"),
            default_edition = ctx.attrs.default_edition,
            explicit_sysroot_deps = explicit_sysroot_deps,
            panic_runtime = PanicRuntime(ctx.attrs.panic_runtime),
            rust_target_path = ctx.attrs.rust_target_path,
            rustc_binary_flags = ctx.attrs.rustc_binary_flags,
            rustc_flags = ctx.attrs.rustc_flags,
            rustc_target_triple = ctx.attrs.rustc_target_triple or release.host_triple,
            rustdoc = tool("rustdoc"),
        ),
    ]

rust_release_toolchain = rule(
    impl = _rust_release_toolchain_impl,
    attrs = {
        "default_edition": attrs.option(attrs.string(), default = None),
        # Pass the standard library crates explicitly instead of the
        # prebuilt sysroot (the sysroot_* attributes; all unset while those
        # crates are compiled).
        "explicit_sysroot": attrs.bool(default = False),
        "panic_runtime": attrs.enum(["unwind", "abort"], default = "unwind"),
        # The execution platform's toolchain. Buck2 builds host code only for
        # the host it runs on, so its standard library also serves the
        # target platform.
        "release": attrs.exec_dep(providers = [RustReleaseInfo]),
        # A directory with the custom target specification.
        "rust_target_path": attrs.option(attrs.dep(), default = None),
        "rustc_binary_flags": attrs.list(attrs.arg(), default = []),
        "rustc_flags": attrs.list(attrs.arg(), default = []),
        # Unset: the host triple of `release`.
        "rustc_target_triple": attrs.option(attrs.string(), default = None),
        # The standard library crates. Each label must name a target that
        # transitions into the `sysroot-none` configuration.
        "sysroot_core": attrs.option(attrs.dep(), default = None),
        "sysroot_others": attrs.list(attrs.dep(), default = []),
        "sysroot_panic_abort": attrs.option(attrs.dep(), default = None),
        "sysroot_proc_macro": attrs.option(attrs.dep(), default = None),
        "sysroot_std": attrs.option(attrs.dep(), default = None),
    },
    is_toolchain_rule = True,
)
