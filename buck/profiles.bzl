"""Build-mode settings shared by projects that use rust-xbox360."""

load("@prelude//rules.bzl", "clone_rule")
load("@prelude//dist:dist_info.bzl", "DistInfo")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:rust_binary.bzl", "rust_binary_impl")

_DEV = "xbox360//platforms:mode[dev]"
_RELEASE = "xbox360//platforms:mode[release]"
_RELEASE_NO_SYMBOLS = "xbox360//platforms:mode[release-no-symbols]"


def profile_rustc_flags():
    """Host rustc flags for the selected build mode."""
    return select({
        _DEV: ["-Copt-level=0", "-Cdebuginfo=full"],
        _RELEASE: ["-Copt-level=3", "-Cdebuginfo=full"],
        _RELEASE_NO_SYMBOLS: ["-Copt-level=3", "-Cdebuginfo=none", "-Cstrip=symbols"],
        "DEFAULT": ["-Copt-level=0", "-Cdebuginfo=full"],
    })


def profile_rustc_binary_flags():
    """Host binary and test flags for the selected build mode."""
    return select({
        _RELEASE: ["-Clto=thin"],
        _RELEASE_NO_SYMBOLS: ["-Clto=thin"],
        "DEFAULT": [],
    })


def profile_xbox360_rustc_flags():
    """Console overrides for the selected build mode."""
    return select({
        _RELEASE_NO_SYMBOLS: ["-Cdebuginfo=none", "-Cstrip=symbols"],
        "DEFAULT": [],
    })


def profile_separate_debug_info():
    """Whether host executables emit an external symbol file."""
    return select({
        _RELEASE_NO_SYMBOLS: False,
        "DEFAULT": True,
    })


def _subtarget_providers(collection):
    providers = []
    for provider_type in [DefaultInfo, RunInfo]:
        provider = collection.get(provider_type)
        if provider != None:
            providers.append(provider)
    return providers


def _profiled_rust_binary_impl(ctx: AnalysisContext) -> list[Provider]:
    providers = rust_binary_impl(ctx)
    if not ctx.attrs.profile_split_debug_info or not ctx.attrs.profile_is_elf:
        return providers

    binary_info = [provider for provider in providers if isinstance(provider, DefaultInfo)][0]
    binary = binary_info.default_outputs[0]
    name = binary.short_path[binary.short_path.rfind("/") + 1:]
    dist = binary_info.sub_targets.get("dist")
    original_bundle = dist.get(DefaultInfo).default_outputs[0] if dist != None else None
    bundle = ctx.actions.declare_output("stripped", dir = True, has_content_based_path = True)
    output = bundle.project(name)
    symbols = ctx.actions.declare_output("symbols/" + name + ".debug", has_content_based_path = True)
    command = cmd_args(
        ctx.attrs._profile_python_bootstrap[PythonBootstrapToolchainInfo].interpreter,
        ctx.attrs._profile_split_debug,
        ctx.attrs._profile_llvm_objcopy[RunInfo].args,
        "--profile-objcopy-end",
        binary,
        bundle.as_output(),
        name,
        symbols.as_output(),
    )
    if original_bundle != None:
        command.add(original_bundle)
    ctx.actions.run(command, category = "split_debug_info")

    other_outputs = [
        bundle if original_bundle != None and artifact == original_bundle else artifact
        for artifact in binary_info.other_outputs
    ]
    result = []
    for provider in providers:
        if isinstance(provider, DefaultInfo):
            sub_targets = {
                subtarget_name: (
                    [DefaultInfo(default_output = bundle)]
                    if subtarget_name == "dist"
                    else _subtarget_providers(subtarget)
                )
                for subtarget_name, subtarget in binary_info.sub_targets.items()
            } | {"symbols": [DefaultInfo(default_output = symbols)]}
            result.append(DefaultInfo(
                default_output = output,
                other_outputs = other_outputs,
                sub_targets = sub_targets,
            ))
        elif isinstance(provider, RunInfo):
            result.append(RunInfo(args = cmd_args(output, hidden = [bundle] + other_outputs)))
        elif isinstance(provider, DistInfo) and original_bundle != None:
            result.append(DistInfo(
                shared_libs = provider.shared_libs,
                nondebug_runtime_files = [
                    bundle if artifact == original_bundle else artifact
                    for artifact in provider.nondebug_runtime_files
                ],
                relocatable_resources_contents = provider.relocatable_resources_contents,
                relocatable_resources_json = provider.relocatable_resources_json,
            ))
        else:
            result.append(provider)
    return result


_profiled_rust_binary = clone_rule(
    "rust_binary",
    extra_attrs = {
        "profile_is_elf": attrs.bool(),
        "profile_split_debug_info": attrs.bool(),
        "_profile_python_bootstrap": attrs.toolchain_dep(default = "toolchains//:python_bootstrap", providers = [PythonBootstrapToolchainInfo]),
        "_profile_llvm_objcopy": attrs.exec_dep(default = "toolchains//:llvm-objcopy", providers = [RunInfo]),
        "_profile_split_debug": attrs.source(default = "//buck:split_debug.py"),
    },
    impl_override = _profiled_rust_binary_impl,
)


def profiled_rust_binary(**kwargs):
    """A rust_binary with an external ELF symbol file by default."""
    _profiled_rust_binary(
        profile_is_elf = select({
            "prelude//os:linux": True,
            "DEFAULT": False,
        }),
        profile_split_debug_info = profile_separate_debug_info(),
        separate_debug_info = profile_separate_debug_info(),
        **kwargs
    )
