use crate::ai::{AIFactory, AIProvider};
use crate::db::Database;
use tauri::State;

#[tauri::command]
pub async fn test_ai_connection(
    db: State<'_, Database>,
) -> Result<serde_json::Value, String> {
    let refiner = AIFactory::create_from_settings(&db).map_err(|e| e.to_string())?;
    let provider_name = refiner.provider_name();

    match refiner.test_connection().await {
        Ok(true) => Ok(serde_json::json!({
            "success": true,
            "provider": provider_name,
            "message": format!("تم الاتصال بنجاح مع {}", provider_name)
        })),
        Ok(false) => Ok(serde_json::json!({
            "success": false,
            "provider": provider_name,
            "message": format!("فشل الاتصال مع {}", provider_name)
        })),
        Err(e) => Ok(serde_json::json!({
            "success": false,
            "provider": provider_name,
            "message": format!("خطأ في الاتصال: {}", e)
        })),
    }
}

#[tauri::command]
pub async fn test_specific_provider(
    provider: String,
    api_key: String,
) -> Result<serde_json::Value, String> {
    let provider_enum = AIProvider::from_str(&provider);
    let api_key_opt = if api_key.is_empty() {
        None
    } else {
        Some(api_key)
    };

    let refiner = AIFactory::create(provider_enum, api_key_opt).map_err(|e| e.to_string())?;
    let provider_name = refiner.provider_name();

    match refiner.test_connection().await {
        Ok(true) => Ok(serde_json::json!({
            "success": true,
            "provider": provider_name,
            "message": format!("تم الاتصال بنجاح مع {}", provider_name)
        })),
        Ok(false) => Ok(serde_json::json!({
            "success": false,
            "provider": provider_name,
            "message": format!("فشل الاتصال مع {}", provider_name)
        })),
        Err(e) => Ok(serde_json::json!({
            "success": false,
            "provider": provider_name,
            "message": format!("خطأ في الاتصال: {}", e)
        })),
    }
}

#[tauri::command]
pub fn get_ai_providers() -> serde_json::Value {
    serde_json::json!([
        {"id": "local", "name": "Local Server", "requires_key": false},
        {"id": "claude", "name": "Claude (Anthropic)", "requires_key": true},
        {"id": "openai", "name": "OpenAI (GPT-4)", "requires_key": true},
        {"id": "gemini", "name": "Google Gemini", "requires_key": true}
    ])
}

#[tauri::command]
pub fn get_current_ai_provider(db: State<'_, Database>) -> Result<serde_json::Value, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;

    let provider = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'ai_provider'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "local".to_string());

    let ai_enabled = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'ai_refinement'",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "false".to_string());

    Ok(serde_json::json!({
        "provider": provider,
        "enabled": ai_enabled == "true"
    }))
}
