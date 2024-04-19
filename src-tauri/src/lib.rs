use std::sync::Arc;

use schoolsoft::{Client, ClientBuilder};
use tauri::{async_runtime::Mutex, AppHandle};
use tauri::Manager;
use tauri_plugin_store::{Store, StoreBuilder};
use types::Schools;

use crate::types::School;

mod types;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Refresh the in memory list of all available schools.
///
/// Should only have to be called once per session.
#[tauri::command]
async fn schools_refresh(
    schools: tauri::State<'_, Arc<Mutex<Schools>>>,
    client: tauri::State<'_, Arc<Mutex<Client>>>,
) -> Result<(), String> {
    let mut schools = schools.lock().await;
    let client = client.lock().await;

    *schools = client
        .schools()
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(School::from)
        .collect();

    Ok(())
}

/// Filter out schools
#[tauri::command]
async fn schools_filter(
    schools: tauri::State<'_, Arc<Mutex<Schools>>>,
    query: String,
    include_unsupported: bool,
) -> Result<Vec<School>, String> {
    let schools = schools.lock().await;

    Ok(schools
        .iter()
        .filter(|school| include_unsupported | school.supported)
        .filter(|school| school.name.to_lowercase().contains(&query.to_lowercase()))
        .cloned()
        .collect())
}

/// Login
#[tauri::command]
async fn login(
    school: String,
    username: String,
    password: String,
    client: tauri::State<'_, Arc<Mutex<Client>>>,
) -> Result<String, String> {
    let mut client = client.lock().await;

    client
        .login(&username, &password, &school)
        .await
        .map_err(|e| e.to_string())?;

    match &client.user {
        Some(u) => Ok(u.name.clone()),
        None => Err("no usr".to_string()),
    }
}

/// check login state
#[tauri::command]
async fn is_logged_in(
    app: AppHandle
) -> Result<bool, ()> {
    let mut store = StoreBuilder::new("./user.bin").build(app.clone());
    store.load();
    Ok(store.has("appkey"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = ClientBuilder::new().build();
    let schools = Schools::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(Arc::new(Mutex::new(client)))
        .manage(Arc::new(Mutex::new(schools)))
        .invoke_handler(tauri::generate_handler![
            is_logged_in,
            login,
            schools_refresh,
            schools_filter,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
