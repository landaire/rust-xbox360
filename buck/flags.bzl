"""Code generation settings for Xbox 360 code."""

# Size matters more than speed for code that shares the console's memory
# with the running title.
XBOX360_OPT_LEVEL = "z"

XBOX360_RUSTC_FLAGS = [
    "-Copt-level=" + XBOX360_OPT_LEVEL,
    "-Ccodegen-units=1",
]
