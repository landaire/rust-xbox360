"""Xbox 360 titles: a Rust or C executable converted to a XEX by xex-link.

`xex_core`, `xex_std` and `xex_std_system` convert the image of any
executable rule.

`xbox360_executable_core`, `xbox360_executable_std` and
`xbox360_executable_std_system` compile a binary crate as one fat LTO code
generation unit, and rustc writes that unit as assembly along with the image
it links. The prelude's `rust_binary` has
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

`xbox360_c_title` compiles C sources with clang and converts the image.

Every rule fails the build if the image holds a reservation that the
kernel loader reports as unprotected.

Every rule's `[elf]` is the image with symbols and debug information, which
xex-link leaves out of the XEX. `[gdb]` and `[lldb]` are command files that
load it into a debugger.
"""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//linking:link_info.bzl", "LinkStrategy")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:build_params.bzl", "MetadataKind")
load("@prelude//rust:link_info.bzl", "RustLinkInfo", "RustProcMacroInfo", "RustProcMacroPlugin", "get_available_proc_macros")
load("@prelude//rust:rust_toolchain.bzl", "RustToolchainInfo")
load(":custom_toolchain.bzl", "Xbox360ClangInfo")
load(":platforms.bzl", "xbox360_core_transition", "xbox360_std_system_transition", "xbox360_std_transition")

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

# Files that a XEX embeds as named resources, which the title finds with
# `XexGetModuleSection`.
XexResourcesInfo = provider(fields = {
    # Resource name (1 to 8 bytes) to the file.
    "files": provider_field(dict[str, Artifact]),
})

def _xex_resources_impl(ctx: AnalysisContext) -> list[Provider]:
    return [DefaultInfo(other_outputs = ctx.attrs.files.values()), XexResourcesInfo(files = ctx.attrs.files)]

# Resources for the `resources` of the XEX rules and `xbox360_title`: a map
# of resource names to files or targets.
xex_resources = rule(
    impl = _xex_resources_impl,
    attrs = {"files": attrs.dict(attrs.string(), attrs.source())},
)

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
    if ctx.attrs.resources:
        for name, src in ctx.attrs.resources[XexResourcesInfo].files.items():
            cmd.add("--resource", cmd_args(name, "=", src, delimiter = ""))
    ctx.actions.run(cmd, category = "xex_link")
    return xex

def _debugger_files(ctx: AnalysisContext, elf: Artifact, library: Dependency | None) -> dict[str, list[Provider]]:
    """`[gdb]` and `[lldb]`: command files that load `elf` and, with
    `library` (`xbox360//sysroot:library`), find the standard library
    source. Paths are relative to the project root, as are the source paths
    in the debug information, so the debugger runs there."""
    gdb = [cmd_args("file ", elf, delimiter = "")]
    lldb = [cmd_args("target create ", elf, delimiter = "")]
    outputs = [elf]

    if library:
        source = library[DefaultInfo].default_outputs[0]

        # rustc records the library sources under the package of the
        # sysroot crates, without the leading `./` of the remapped prefix.
        recorded = cmd_args(library.label.path, "/library", delimiter = "", replace_regex = ("^\\./", ""))
        gdb.append(cmd_args("set substitute-path ", recorded, " ", source, delimiter = ""))
        lldb.append(cmd_args("settings set target.source-map ", recorded, " ", source, delimiter = ""))
        outputs.append(source)

    files = {}
    for name, lines in [("gdb", gdb), ("lldb", lldb)]:
        script, _ = ctx.actions.write(name + ".commands", cmd_args(lines, "", delimiter = "\n"), allow_args = True)
        files[name] = [DefaultInfo(default_output = script, other_outputs = outputs)]

    return files

