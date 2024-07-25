use hyprland::{
    data::Clients,
    shared::HyprData,
    dispatch,
    dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial, WindowIdentifier}
};
use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<String>>();
    let Some(application_name) = arguments.get(1) else {
        return;
    };
    let Ok(clients) = Clients::get() else {
        return;
    };
    let Some(target_client) = clients.iter().find(|client| client.initial_class.eq(application_name)) else {
        return;
    };
    let workspace_id = WorkspaceIdentifierWithSpecial::Id(target_client.workspace.id);
    _ = dispatch!(Workspace, workspace_id);

    let window_id = WindowIdentifier::ProcessId(target_client.pid as u32);
    _ = dispatch!(FocusWindow, window_id);
}
