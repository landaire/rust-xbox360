"""Configuration transitions into the Xbox 360 target."""

_REFS = {
    "console": "xbox360//platforms:console",
    "cpu": "prelude//cpu/constraints:cpu",
    "os": "prelude//os/constraints:os",
    "sysroot": "xbox360//platforms:sysroot",
    "sysroot_core": "xbox360//platforms:sysroot-core",
    "sysroot_none": "xbox360//platforms:sysroot-none",
    "sysroot_std": "xbox360//platforms:sysroot-std",
    "xbox360": "xbox360//platforms:xbox360",
}

def _with_constraints(platform: PlatformInfo, label: str, remove: list, add: list) -> PlatformInfo:
    constraints = {
        setting: value
        for setting, value in platform.configuration.constraints.items()
        if setting not in [r[ConstraintSettingInfo].label for r in remove]
    }
    for value in add:
        info = value[ConstraintValueInfo]
        constraints[info.setting.label] = info
    return PlatformInfo(
        label = label,
        configuration = ConfigurationInfo(
            constraints = constraints,
            values = platform.configuration.values,
        ),
    )

def _title(platform: PlatformInfo, refs: struct, sysroot, label: str) -> PlatformInfo:
    # The host os and cpu are removed so that selects written for host
    # platforms (for example in generated third-party rules) fall back to
    # their defaults.
    return _with_constraints(
        platform,
        label,
        remove = [refs.os, refs.cpu],
        add = [refs.xbox360, sysroot],
    )

def _core_impl(platform: PlatformInfo, refs: struct) -> PlatformInfo:
    return _title(platform, refs, refs.sysroot_core, "xbox360-core")

def _std_impl(platform: PlatformInfo, refs: struct) -> PlatformInfo:
    return _title(platform, refs, refs.sysroot_std, "xbox360-std")

xbox360_core_transition = transition(impl = _core_impl, refs = _REFS)

xbox360_std_transition = transition(impl = _std_impl, refs = _REFS)

def _sysroot_build_transition_impl(ctx: AnalysisContext) -> list[Provider]:
    none = ctx.attrs.sysroot_none[ConstraintValueInfo]

    def impl(platform: PlatformInfo) -> PlatformInfo:
        constraints = dict(platform.configuration.constraints)
        constraints[none.setting.label] = none
        return PlatformInfo(
            label = "xbox360-sysroot",
            configuration = ConfigurationInfo(
                constraints = constraints,
                values = platform.configuration.values,
            ),
        )

    return [DefaultInfo(), TransitionInfo(impl = impl)]

# Configures the standard library crates, which the toolchain compiles
# without a sysroot. Toolchain rules cannot transition their own deps, so
# the toolchain reaches the crates through `transition_alias` targets that
# use this as their incoming transition.
sysroot_build_transition = rule(
    impl = _sysroot_build_transition_impl,
    attrs = {
        "sysroot_none": attrs.dep(providers = [ConstraintValueInfo]),
    },
    is_configuration_rule = True,
)
