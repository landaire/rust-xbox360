"""Xbox 360 titles: a Rust executable converted to a XEX by xex-link.

`xex_core` and `xex_std` convert the image of any executable rule.

`xbox360_executable_core` and `xbox360_executable_std` compile a binary
crate as one fat LTO code generation unit, and rustc writes that unit as
assembly along with the image it links. The prelude's `rust_binary` has
no such output: its `[asm]` subtarget compiles without linking, which
skips LTO. `xex_from_assembly` turns that assembly into a XEX. With
`rewrite`, xbox360-asm-rewrite replaces the callee-saved register saves
and restores in prologues and epilogues with calls to shared helper
routines, and an audit of the linked image's disassembly fails the build
if a rewritable sequence remains. rustc assembles the result with the
integrated assembler of its LLVM, and rust-lld links it with the code that
LTO leaves out (compiler_builtins). The link mirrors the one rustc runs;
the build fails if the mirror, applied to the object rustc linked, does
not give rustc's image.

Every rule fails the build if the image holds a reservation that the
kernel loader reports as unprotected.
"""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//linking:link_info.bzl", "LinkStrategy")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:build_params.bzl", "MetadataKind")
load("@prelude//rust:link_info.bzl", "RustLinkInfo")
load("@prelude//rust:rust_toolchain.bzl", "RustToolchainInfo")
load(":platforms.bzl", "xbox360_core_transition", "xbox360_std_transition")

Xbox360ExecutableInfo = provider(fields = {
    # The LTO code generation unit as assembly.
    "assembly": provider_field(Artifact),
    # The `compiler_builtins` rlib, which LTO leaves out.
    "compiler_builtins": provider_field(Artifact),
    # The image rustc links.
    "image": provider_field(Artifact),
    "link_script": provider_field(Artifact),
    # The LTO code generation unit as the object rustc links.
    "object": provider_field(Artifact),
})

_TOOL_ATTRS = {
    "_asm_rewrite": attrs.exec_dep(
        providers = [RunInfo],
        default = "xbox360//crates/xbox360-asm-rewrite:xbox360-asm-rewrite",
    ),
    "_python_bootstrap_toolchain": toolchains_common.python_bootstrap(),
    "_write_stdout": attrs.source(default = "xbox360//buck:write_stdout.py"),
    "_xex_link": attrs.exec_dep(providers = [RunInfo], default = "xbox360//crates/xex-link:xex-link"),
}

def _write_stdout(ctx: AnalysisContext, output: Artifact, command: cmd_args, category: str):
    ctx.actions.run(
        cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._write_stdout,
            output.as_output(),
            command,
        ),
        category = category,
    )

def _check_reservations(ctx: AnalysisContext, image: Artifact) -> Artifact:
    reservations = ctx.actions.declare_output("reservations")
    _write_stdout(
        ctx,
        reservations,
        cmd_args(ctx.attrs._asm_rewrite[RunInfo], "reservations", image),
        category = "reservations",
    )
    return reservations

def _xex_link(ctx: AnalysisContext, image: Artifact, checks: list[Artifact]) -> Artifact:
    xex = ctx.actions.declare_output(ctx.attrs.out or ctx.label.name + ".xex")
    cmd = cmd_args(
        ctx.attrs._xex_link[RunInfo],
        image,
        xex.as_output(),
        # The XEX exists only if the checks pass.
        hidden = checks,
    )
    if ctx.attrs.manifest:
        cmd.add("--manifest", ctx.attrs.manifest)
    ctx.actions.run(cmd, category = "xex_link")
    return xex

def _xex_impl(ctx: AnalysisContext) -> list[Provider]:
    elf = ctx.attrs.elf[DefaultInfo].default_outputs[0]
    reservations = _check_reservations(ctx, elf)
    xex = _xex_link(ctx, elf, [reservations])
    return [DefaultInfo(default_output = xex, sub_targets = {
        "elf": [DefaultInfo(default_output = elf)],
        "reservations": [DefaultInfo(default_output = reservations)],
    })]

