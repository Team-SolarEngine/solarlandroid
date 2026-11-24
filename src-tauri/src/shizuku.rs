use serde_json::{json, Value};
use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    AppHandle, Manager, Runtime,
};

#[derive(Clone)]
struct Shizuku<R: Runtime>(PluginHandle<R>);

fn call<R: Runtime>(plugin: &PluginHandle<R>, command: &str, payload: Value) -> Result<Value, String> {
    plugin
        .run_mobile_plugin(command, payload)
        .map_err(|e| e.to_string())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("shizuku")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            {
                let handle = api.register_android_plugin(
                    "net.solarengine.solarlanddroid",
                    "ShizukuPlugin",
                )?;
                app.manage(Shizuku(handle));
            }

            #[cfg(not(target_os = "android"))]
            let _ = (app, api);

            Ok(())
        })
        .build()
}

#[tauri::command]
pub fn shizuku_status<R: Runtime>(app: AppHandle<R>) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "status", json!({}))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(json!({
            "binder": false,
            "permissionGranted": false,
            "serviceReady": false,
            "error": "Android only"
        }))
    }
}

#[tauri::command]
pub fn shizuku_request_permission<R: Runtime>(app: AppHandle<R>) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "requestPermission", json!({}))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = app;
        Ok(json!({ "ok": false, "error": "Android only" }))
    }
}

#[tauri::command]
pub fn shizuku_list_folder<R: Runtime>(app: AppHandle<R>, path: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "listFolder", json!({ "path": path }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, path);
        Ok(json!({ "items": [] }))
    }
}

#[tauri::command]
pub fn shizuku_make_dirs<R: Runtime>(app: AppHandle<R>, path: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "makeDirs", json!({ "path": path }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, path);
        Ok(json!({ "ok": true }))
    }
}

#[tauri::command]
pub fn shizuku_rename<R: Runtime>(app: AppHandle<R>, from: String, to: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "rename", json!({ "from": from, "to": to }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, from, to);
        Ok(json!({ "ok": true }))
    }
}

#[tauri::command]
pub fn shizuku_delete<R: Runtime>(app: AppHandle<R>, path: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "delete", json!({ "path": path }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, path);
        Ok(json!({ "ok": true }))
    }
}

#[tauri::command]
pub fn shizuku_unzip<R: Runtime>(app: AppHandle<R>, zip: String, dest: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "unzip", json!({ "zip": zip, "dest": dest }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, zip, dest);
        Ok(json!({ "ok": true }))
    }
}

#[tauri::command]
pub fn shizuku_open_folder<R: Runtime>(app: AppHandle<R>, path: String) -> Result<Value, String> {
    #[cfg(target_os = "android")]
    {
        let plugin = app.state::<Shizuku<R>>();
        call(&plugin.0, "openFolder", json!({ "path": path }))
    }

    #[cfg(not(target_os = "android"))]
    {
        let _ = (app, path);
        Ok(json!({ "ok": true }))
    }
}
