use std::sync::Arc;

use schoolsoft::{Client, ClientBuilder};
use tauri::async_runtime::Mutex;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = ClientBuilder::new().build();
    let schools = Schools::default();

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(client)))
        .manage(Arc::new(Mutex::new(schools)))
        .invoke_handler(tauri::generate_handler![
            greet,
            schools_refresh,
            schools_filter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