def _xex_impl(ctx: AnalysisContext) -> list[Provider]:
    elf = ctx.attrs.elf[DefaultInfo].default_outputs[0]
    reservations = _check_reservations(ctx, elf)
    xex = _xex_link(ctx, elf, [reservations])
    return [DefaultInfo(default_output = xex, sub_targets = {
        "elf": [DefaultInfo(default_output = elf)],
        "reservations": [DefaultInfo(default_output = reservations)],
    } | _debugger_files(ctx, elf, ctx.attrs._sysroot_library))]

_XEX_ATTRS = {
    # See xex-link's documentation for the format; without one, xex-link's
    # defaults apply.
    "manifest": attrs.option(attrs.source(), default = None),
    "out": attrs.option(attrs.string(), default = None),
    # An `xex_resources` target: the files that the image embeds by
    # resource name.
    "resources": attrs.option(attrs.dep(providers = [XexResourcesInfo]), default = None),
} | _TOOL_ATTRS

# The standard library source, for the debugger command files of Rust
# titles.
_SYSROOT_LIBRARY_ATTRS = {
    "_sysroot_library": attrs.dep(default = "xbox360//sysroot:library"),
}

# The rule configures itself, and so the executable, for the console. The
# core variant provides core, alloc and compiler_builtins; the std variant
# also std, and the std_system variant the std of system modules, whose
# threads start in the system process and whose sockets open as the system.
xex_core = rule(impl = _xex_impl, attrs = _XEX_ATTRS | _SYSROOT_LIBRARY_ATTRS | {"elf": attrs.dep()}, cfg = xbox360_core_transition)

xex_std = rule(impl = _xex_impl, attrs = _XEX_ATTRS | _SYSROOT_LIBRARY_ATTRS | {"elf": attrs.dep()}, cfg = xbox360_std_transition)

xex_std_system = rule(impl = _xex_impl, attrs = _XEX_ATTRS | _SYSROOT_LIBRARY_ATTRS | {"elf": attrs.dep()}, cfg = xbox360_std_system_transition)

def _xbox360_c_title_impl(ctx: AnalysisContext) -> list[Provider]:
    objects = []
    for src in ctx.attrs.srcs:
        obj = ctx.actions.declare_output("objects", src.short_path + ".o")
        ctx.actions.run(
            cmd_args(
                ctx.attrs._clang[RunInfo],
                "--target=" + ctx.attrs._clang[Xbox360ClangInfo].target_triple,
                # No C library exists for the console.
                "-ffreestanding",
                "-fno-builtin",
                # Debug information stays in the ELF that a debugger loads.
                "-g",
                ctx.attrs.compiler_flags,
                "-c",
                src,
                "-o",
                obj.as_output(),
            ),
            category = "cc",
            identifier = src.short_path,
        )
        objects.append(obj)

    image = ctx.actions.declare_output("image")
    ctx.actions.run(
        cmd_args(
            ctx.attrs._clang[Xbox360ClangInfo].linker,
            "-flavor",
            "gnu",
            objects,
            cmd_args(ctx.attrs.link_script, format = "-T{}"),
            "--no-rosegment",
            "-o",
            image.as_output(),
        ),
        category = "link",
    )

    reservations = _check_reservations(ctx, image)
    xex = _xex_link(ctx, image, [reservations])
    return [DefaultInfo(default_output = xex, sub_targets = {
        "elf": [DefaultInfo(default_output = image)],
        "reservations": [DefaultInfo(default_output = reservations)],
    } | _debugger_files(ctx, image, None))]

# A XEX from C sources. The configured clang compiles them for big-endian
# PowerPC without a C library, and its linker links the objects with the GNU
# ld script `link_script`.
xbox360_c_title = rule(
    impl = _xbox360_c_title_impl,
    attrs = _XEX_ATTRS | {
        "compiler_flags": attrs.list(attrs.string(), default = []),
        "link_script": attrs.source(),
        "srcs": attrs.list(attrs.source()),
        "_clang": attrs.toolchain_dep(
            providers = [RunInfo, Xbox360ClangInfo],
            default = "toolchains//:clang",
        ),
    },
)

