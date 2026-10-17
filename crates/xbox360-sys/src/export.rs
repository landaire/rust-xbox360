/// One `.xexports` record: big-endian ordinal, then the entry address.
/// xex-link turns the records into the XEX export table.
#[repr(C)]
pub struct ExportRecord {
    ordinal: u32,
    entry: *const (),
}

// SAFETY: the record is immutable and only read by the linker.
unsafe impl Sync for ExportRecord {}

impl ExportRecord {
    /// Used by [`export!`](crate::export!) and [`export_raw!`](crate::export_raw!).
    #[doc(hidden)]
    pub const fn new(ordinal: u16, entry: *const ()) -> Self {
        assert!(ordinal != 0, "export ordinals start at 1");

        Self {
            ordinal: ordinal as u32,
            entry,
        }
    }
}

/// Export `$function` (as for [`bridged!`](crate::bridged!)) at `$ordinal`
/// behind [`entry_bridge`](crate::entry_bridge) or `bridge`. Ordinals that
/// share one address are declared once each.
#[macro_export]
macro_rules! export {
    ($ordinal:literal, $function:path, bridge = $bridge:path) => {
        const _: () = {
            $crate::bridged!(entry, $function, bridge = $bridge);

            #[used]
            #[unsafe(link_section = ".xexports")]
            static RECORD: $crate::ExportRecord =
                $crate::ExportRecord::new($ordinal, entry as *const ());
        };
    };
    ($ordinal:literal, $function:path) => {
        $crate::export!($ordinal, $function, bridge = $crate::entry_bridge);
    };
}

/// Export `$entry` at `$ordinal` with no bridge. `$entry` must implement the
/// console calling convention itself, including full 64-bit preservation of
/// any of r14-r31 it changes.
#[macro_export]
macro_rules! export_raw {
    ($ordinal:literal, $entry:path) => {
        const _: () = {
            #[used]
            #[unsafe(link_section = ".xexports")]
            static RECORD: $crate::ExportRecord =
                $crate::ExportRecord::new($ordinal, $entry as *const ());
        };
    };
}
