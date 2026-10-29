"""Rust toolchain built from the archives pinned in rust_release.bzl."""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:rust_toolchain.bzl", "PanicRuntime", "RustExplicitSysrootDeps", "RustToolchainInfo")
load(":rust_release.bzl", "RUST_HOSTS")

# The components of rust_release.bzl that the Rust toolchain runs: rustc and
# rustdoc, the host standard library, and clippy-driver.
RUST_TOOLCHAIN_COMPONENTS = ["rustc", "rust-std", "clippy-preview"]

RustReleaseInfo = provider(fields = {
    # The component archives in one directory, laid out like a rustup
    # toolchain.
    "sysroot": provider_field(Artifact),
    "executable_suffix": provider_field(str),
    "host_triple": provider_field(str),
})

def _select_host(hosts: dict, field: typing.Callable) -> Select:
    """Selects `field(host)` for the configured host."""
    return select({
        "prelude//os:" + os_name: select({
            "prelude//cpu:" + cpu: field(host)
            for cpu, host in cpus.items()
        })
        for os_name, cpus in hosts.items()
    })

def _rust_release_archives(name: str, components: list[str], hosts: dict) -> list[str]:
    """Declares an `http_file` per component and returns their labels."""
    labels = []
    for component in components:
        archive = "{}-{}-archive".format(name, component)
        native.http_file(
            name = archive,
            urls = [_select_host(hosts, lambda host: host["archives"][component]["url"])],
            sha256 = _select_host(hosts, lambda host: host["archives"][component]["sha256"]),
            out = "{}.tar.xz".format(component),
        )
        labels.append(":" + archive)
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

def rust_release(
        name: str,
        components: list[str] = RUST_TOOLCHAIN_COMPONENTS,
        hosts: dict = RUST_HOSTS,
        visibility: list[str] = []):
    """Installs `components` of the host's entry in `hosts` into one directory.

    `hosts` has the layout of RUST_HOSTS in rust_release.bzl.
    """
    _rust_release(
        name = name,
        archives = _rust_release_archives(name, components, hosts),
        executable_suffix = select({
            "DEFAULT": "",
            "prelude//os:windows": ".exe",
        }),
        host_triple = _select_host(hosts, lambda host: host["triple"]),
        visibility = visibility,
    )

def _release_tool_impl(ctx: AnalysisContext) -> list[Provider]:
    release = ctx.attrs.release[RustReleaseInfo]
    path = ctx.attrs.path.format(host_triple = release.host_triple)
    return [
        DefaultInfo(),
        RunInfo(args = cmd_args(
            cmd_args(release.sysroot, format = "{}/" + path + release.executable_suffix),
            ctx.attrs.args,
        )),
    ]

# An executable of a rust_release, to run from rules or with `buck2 run`.
release_tool = rule(
    impl = _release_tool_impl,
    attrs = {
        # Arguments before those of the caller.
        "args": attrs.list(attrs.string(), default = []),
        # Relative to the release directory, without the executable suffix;
        # `{host_triple}` stands for the release's host triple.
        "path": attrs.string(),
        "release": attrs.exec_dep(providers = [RustReleaseInfo]),
    },
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
            rustdoc_flags = ctx.attrs.rustdoc_flags,
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
        # Also passed to tests, which the prelude builds as binaries.
        "rustc_binary_flags": attrs.list(attrs.arg(), default = []),
        "rustc_flags": attrs.list(attrs.arg(), default = []),
        "rustdoc_flags": attrs.list(attrs.arg(), default = []),
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
