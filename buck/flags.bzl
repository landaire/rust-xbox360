"""Code generation settings for Xbox 360 code."""

# Size matters more than speed for code that shares the console's memory
# with the running title.
XBOX360_OPT_LEVEL = "z"

XBOX360_RUSTC_FLAGS = [
    "-Copt-level=" + XBOX360_OPT_LEVEL,
    "-Ccodegen-units=1",
    # Debug information stays in the ELF that a debugger loads; xex-link
    # copies only loaded sections into the XEX.
    "-Cdebuginfo=full",
]

# The standard library crates, as in the Rust release: lines and inlined
# functions without types or variables.
XBOX360_SYSROOT_DEBUGINFO = "-Cdebuginfo=limited"
