use obs_wrapper::{
    // Macro for registering modules
    obs_register_module,
    // Macro for creating strings
    obs_string,
    // Everything required for modules
    prelude::*,
    // Everything required for creating a source
    source::*,
};

pub mod native_shims;

/// The state of the source that is managed by OBS and used in each trait method.
struct SourceData;

/// Screen Cast Source
///
/// The struct that represents our source.
struct ScreenCastSource;

impl Sourceable for ScreenCastSource {
    fn get_id() -> ObsString {
        obs_string!("portal_screencast_source")
    }

    fn get_type() -> SourceType {
        SourceType::INPUT
    }
}

impl GetNameSource<SourceData> for ScreenCastSource {
    fn get_name() -> ObsString {
        obs_string!("Portal ScreenCast")
    }
}

/// Screen Cast OBS Module
///
/// This is a wrapper around our OBS module. Used to register our source type.
#[repr(transparent)] 
struct PortalScreenCastModule(ModuleContext);

impl Module for PortalScreenCastModule {
    fn new(context: ModuleContext) -> Self {
        Self(context)
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.0
    }

    /// Module Load Callback
    ///
    /// Called by OBS when the module is loaded. We register our source type
    /// with OBS here.
    fn load(&mut self, load_context: &mut LoadContext) -> bool {

        let source = load_context
            .create_source_builder::<ScreenCastSource, SourceData>()
            .enable_get_name()
            .build();

        load_context.register_source(source);

        true
    }

    fn description() -> ObsString {
        obs_string!("Access to the ScreenCast portal to capture windows and monitors.")
    }

    fn name() -> ObsString {
        obs_string!("Portal ScreenCast Module")
    }

    fn author() -> ObsString {
        obs_string!(env!("CARGO_PKG_AUTHORS"))
    }
}

obs_register_module!(PortalScreenCastModule);
