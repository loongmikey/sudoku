//! User-facing string resources.
//!
//! All UI text is centralized here for easy maintenance and future
//! internationalisation. The current default language is Chinese.

use crate::generator::Difficulty;

pub fn title() -> &'static str {
    "数独 · Sudoku"
}

pub fn difficulty_label(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "简单",
        Difficulty::Medium => "中等",
        Difficulty::Hard => "困难",
        Difficulty::Expert => "专家",
    }
}

pub fn undo() -> &'static str {
    "撤销"
}

pub fn redo() -> &'static str {
    "重做"
}

pub fn clear() -> &'static str {
    "清除"
}

pub fn pencil_on() -> &'static str {
    "标注: ON"
}

pub fn pencil_off() -> &'static str {
    "标注: OFF"
}

pub fn hint(count: usize) -> String {
    format!("提示 {count}")
}

pub fn new_game() -> &'static str {
    "新游戏"
}

pub fn settings() -> &'static str {
    "设置"
}

pub fn light_theme() -> &'static str {
    "浅色"
}

pub fn dark_theme() -> &'static str {
    "深色"
}

pub fn difficulty_label_text(label: &str) -> String {
    format!("难度: {label}")
}

pub fn filled(count: usize) -> String {
    format!("已填: {count}/81")
}

pub fn errors(count: usize) -> String {
    format!("错误 {count}")
}

pub fn elapsed_time(formatted: &str) -> String {
    format!("时间 {formatted}")
}

pub fn status_complete() -> &'static str {
    "状态: 已完成"
}

pub fn auto_marks() -> &'static str {
    "自动标注候选"
}

pub fn close() -> &'static str {
    "关闭"
}
