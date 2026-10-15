"""C/C++ toolchain pieces for the host and the Xbox 360."""

load("@prelude//cxx:cxx_toolchain_types.bzl", "CxxPlatformInfo", "CxxToolchainInfo", "LinkerInfo", "LinkerType")
load("@prelude//toolchains:cxx.bzl", "CxxToolsInfo")
load(":rust.bzl", "RustReleaseInfo")

def _copy(constructor, provider, **changes):
    """Returns `constructor` applied to the fields of `provider`, some replaced."""
    return constructor(**({field: getattr(provider, field) for field in dir(provider)} | changes))

def _linker_override_impl(ctx: AnalysisContext) -> list[Provider]:
    base = ctx.attrs.base
    toolchain = base[CxxToolchainInfo]
    if ctx.attrs.linker:
        toolchain = _copy(
            CxxToolchainInfo,
            toolchain,
            linker_info = _copy(
                LinkerInfo,
                toolchain.linker_info,
                linker = ctx.attrs.linker[RunInfo],
                linker_flags = [],
                post_linker_flags = [],
            ),
        )
    return [
        DefaultInfo(),
        toolchain,
        base[CxxPlatformInfo],
    ]

# `base` with `linker` (when set) as the linker and no linker flags. The
# prelude's cxx_toolchain_override needs Objective-C settings in its base,
# which a cxx_tools_info_toolchain does not have.
linker_override_toolchain = rule(
    impl = _linker_override_impl,
    attrs = {
        "base": attrs.toolchain_dep(providers = [CxxToolchainInfo, CxxPlatformInfo]),
        "linker": attrs.option(attrs.exec_dep(providers = [RunInfo]), default = None),
    },
    is_toolchain_rule = True,
)

def _host_cxx_tools_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        CxxToolsInfo(
            compiler = "clang",
            compiler_type = "clang",
            cxx_compiler = "clang++",
            asm_compiler = "clang",
            asm_compiler_type = "clang",
            rc_compiler = None,
            cvtres_compiler = None,
            archiver = "ar",
            archiver_type = "gnu",
            linker = ctx.attrs.linker,
            linker_type = LinkerType("gnu"),
        ),
    ]

# The prelude's clang tools with another linker driver.
host_cxx_tools = rule(
    impl = _host_cxx_tools_impl,
    attrs = {
        "linker": attrs.string(),
    },
)

def _rust_lld_impl(ctx: AnalysisContext) -> list[Provider]:
    release = ctx.attrs.release[RustReleaseInfo]
    lld = cmd_args(
        release.sysroot,
        format = "{}/lib/rustlib/" + release.host_triple + "/bin/rust-lld" + release.executable_suffix,
    )
    return [DefaultInfo(), RunInfo(args = lld)]

# rust-lld from the Rust release. rustc passes `-flavor gnu` itself because
# the Xbox 360 target specification selects the gnu-lld linker flavor.
rust_lld = rule(
    impl = _rust_lld_impl,
    attrs = {
        "release": attrs.dep(providers = [RustReleaseInfo]),
    },
)