def _rlib(dep: Dependency) -> Artifact:
    # Executables for the console link statically (no position-independent
    # code), as with the target specification's relocation model. The
    # `link` rlibs hold the code; `full` ones only metadata.
    return dep[RustLinkInfo].strategies[LinkStrategy("static")].outputs[MetadataKind("link")]

def _dependency_dirs(dep: Dependency) -> cmd_args:
    """`-Ldependency` for the directory of each of `dep`'s transitive rlibs."""
    transitive = dep[RustLinkInfo].strategies[LinkStrategy("static")].transitive_deps[MetadataKind("link")]
    return cmd_args(transitive.project_as_args("artifacts_args"), format = "-Ldependency={}", parent = 1)

def _proc_macro_dirs(ctx: AnalysisContext, deps: list[Dependency]) -> cmd_args:
    """`-Ldependency` for the directory of each proc macro that the transitive
    rlibs use: rustc loads a crate's proc macros to read its metadata."""
    available = get_available_proc_macros(ctx)
    labels = {}
    for dep in deps:
        markers = dep[RustLinkInfo].strategies[LinkStrategy("static")].transitive_proc_macro_deps
        for marker in markers:
            labels[marker] = None
    dylibs = [available[label][RustProcMacroInfo].dylib for label in sorted(labels)]
    return cmd_args(dylibs, format = "-Ldependency={}", parent = 1)

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
            _proc_macro_dirs(ctx, ctx.attrs.deps),
            cmd_args(srcs, format = "--remap-path-prefix={}/=" + ctx.label.package + "/"),
            # Debug information names files relative to the project root,
            # as with the prelude's Rust rules.
            "-Zremap-cwd-prefix=.",
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
    # Proc macros reach the rule as plugins, built for the execution platform.
    "deps": attrs.list(attrs.dep(providers = [RustLinkInfo], pulls_plugins = [RustProcMacroPlugin]), default = []),
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
# alloc and compiler_builtins; the std variant also std, and the std_system
# variant the std of system modules.
xbox360_executable_core = rule(
    impl = _xbox360_executable_impl,
    attrs = _EXECUTABLE_ATTRS,
    cfg = xbox360_core_transition,
    uses_plugins = [RustProcMacroPlugin],
)

xbox360_executable_std = rule(
    impl = _xbox360_executable_impl,
    attrs = _EXECUTABLE_ATTRS,
    cfg = xbox360_std_transition,
    uses_plugins = [RustProcMacroPlugin],
)

