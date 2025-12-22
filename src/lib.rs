use zed_extension_api as zed;

struct PolarExtension {}

impl zed::Extension for PolarExtension {
    fn new() -> Self {
        PolarExtension {}
    }
}

zed::register_extension!(PolarExtension);
