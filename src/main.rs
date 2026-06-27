//! Sudoku — Main Entry Point
//!
//! Launches the egui-based Sudoku game application.

use std::sync::Arc;

mod fireworks;
mod game;
mod generator;
mod i18n;
mod solver;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let mut fonts = eframe::egui::FontDefinitions::default();

    // Try loading a CJK font from common system paths across platforms.
    // The font is needed to render Chinese UI strings properly.
    let font_paths = [
        // Windows
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        // Linux
        "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/wqy-zenhei/wqy-zenhei.ttc",
        // macOS
        "/System/Library/Fonts/PingFang.ttc",
        "/System/Library/Fonts/STHeiti Medium.ttc",
    ];

    let mut font_loaded = false;
    for path in &font_paths {
        if let Ok(font_data) = std::fs::read(path) {
            fonts.font_data.insert(
                "chinese".to_string(),
                Arc::new(eframe::egui::FontData::from_owned(font_data)),
            );
            font_loaded = true;
            break;
        }
    }

    if !font_loaded {
        eprintln!("WARNING: No CJK font found. Chinese text may render as tofu/boxes.");
    }

    // Put Chinese font first for Proportional and Monospace families (highest priority)
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "chinese".to_string());
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "chinese".to_string());

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 850.0])
            .with_min_inner_size([450.0, 500.0])
            .with_title("数独 · Sudoku"),
        ..Default::default()
    };

    eframe::run_native(
        "数独 · Sudoku",
        options,
        Box::new(move |cc| {
            // Set fonts on the context before creating the app
            cc.egui_ctx.set_fonts(fonts.clone());
            Ok(Box::new(ui::SudokuApp::new(cc)))
        }),
    )?;

    Ok(())
}