_XEX_ATTRS = {
    # See xex-link's documentation for the format; without one, xex-link's
    # defaults apply.
    "manifest": attrs.option(attrs.source(), default = None),
    "out": attrs.option(attrs.string(), default = None),
} | _TOOL_ATTRS

# The rule configures itself, and so the executable, for the console. The
# core variant provides core, alloc and compiler_builtins; the std variant
# also std.
xex_core = rule(impl = _xex_impl, attrs = _XEX_ATTRS | {"elf": attrs.dep()}, cfg = xbox360_core_transition)

xex_std = rule(impl = _xex_impl, attrs = _XEX_ATTRS | {"elf": attrs.dep()}, cfg = xbox360_std_transition)

def _rlib(dep: Dependency) -> Artifact:
    # Executables for the console link statically (no position-independent
    # code), as with the target specification's relocation model. The
    # `link` rlibs hold the code; `full` ones only metadata.
    return dep[RustLinkInfo].strategies[LinkStrategy("static")].outputs[MetadataKind("link")]

def _dependency_dirs(dep: Dependency) -> cmd_args:
    """`-Ldependency` for the directory of each of `dep`'s transitive rlibs."""
    transitive = dep[RustLinkInfo].strategies[LinkStrategy("static")].transitive_deps[MetadataKind("link")]
    return cmd_args(transitive.project_as_args("artifacts_args"), format = "-Ldependency={}", parent = 1)

def _extern(dep: Dependency, options: str = "") -> cmd_args:
    crate = dep[RustLinkInfo].crate.simple
    return cmd_args(_rlib(dep), format = "--extern=" + options + crate + "={}")

def _check_rustc_flags(flags: list[str]):
    """Fails on flags that change rustc's link: `xex_from_assembly` repeats
    that link with fixed arguments and would no longer reproduce it."""
    for index, flag in enumerate(flags):
        option = flags[index + 1] if flag == "-C" and index + 1 < len(flags) else flag.removeprefix("-C")
        if option.startswith("link") or flag.startswith("-l"):
            fail("rustc_flags may not change the link: `{}`".format(option if option.startswith("link") else flag))

