"""Imports a source-built compiler distribution into the main Buck graph."""


def _local_toolchain_dist_impl(ctx: AnalysisContext) -> list[Provider]:
    if not ctx.attrs.srcs:
        fail("local compiler distribution is missing; run buck/build_custom_toolchain.py")
    dist = ctx.actions.copied_dir("dist", ctx.attrs.srcs)
    return [DefaultInfo(default_output = dist)]


local_toolchain_dist = rule(
    impl = _local_toolchain_dist_impl,
    attrs = {
        "srcs": attrs.dict(key = attrs.string(), value = attrs.source()),
    },
)
