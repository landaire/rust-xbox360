"""Pin for a published prebuilt PPC64 toolchain.

The default branch builds from source. The generated `nightly-toolchain` branch
replaces this file with the manifest of its tested nightly archive.
"""

DEFAULT_PPC64_TOOLCHAIN = "source"

NIGHTLY_TOOLCHAIN = {
    "sha256": "",
    "source_commit": "",
    "strip_prefix": "",
    "url": "",
}
