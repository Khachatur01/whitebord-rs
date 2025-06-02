use standard_tool_plugin::tool::Key;

pub fn from_js_key(key: &str) -> Option<Key> {
    match key {
        "Escape" => Some(Key::Esc),
        "Enter" => Some(Key::Enter),
        "Backspace" => Some(Key::Backspace),
        "Delete" => Some(Key::Delete),
        "ArrowLeft" => Some(Key::ArrowLeft),
        "ArrowUp" => Some(Key::ArrowUp),
        "ArrowRight" => Some(Key::ArrowRight),
        "ArrowDown" => Some(Key::ArrowDown),
        _ => None
    }
}
