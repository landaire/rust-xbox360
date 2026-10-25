"""The Rust standard library crates for the Xbox 360 target."""

load("@prelude//decls:toolchains_common.bzl", "toolchains_common")
load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")
load("@prelude//rust:cargo_buildscript.bzl", "buildscript_run")
load("@prelude//rust/buildscript:buildscript_platform.bzl", "transition_alias")
load("//buck:flags.bzl", "XBOX360_OPT_LEVEL", "XBOX360_SYSROOT_DEBUGINFO")
load("//sysroot:crates.bzl", "SYSROOT_CRATES")

# Every standard library crate is compiled like `cargo -Zbuild-std` does.
# These flags follow the console flags of the toolchain.
_RUSTC_FLAGS = [
    "-Zforce-unstable-if-unmarked",
    "--cap-lints=allow",
]

def _sysroot_debug_info():
    return select({
        "//platforms:mode[release-no-symbols]": ["-Cdebuginfo=none"],
        "DEFAULT": [XBOX360_SYSROOT_DEBUGINFO],
    })

# Crates built a second time with extra rustc flags, by target name. The
# standard library of system modules starts its threads in the system
# process and opens its sockets as the system.
_VARIANTS = {
    "std-system-0.0.0": struct(crate = "std@0.0.0", rustc_flags = ["--cfg=xbox360_system"]),
}

# The crates that the Rust toolchain passes to other crates, by target name.
SYSROOT_EXPORTS = {
    "alloc": "alloc-0.0.0",
    "compiler_builtins": "compiler_builtins-0.1.160",
    "core": "core-0.0.0",
    "panic_abort": "panic_abort-0.0.0",
    "std": "std-0.0.0",
    "std-system": "std-system-0.0.0",
}

def _rust_library_source_impl(ctx: AnalysisContext) -> list[Provider]:
    library = ctx.actions.declare_output("library", dir = True)
    ctx.actions.run(
        cmd_args(
            ctx.attrs._python_bootstrap_toolchain[PythonBootstrapToolchainInfo].interpreter,
            ctx.attrs._assembler,
            library.as_output(),
            ctx.attrs.archive[DefaultInfo].default_outputs[0],
            ctx.attrs.overlay[DefaultInfo].default_outputs[0],
        ),
        category = "rust_library_source",
    )
    return [DefaultInfo(
        default_output = library,
        sub_targets = {
            directory: [DefaultInfo(default_output = library.project(directory))]
            for directory in ctx.attrs.package_dirs
        },
    )]

# The `library` directory of rust-src with the overlay applied. Each entry
# of `package_dirs` is also a subtarget for that package's directory.
rust_library_source = rule(
    impl = _rust_library_source_impl,
    attrs = {
        "archive": attrs.dep(),
        "overlay": attrs.dep(),
        "package_dirs": attrs.list(attrs.string(), default = []),
        "_assembler": attrs.source(default = "xbox360//buck:assemble_library_source.py"),
        "_python_bootstrap_toolchain": toolchains_common.python_bootstrap(),
    },
)

def _target_name(key: str) -> str:
    return key.replace("@", "-")

def _package_dir(crate: dict) -> str:
    """Directory of a library package, relative to the library directory."""
    root = crate["build_script"]["root"] if crate["build_script"] else crate["root"]
    return root.rpartition("/src/")[0] if "/src/" in root else root.rpartition("/")[0]

def sysroot_crates(library_source: str):
    """Declares a target per crate in SYSROOT_CRATES, its variants, and public aliases.

    `library_source` is a rust_library_source target with a subtarget for
    each library package that has a build script.
    """
    libraries = {}
    for key, crate in SYSROOT_CRATES.items():
        name = _target_name(key)
        if crate["registry_sha256"]:
            archive = name + "-crate"
            native.http_archive(
                name = archive,
                sha256 = crate["registry_sha256"],
                strip_prefix = "{}-{}".format(crate["name"], crate["version"]),
                type = "tar.gz",
                urls = ["https://static.crates.io/crates/{0}/{0}-{1}.crate".format(crate["name"], crate["version"])],
            )
            srcs = [":" + archive]
            prefix = archive + "/"
            manifest_dir = ":" + archive
        else:
            srcs = [library_source]
            prefix = "library/"
            manifest_dir = "{}[{}]".format(library_source, _package_dir(crate))

        rustc_flags = list(_RUSTC_FLAGS)
        env = {}
        if crate["build_script"]:
            native.rust_binary(
                name = name + "-build-script-build",
                srcs = srcs,
                crate = "build_script_build",
                crate_root = prefix + crate["build_script"]["root"],
                edition = crate["build_script"]["edition"],
                features = crate["features"],
                rustc_flags = ["--cap-lints=allow"],
            )
            buildscript_run(
                name = name + "-build-script-run",
                buildscript_rule = ":{}-build-script-build".format(name),
                # The prelude sets OPT_LEVEL only for numeric levels.
                env = {"OPT_LEVEL": XBOX360_OPT_LEVEL},
                features = crate["features"],
                manifest_dir = manifest_dir,
                package_name = crate["name"],
                version = crate["version"],
            )
            rustc_flags.append("@$(location :{}-build-script-run[rustc_flags])".format(name))
            env["OUT_DIR"] = "$(location :{}-build-script-run[out_dir])".format(name)

        libraries[key] = dict(
            srcs = srcs,
            crate = crate["crate"],
            crate_root = prefix + crate["root"],
            edition = crate["edition"],
            env = env,
            features = crate["features"],
            named_deps = {
                extern: ":" + _target_name(dep)
                for extern, dep in crate["deps"].items()
            },
            rustc_flags = rustc_flags + _sysroot_debug_info(),
            # Skipped by host builds such as `buck2 build xbox360//...`.
            target_compatible_with = ["//platforms:target[xbox]"],
        )
        native.rust_library(name = name, **libraries[key])

    # A variant shares the build script run of its crate.
    for name, variant in _VARIANTS.items():
        library = libraries[variant.crate]
        native.rust_library(
            name = name,
            **(library | {"rustc_flags": library["rustc_flags"] + variant.rustc_flags})
        )

    for alias, name in SYSROOT_EXPORTS.items():
        transition_alias(
            name = alias,
            actual = ":" + name,
            incoming_transition = "//platforms:sysroot-build-transition",
            visibility = ["PUBLIC"],
        )

def sysroot_package_dirs() -> list[str]:
    """Library package directories that need a subtarget of the source."""
    return [
        _package_dir(crate)
        for crate in SYSROOT_CRATES.values()
        if crate["build_script"] and not crate["registry_sha256"]
    ]
