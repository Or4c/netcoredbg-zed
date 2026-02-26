use zed_extension_api::{
    self as zed, DebugAdapterBinary, DebugConfig, DebugScenario, DebugTaskDefinition,
    StartDebuggingRequestArgumentsRequest, Worktree, serde_json::Value,
};

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

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        todo!()
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        todo!()
    }

    fn dap_config_to_scenario(
        &mut self,
        _adapter_name: DebugConfig,
    ) -> Result<DebugScenario, String> {
        todo!();
    }
    // methods...
}

zed::register_extension!(NetCoreDbgExtension);