def _xbox360_executable_impl(ctx: AnalysisContext) -> list[Provider]:
    toolchain = ctx.attrs._rust_toolchain[RustToolchainInfo]
    _check_rustc_flags(ctx.attrs.rustc_flags)
    sysroot = toolchain.explicit_sysroot_deps
    compiler_builtins = [dep for dep in sysroot.others if dep[RustLinkInfo].crate.simple == "compiler_builtins"]
    len(compiler_builtins) == 1 or fail("the toolchain provides no compiler_builtins")

    # The options the prelude passes for the standard library crates.
    # `force` links std even into a crate that does not name it.
    sysroot_externs = [_extern(sysroot.core, "noprelude,nounused:")]
    if sysroot.std:
        sysroot_externs.append(_extern(sysroot.std, "noprelude,nounused,force:"))
    if sysroot.panic_abort:
        sysroot_externs.append(_extern(sysroot.panic_abort, "noprelude,nounused:"))
    sysroot_externs.extend([_extern(dep, "noprelude,nounused:") for dep in sysroot.others])
    sysroot_crates = [dep for dep in [sysroot.core, sysroot.std, sysroot.panic_abort] if dep] + sysroot.others

    srcs = ctx.actions.symlinked_dir("srcs", {src.short_path: src for src in ctx.attrs.srcs})
    empty_sysroot = ctx.actions.symlinked_dir("sysroot", {})
    assembly = ctx.actions.declare_output("{}.s".format(ctx.attrs.crate))
    obj = ctx.actions.declare_output("{}.o".format(ctx.attrs.crate))
    image = ctx.actions.declare_output(ctx.attrs.crate)

    ctx.actions.run(
        cmd_args(
            toolchain.compiler,
            cmd_args(srcs, format = "{}/" + ctx.attrs.crate_root),
            "--crate-name=" + ctx.attrs.crate,
            "--crate-type=bin",
            "--edition=" + ctx.attrs.edition,
            "--target=" + toolchain.rustc_target_triple,
            # Only the crates passed with --extern.
            cmd_args(empty_sysroot, format = "--sysroot={}"),
            ['--cfg=feature="{}"'.format(feature) for feature in ctx.attrs.features],
            [_extern(dep) for dep in ctx.attrs.deps],
            sysroot_externs,
            [_dependency_dirs(dep) for dep in ctx.attrs.deps + sysroot_crates],
            cmd_args(srcs, format = "--remap-path-prefix={}/=" + ctx.label.package + "/"),
            cmd_args(ctx.attrs._rust_lld[RunInfo], format = "-Clinker={}"),
            cmd_args(ctx.attrs.link_script, format = "-Clink-arg=-T{}"),
            "-Clink-arg=--no-rosegment",
            "-Cstrip=symbols",
            toolchain.rustc_flags,
            toolchain.rustc_binary_flags,
            ctx.attrs.rustc_flags,
            # The assembly is the whole program only as one fat LTO unit.
            "-Clto=fat",
            "-Ccodegen-units=1",
            cmd_args("--emit=asm=", assembly.as_output(), ",obj=", obj.as_output(), ",link", delimiter = ""),
            "-o",
            image.as_output(),
        ),
        env = ctx.attrs.env | {
            "CARGO_MANIFEST_DIR": srcs,
            # The toolchain's console flags include unstable options.
            "RUSTC_BOOTSTRAP": "1",
            "RUST_TARGET_PATH": toolchain.rust_target_path[DefaultInfo].default_outputs[0],
        },
        category = "rustc",
    )

    return [
        DefaultInfo(default_output = image, sub_targets = {
            "asm": [DefaultInfo(default_output = assembly)],
            "obj": [DefaultInfo(default_output = obj)],
        }),
        Xbox360ExecutableInfo(
            assembly = assembly,
            compiler_builtins = _rlib(compiler_builtins[0]),
            image = image,
            link_script = ctx.attrs.link_script,
            object = obj,
        ),
    ]

_EXECUTABLE_ATTRS = {
    "crate": attrs.string(),
    "crate_root": attrs.string(),
    "deps": attrs.list(attrs.dep(providers = [RustLinkInfo]), default = []),
    "edition": attrs.string(),
    "env": attrs.dict(attrs.string(), attrs.string(), default = {}),
    "features": attrs.list(attrs.string(), default = []),
    "labels": attrs.list(attrs.string(), default = []),
    # GNU ld script that places the sections.
    "link_script": attrs.source(),
    # Flags that change the link are rejected; see `_check_rustc_flags`.
    "rustc_flags": attrs.list(attrs.string(), default = []),
    "srcs": attrs.list(attrs.source(), default = []),
    "_rust_lld": attrs.exec_dep(providers = [RunInfo], default = "toolchains//:rust-lld"),
    "_rust_toolchain": toolchains_common.rust(),
}

# A binary crate for the console. The rules configure themselves for the
# console, so host builds include them. The core variant provides core,
# alloc and compiler_builtins; the std variant also std.
xbox360_executable_core = rule(
    impl = _xbox360_executable_impl,
    attrs = _EXECUTABLE_ATTRS,
    cfg = xbox360_core_transition,
)

xbox360_executable_std = rule(
    impl = _xbox360_executable_impl,
    attrs = _EXECUTABLE_ATTRS,
    cfg = xbox360_std_transition,
)

