use std::sync::Arc;
use tauri::Runtime as TauriRuntime;
use tauri::http::{header, Response, StatusCode};
use tauri::plugin::{ Builder as PluginBuilder, TauriPlugin };
   




fn get_component_config() -> super::types::Config {
    let bytes = std::fs::read_to_string(std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tauri-plugin-icons.json")).unwrap();
    let config: super::types::Config = serde_json::from_str(&bytes).unwrap();
    config
}


fn load_bytes(bytes: &'static [u8]) -> std::io::Result<super::types::IconCollection> {
    let data: super::types::IconCollection = bincode::deserialize(&bytes[..]).unwrap();
    Ok(data)
}


fn get_icon(mut path: &str, icons: &super::types::IconCollection) -> Option<Vec<u8>> {
        if path.starts_with("/") {
            path = path.strip_prefix("/").unwrap();
        }
        if path.starts_with("./") {
            path = path.strip_prefix("./").unwrap();
        }
        if path.ends_with(".svg") {
            path = path.strip_suffix(".svg").unwrap();
        }
        for (key, file) in icons {
            if path == key {
                return Some(file.2.clone());
            }
        }
        None
}








pub fn init<R: TauriRuntime>(bytes: &'static [u8]) -> TauriPlugin<R> {

    let config = get_component_config();
    let icons = Arc::new(load_bytes(bytes).unwrap());
    let mut js_script = include_str!("../components/element.js").to_string();

    js_script = js_script.replace("[UI_NAME]", &config.component.ui_name);
    js_script = js_script.replace("[UI_SIZE]", &config.component.ui_size);
    js_script = js_script.replace("[UI_COLOR]", &config.component.ui_color);
    js_script = js_script.replace("[COMPONENT_NAME]", &config.component.name);

    PluginBuilder::<R>::new("uiicons")
        .invoke_handler(tauri::generate_handler![])
        .js_init_script(js_script)
        .register_uri_scheme_protocol("uiicons", move |_app, request| {
            if let Some(icon) = get_icon(request.uri().path(), icons.as_ref()) {
                return response(
                    StatusCode::OK, 
                    "image/svg+xml", 
                    icon.to_vec()
                );
            }
            return response(
                StatusCode::OK, 
                "image/svg+xml", 
                get_icon("internal-error-404", &icons).unwrap().to_vec()
            );
        })
    .build()
}




fn response(status : StatusCode, mime : &str, data : Vec<u8>) -> Response<Vec<u8>> {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .status(status)
        .header(header::CONTENT_TYPE, mime)
        .body(data)
        .unwrap() 
}