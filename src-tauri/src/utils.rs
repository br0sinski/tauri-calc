#[tauri::command]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tauri::command]
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[tauri::command]
pub fn mult(a: i32, b: i32) -> i32 {
    a * b
}

#[tauri::command]
pub fn div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero not possible".to_string())
    } else {
        Ok(a / b)
    }
}

#[tauri::command]
pub fn square(a: i32, x: u32) -> i32 {
    a.pow(x)
}