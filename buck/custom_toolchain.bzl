"""Optional source-built LLVM and Rust compiler toolchain."""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load(":rust.bzl", "RustReleaseInfo")

Xbox360ClangInfo = provider(fields = {
    "target_triple": provider_field(str),
})

def _buck_llvm_impl(ctx: AnalysisContext) -> list[Provider]:
    llvm = ctx.attrs.llvm[DefaultInfo].default_outputs[0]
    return [
        DefaultInfo(default_output = llvm),
        RustReleaseInfo(
            sysroot = llvm,
            executable_suffix = "",
            host_triple = "x86_64-unknown-linux-gnu",
        ),
    ]


buck_llvm = rule(
    impl = _buck_llvm_impl,
    attrs = {
        "llvm": attrs.exec_dep(),
    },
)

def _buck_rust_impl(ctx: AnalysisContext) -> list[Provider]:
    dist = ctx.attrs.dist[DefaultInfo].default_outputs[0]
    return [
        DefaultInfo(default_output = dist),
        RustReleaseInfo(
            sysroot = dist,
            executable_suffix = "",
            host_triple = "x86_64-unknown-linux-gnu",
        ),
    ]


buck_rust = rule(
    impl = _buck_rust_impl,
    attrs = {
        "dist": attrs.exec_dep(),
    },
)


def _path_clang_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        RunInfo(args = cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._runner,
        )),
        Xbox360ClangInfo(target_triple = ctx.attrs.target_triple),
    ]


path_clang = rule(
    impl = _path_clang_impl,
    attrs = {
        "_python_bootstrap_toolchain": toolchains_common.python_bootstrap(),
        "target_triple": attrs.string(default = "powerpc-unknown-none-eabi"),
        "_runner": attrs.source(default = "xbox360//buck:run_clang.py"),
    },
)


def _xbox360_clang_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        ctx.attrs.tool[RunInfo],
        Xbox360ClangInfo(target_triple = ctx.attrs.target_triple),
    ]


xbox360_clang = rule(
    impl = _xbox360_clang_impl,
    attrs = {
        "target_triple": attrs.string(),
        "tool": attrs.exec_dep(providers = [RunInfo]),
    },
)