# Assembles `module.s` with rustc: module-level assembly passes through
# LLVM's assembly parser, as with llvm-mc, for the target specification's
# CPU and features. The crate has no other items, so the object holds only
# the assembly.
_ASSEMBLER_CRATE = """\
#![feature(no_core, rustc_attrs, decl_macro)]
#![allow(internal_features)]
#![no_core]

#[rustc_builtin_macro]
macro global_asm($($arguments:tt)*) {}

#[rustc_builtin_macro]
macro include_str($($arguments:tt)*) {}

global_asm!(include_str!("module.s"), options(raw));
"""

def _assemble(ctx: AnalysisContext, assembly: Artifact) -> Artifact:
    toolchain = ctx.attrs._rust_toolchain[RustToolchainInfo]
    crate = ctx.actions.symlinked_dir("assembler", {
        "module.rs": ctx.actions.write("assembler.rs", _ASSEMBLER_CRATE),
        "module.s": assembly,
    })
    obj = ctx.actions.declare_output("module.o")
    ctx.actions.run(
        cmd_args(
            toolchain.compiler,
            cmd_args(crate, format = "{}/module.rs"),
            "--crate-name=module",
            "--crate-type=lib",
            "--edition=2024",
            "--target=" + toolchain.rustc_target_triple,
            # Custom target specifications are unstable.
            "-Zunstable-options",
            # The CPU and features of the console flags, which the LTO unit
            # was compiled for.
            toolchain.rustc_flags,
            "--emit=obj",
            # rustc writes its intermediate files next to the output, which
            # is private to this action, rather than in the working directory.
            "-o",
            obj.as_output(),
        ),
        env = {
            "RUSTC_BOOTSTRAP": "1",
            "RUST_TARGET_PATH": toolchain.rust_target_path[DefaultInfo].default_outputs[0],
        },
        category = "assemble",
    )
    return obj

def _link(ctx: AnalysisContext, executable: Xbox360ExecutableInfo, obj: Artifact, name: str, strip: bool) -> Artifact:
    image = ctx.actions.declare_output(name)

    # The command rustc runs to link the executable, with `obj` as its code
    # generation unit. `_check_link` confirms that it is.
    ctx.actions.run(
        cmd_args(
            ctx.attrs._rust_lld[RunInfo],
            "-flavor",
            "gnu",
            obj,
            "--as-needed",
            "-Bstatic",
            executable.compiler_builtins,
            "-Bdynamic",
            "-z",
            "noexecstack",
            "-o",
            image.as_output(),
            "--gc-sections",
            ["--strip-all"] if strip else [],
            cmd_args(executable.link_script, format = "-T{}"),
            "--no-rosegment",
        ),
        category = "link",
        identifier = name,
    )
    return image

def _check_link(ctx: AnalysisContext, executable: Xbox360ExecutableInfo) -> Artifact:
    """Links the object rustc linked as `_link` does, and compares the images."""
    image = _link(ctx, executable, executable.object, "rustc-image", strip = True)
    result = ctx.actions.declare_output("link-check")
    ctx.actions.run(
        cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._compare_files,
            result.as_output(),
            executable.image,
            image,
        ),
        category = "link_check",
    )
    return result

