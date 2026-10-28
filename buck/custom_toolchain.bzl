"""The source-built PPC64 compiler and the C compilers for the console."""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load(":rust.bzl", "RustReleaseInfo")

Xbox360ClangInfo = provider(fields = {
    "target_triple": provider_field(str),
    # Links the objects that the compiler produces.
    "linker": provider_field(RunInfo),
})

def _bootstrap_dist_impl(ctx: AnalysisContext) -> list[Provider]:
    if ctx.attrs.host_triple == None:
        fail("the PPC64 toolchain builds only on x86_64 Linux hosts; use the default PPC32 toolchain")
    dist = ctx.attrs.dist[DefaultInfo].default_outputs[0]
    return [
        DefaultInfo(default_output = dist),
        RustReleaseInfo(
            sysroot = dist,
            executable_suffix = "",
            host_triple = ctx.attrs.host_triple,
        ),
    ]

# A distribution that rustc_bootstrap builds from source, laid out like a Rust
# release: the compiler or the LLVM installation.
bootstrap_dist = rule(
    impl = _bootstrap_dist_impl,
    attrs = {
        "dist": attrs.exec_dep(),
        "host_triple": attrs.default_only(attrs.option(attrs.string(), default = select({
            "DEFAULT": None,
            "prelude//os:linux": select({
                "DEFAULT": None,
                "prelude//cpu:x86_64": "x86_64-unknown-linux-gnu",
            }),
        }))),
    },
)

def _path_clang_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        RunInfo(args = cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._runner,
        )),
    ]

# clang from the PATH of the Buck2 daemon.
path_clang = rule(
    impl = _path_clang_impl,
    attrs = {
        "_python_bootstrap_toolchain": toolchains_common.python_bootstrap(),
        "_runner": attrs.source(default = "xbox360//buck:run_clang.py"),
    },
)

def _xbox360_clang_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        ctx.attrs.tool[RunInfo],
        Xbox360ClangInfo(
            target_triple = ctx.attrs.target_triple,
            linker = ctx.attrs.linker[RunInfo],
        ),
    ]

# The C compiler and linker of console titles.
xbox360_clang = rule(
    impl = _xbox360_clang_impl,
    attrs = {
        "linker": attrs.exec_dep(providers = [RunInfo]),
        "target_triple": attrs.string(),
        "tool": attrs.exec_dep(providers = [RunInfo]),
    },
    is_toolchain_rule = True,
)
