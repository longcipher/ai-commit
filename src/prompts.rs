pub const DEFAULT_SYSTEM_PROMPT: &str = include_str!("../prompts/system.md");

pub fn get_system_prompt() -> String {
    DEFAULT_SYSTEM_PROMPT.to_string()
}
