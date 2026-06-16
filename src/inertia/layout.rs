use axum_inertia::InertiaConfig;
use sailfish::TemplateSimple;

use crate::config::AppConfig;

/// Sailfish template untuk root HTML Inertia (first page load).
#[derive(TemplateSimple)]
#[template(path = "root.stpl")]
struct RootTemplate<'a> {
    page_json: &'a str,
    asset_url: &'a str,
    title: &'a str,
    dev_mode: bool,
}

/// Build InertiaConfig dengan layout function dari Sailfish.
pub fn build(config: &AppConfig) -> InertiaConfig {
    let asset_url = if config.dev_mode {
        config.vite_dev_url.clone()
    } else {
        String::new()
    };
    let dev_mode = config.dev_mode;

    let layout = move |page_json: String| -> String {
        let template = RootTemplate {
            page_json: &page_json,
            asset_url: &asset_url,
            title: "Laju Rust",
            dev_mode,
        };
        template.render_once().unwrap_or_else(|e| {
            format!("Template error: {}", e)
        })
    };

    let version = if dev_mode {
        Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        )
    } else {
        None
    };

    InertiaConfig::new(version, Box::new(layout))
}
