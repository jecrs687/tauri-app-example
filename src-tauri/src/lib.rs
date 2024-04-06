#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // your code here
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}
