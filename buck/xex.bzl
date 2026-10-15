"""Xbox 360 titles: a Rust executable converted to a XEX by xex-link."""

load(":platforms.bzl", "xbox360_core_transition", "xbox360_std_transition")

def _xex_impl(ctx: AnalysisContext) -> list[Provider]:
    xex = ctx.actions.declare_output(ctx.attrs.out or ctx.label.name + ".xex")
    elf = ctx.attrs.elf[DefaultInfo].default_outputs[0]
    cmd = cmd_args(ctx.attrs._xex_link[RunInfo], elf, xex.as_output())
    if ctx.attrs.manifest:
        cmd.add("--manifest", ctx.attrs.manifest)
    ctx.actions.run(cmd, category = "xex_link")
    return [DefaultInfo(default_output = xex, sub_targets = {"elf": [DefaultInfo(default_output = elf)]})]

_XEX_ATTRS = {
    "elf": attrs.dep(),
    # See xex-link's documentation for the format; without one, xex-link's
    # defaults apply.
    "manifest": attrs.option(attrs.source(), default = None),
    "out": attrs.option(attrs.string(), default = None),
    "_xex_link": attrs.exec_dep(providers = [RunInfo], default = "xbox360//:xex-link"),
}

# The rule configures itself, and so the executable, for the console. The
# core variant provides core, alloc and compiler_builtins; the std variant
# also std.
xex_core = rule(impl = _xex_impl, attrs = _XEX_ATTRS, cfg = xbox360_core_transition)

xex_std = rule(impl = _xex_impl, attrs = _XEX_ATTRS, cfg = xbox360_std_transition)

def xbox360_title(
        name: str,
        link_script: str,
        manifest: str | None = None,
        std: bool = False,
        rustc_flags: list[str] = [],
        visibility: list[str] = [],
        **kwargs):
    """Builds `name.xex` from a Rust executable.

    Other arguments are those of `rust_binary`. `link_script` is the GNU ld
    script that places the sections; `manifest` is the xex-link manifest
    with the title's XEX metadata. `std` selects the standard library;
    otherwise the crate is `no_std` and may use core and alloc.
    """
    native.export_file(
        name = name + "-link-script",
        src = link_script,
    )
    native.rust_binary(
        name = name + "-elf",
        crate = kwargs.pop("crate", name.replace("-", "_")),
        # A static link strategy keeps the target's static relocation model;
        # the prelude otherwise compiles position-independent code.
        link_style = "static",
        rustc_flags = [
            "-Clink-arg=-T$(location :{}-link-script)".format(name),
            "-Clink-arg=--no-rosegment",
        ] + rustc_flags,
        target_compatible_with = ["xbox360//platforms:xbox360"],
        **kwargs
    )
    (xex_std if std else xex_core)(
        name = name,
        elf = ":{}-elf".format(name),
        manifest = manifest,
        visibility = visibility,
    )
