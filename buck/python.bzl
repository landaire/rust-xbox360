"""Python bootstrap toolchain for builds that target the console."""

load("@prelude//python_bootstrap:python_bootstrap.bzl", "PythonBootstrapToolchainInfo")

def _exec_python_bootstrap_toolchain_impl(ctx: AnalysisContext) -> list[Provider]:
    return [
        DefaultInfo(),
        PythonBootstrapToolchainInfo(interpreter = ctx.attrs.interpreter[RunInfo].args),
    ]

# The prelude's python_bootstrap_toolchain configures its interpreter for
# the target platform, which fails for targets without a host os. The
# interpreter runs on the execution platform, so it is configured there.
exec_python_bootstrap_toolchain = rule(
    impl = _exec_python_bootstrap_toolchain_impl,
    attrs = {
        "interpreter": attrs.exec_dep(providers = [RunInfo]),
    },
    is_toolchain_rule = True,
)