def _xex_from_assembly_impl(ctx: AnalysisContext) -> list[Provider]:
    executable = ctx.attrs.executable[Xbox360ExecutableInfo]
    rewrite = ctx.attrs._asm_rewrite[RunInfo]
    assembly = executable.assembly
    sub_targets = {}
    checks = []

    if ctx.attrs.rewrite:
        rewritten = ctx.actions.declare_output("rewritten.s")
        scope = ctx.actions.declare_output("scope")
        refusals = ctx.actions.declare_output("refusals")
        report = ctx.actions.declare_output("rewrite-report")
        _write_stdout(
            ctx,
            report,
            cmd_args(
                rewrite,
                "rewrite",
                assembly,
                rewritten.as_output(),
                "--scope",
                scope.as_output(),
                "--refusals",
                refusals.as_output(),
            ),
            category = "asm_rewrite",
        )
        assembly = rewritten
        sub_targets["refusals"] = [DefaultInfo(default_output = refusals)]
        sub_targets["rewrite-report"] = [DefaultInfo(default_output = report)]

    obj = _assemble(ctx, assembly)
    image = _link(ctx, executable, obj, "image", strip = True)

    if ctx.attrs.rewrite:
        unstripped = _link(ctx, executable, obj, "image.unstripped", strip = False)
        disassembly = ctx.actions.declare_output("disassembly")
        _write_stdout(
            ctx,
            disassembly,
            cmd_args(ctx.attrs._llvm_objdump[RunInfo], "-d", "--no-show-raw-insn", unstripped),
            category = "disassemble",
        )
        audit = ctx.actions.declare_output("audit")
        _write_stdout(
            ctx,
            audit,
            cmd_args(rewrite, "audit", disassembly, "--scope", scope),
            category = "asm_audit",
        )
        checks.append(audit)
        sub_targets["audit"] = [DefaultInfo(default_output = audit)]
        sub_targets["disassembly"] = [DefaultInfo(default_output = disassembly)]
        sub_targets["unstripped"] = [DefaultInfo(default_output = unstripped)]

    reservations = _check_reservations(ctx, image)
    checks.append(reservations)
    checks.append(_check_link(ctx, executable))

    xex = _xex_link(ctx, image, checks)

    sub_targets["asm"] = [DefaultInfo(default_output = assembly)]
    sub_targets["elf"] = [DefaultInfo(default_output = image)]
    sub_targets["reservations"] = [DefaultInfo(default_output = reservations)]
    return [DefaultInfo(default_output = xex, sub_targets = sub_targets)]

# A XEX assembled and linked from the assembly of an `xbox360_executable_*`
# target. The rule configures itself for the console; the executable keeps
# its own configuration.
xex_from_assembly = rule(
    impl = _xex_from_assembly_impl,
    cfg = xbox360_core_transition,
    attrs = _XEX_ATTRS | {
        "executable": attrs.dep(providers = [Xbox360ExecutableInfo]),
        "rewrite": attrs.bool(default = True),
        "_compare_files": attrs.source(default = "xbox360//buck:compare_files.py"),
        "_llvm_objdump": attrs.exec_dep(providers = [RunInfo], default = "toolchains//:llvm-objdump"),
        "_rust_lld": attrs.exec_dep(providers = [RunInfo], default = "toolchains//:rust-lld"),
        "_rust_toolchain": toolchains_common.rust(),
    },
)

def xbox360_title(
        name: str,
        link_script: str,
        manifest: str | None = None,
        std: bool = False,
        rewrite: bool = False,
        rustc_flags: list[str] = [],
        visibility: list[str] = [],
        **kwargs):
    """Builds `name.xex` from a Rust executable.

    `link_script` is the GNU ld script that places the sections; `manifest`
    is the xex-link manifest with the title's XEX metadata. `std` selects
    the standard library; otherwise the crate is `no_std` and may use core
    and alloc. Other arguments are those of `rust_binary`.

    With `rewrite`, the executable is an `xbox360_executable_*` target,
    which takes the `srcs`, `crate_root`, `edition`, `deps`, `features` and
    `env` of `rust_binary`, and `xex_from_assembly` replaces its
    callee-saved register saves and restores with shared helper routines.
    """
    crate = kwargs.pop("crate", name.replace("-", "_"))
    if rewrite:
        (xbox360_executable_std if std else xbox360_executable_core)(
            name = name + "-executable",
            crate = crate,
            link_script = link_script,
            rustc_flags = rustc_flags,
            **kwargs
        )
        xex_from_assembly(
            name = name,
            executable = ":{}-executable".format(name),
            manifest = manifest,
            visibility = visibility,
        )
        return

    native.export_file(
        name = name + "-link-script",
        src = link_script,
    )
    native.rust_binary(
        name = name + "-elf",
        crate = crate,
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
