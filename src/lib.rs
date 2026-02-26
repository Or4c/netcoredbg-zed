use zed_extension_api as zed;

struct NetCoreDbgExtension {
    // state...
}

impl zed::Extension for NetCoreDbgExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        NetCoreDbgExtension {}
    }
    // methods...
}

zed::register_extension!(NetCoreDbgExtension);