xbox360_executable_std_system = rule(
    impl = _xbox360_executable_impl,
    attrs = _EXECUTABLE_ATTRS,
    cfg = xbox360_std_system_transition,
    uses_plugins = [RustProcMacroPlugin],
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
            # The crate has no code of its own; the assembly carries the
            # debug information of the LTO unit.
            "-Cdebuginfo=none",
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

    # xex-link copies only the loaded sections, so the image with symbols
    # and debug information is also the debugger's ELF.
    image = _link(ctx, executable, obj, "image", strip = False)

    if ctx.attrs.rewrite:
        disassembly = ctx.actions.declare_output("disassembly")
        _write_stdout(
            ctx,
            disassembly,
            cmd_args(ctx.attrs._llvm_objdump[RunInfo], "-d", "--no-show-raw-insn", image),
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

    reservations = _check_reservations(ctx, image)
    checks.append(reservations)
    checks.append(_check_link(ctx, executable))

    xex = _xex_link(ctx, image, checks)

    sub_targets["asm"] = [DefaultInfo(default_output = assembly)]
    sub_targets["elf"] = [DefaultInfo(default_output = image)]
    sub_targets["reservations"] = [DefaultInfo(default_output = reservations)]
    sub_targets |= _debugger_files(ctx, image, ctx.attrs._sysroot_library)
    return [DefaultInfo(default_output = xex, sub_targets = sub_targets)]

# A XEX assembled and linked from the assembly of an `xbox360_executable_*`
# target. The rule configures itself for the console; the executable keeps
# its own configuration.
xex_from_assembly = rule(
    impl = _xex_from_assembly_impl,
    cfg = xbox360_core_transition,
    attrs = _XEX_ATTRS | _SYSROOT_LIBRARY_ATTRS | {
        "executable": attrs.dep(providers = [Xbox360ExecutableInfo]),
        "rewrite": attrs.bool(default = True),
        "_compare_files": attrs.source(default = "xbox360//buck:compare_files.py"),
        "_llvm_objdump": attrs.exec_dep(providers = [RunInfo], default = "toolchains//:llvm-objdump"),
        "_rust_lld": attrs.exec_dep(providers = [RunInfo], default = "toolchains//:rust-lld"),
        "_rust_toolchain": toolchains_common.rust(),
    },
)

# The rules of each standard library choice of `xbox360_title`.
_EXECUTABLE_RULES = {
    "core": xbox360_executable_core,
    "std": xbox360_executable_std,
    "std-system": xbox360_executable_std_system,
}

_XEX_RULES = {
    "core": xex_core,
    "std": xex_std,
    "std-system": xex_std_system,
}

def xbox360_title(
        name: str,
        link_script: str,
        manifest: str | None = None,
        resources: dict[str, str] | str | None = None,
        std: bool | str = False,
        rewrite: bool = False,
        rustc_flags: list[str] = [],
        visibility: list[str] = [],
        **kwargs):
    """Builds `name.xex` from a Rust executable.

    `link_script` is the GNU ld script that places the sections; `manifest`
    is the xex-link manifest with the title's XEX metadata. `resources`
    maps resource names (1 to 8 bytes) to the files, or targets, that the
    image embeds under them, or names an `xex_resources` target. `std = True`
    selects the standard library; otherwise the crate is `no_std` and may
    use core and alloc. `std = "system"` selects the standard library of
    system modules: its threads start in the system process, which keeps
    them when the title exits, and its sockets open as the system rather
    than as the title. Other arguments are those of `rust_binary`.

    With `rewrite`, the executable is an `xbox360_executable_*` target,
    which takes the `srcs`, `crate_root`, `edition`, `deps`, `features` and
    `env` of `rust_binary`, and `xex_from_assembly` replaces its
    callee-saved register saves and restores with shared helper routines.
    """
    sysroots = {False: "core", True: "std", "system": "std-system"}
    if std not in sysroots:
        fail("{}: std must be False, True or \"system\", not {}".format(name, repr(std)))
    sysroot = sysroots[std]

    crate = kwargs.pop("crate", name.replace("-", "_"))

    # Applies to every rule of the title, so that a title limited to one
    # code generation (such as VMX128 code on PPC64) is skipped elsewhere.
    compatible_with = kwargs.pop("target_compatible_with", [])
    if type(resources) == type({}):
        xex_resources(
            name = name + "-resources",
            files = resources,
        )
        resources = ":{}-resources".format(name)
    if rewrite:
        _EXECUTABLE_RULES[sysroot](
            name = name + "-executable",
            crate = crate,
            link_script = link_script,
            rustc_flags = rustc_flags,
            target_compatible_with = compatible_with,
            **kwargs
        )
        xex_from_assembly(
            name = name,
            executable = ":{}-executable".format(name),
            manifest = manifest,
            resources = resources,
            target_compatible_with = compatible_with,
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
        target_compatible_with = ["xbox360//platforms:target[xbox]"] + compatible_with,
        **kwargs
    )
    _XEX_RULES[sysroot](
        name = name,
        elf = ":{}-elf".format(name),
        manifest = manifest,
        resources = resources,
        target_compatible_with = compatible_with,
        visibility = visibility,
    )
