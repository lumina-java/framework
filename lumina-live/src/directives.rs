use regex::Regex;
use lumina::core::blade::{BladeContext, BladeEngine};
use crate::component::parse_blade_args_to_json;
use crate::registry::LiveRegistry;
use crate::snapshot::{LiveSnapshot, SnapshotManager};

pub fn register_live_directives() {
    // 1. Script injector: @kilatScripts, @luminaLive, @livewireScripts
    let script_handler = |_args: &str, _engine: &BladeEngine, _ctx: &BladeContext| -> String {
        r#"<script src="/_lumina/live/live.js" defer></script>"#.to_string()
    };

    lumina::core::blade::register_global_directive("kilatScripts", script_handler);
    lumina::core::blade::register_global_directive("luminaLive", script_handler);
    lumina::core::blade::register_global_directive("livewireScripts", script_handler);

    // 2. Component renderer: @kilat('name', args), @live('name', args), @livewire('name', args)
    let comp_handler = |args: &str, engine: &BladeEngine, _ctx: &BladeContext| -> String {
        let re_args = Regex::new(r#"^['"]([^'"]+)['"]\s*(?:,\s*(.*))?$"#).unwrap();
        if let Some(caps) = re_args.captures(args.trim()) {
            let name = &caps[1];
            let registry = LiveRegistry::global();
            if let Some(handler) = registry.get(name) {
                let initial_data = if let Some(args_match) = caps.get(2) {
                    parse_blade_args_to_json(args_match.as_str())
                } else {
                    serde_json::json!({})
                };

                match handler.render_initial(initial_data, engine) {
                    Ok((state_val, html)) => {
                        let id = format!("c-{}", &uuid::Uuid::new_v4().simple().to_string()[..8]);
                        let snapshot = LiveSnapshot::new(&id, name, state_val);
                        let sealed = SnapshotManager::seal(&snapshot).unwrap_or_default();
                        format!(
                            r#"<div data-kilat-id="{}" data-kilat-name="{}" data-kilat-snapshot="{}" data-live-id="{}" data-live-name="{}" data-live-snapshot="{}">{}</div>"#,
                            id, name, sealed, id, name, sealed, html
                        )
                    }
                    Err(e) => format!("<!-- Lumina Live Error: {} -->", e),
                }
            } else {
                format!("<!-- Lumina Live Component '{}' not registered -->", name)
            }
        } else {
            format!("<!-- Lumina Live Invalid Syntax: @live({}) -->", args)
        }
    };

    lumina::core::blade::register_global_directive("kilat", comp_handler);
    lumina::core::blade::register_global_directive("live", comp_handler);
    lumina::core::blade::register_global_directive("livewire", comp_handler);
}
