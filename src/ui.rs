use crate::game::{CellStatus, GameState, PencilMarks, BOARD, SIZE};
use crate::generator::Difficulty;
use eframe::egui;
use egui::Color32;

// ── Color palettes ────────────────────────────────────────────────

mod light_theme {
    use super::Color32;

    // ── Design notes (white background) ───────────────────────────
    // GRID_BG: pure white base. All highlights blend on top via alpha.
    // SELECTED_BG: premultiplied (35,92,169,α=180/255) → over white ≈(110,167,244)
    // HIGHLIGHT_BG: premultiplied (76,102,134,α=140/255) → over white ≈(191,217,249)
    // BUTTON_BG / DIFF_ACTIVE: saturated blue for interactive feedback.

    pub const GRID_BG: Color32 = Color32::from_rgb(255, 255, 255);
    pub const TEXT: Color32 = Color32::from_rgb(30, 30, 30);
    pub const FIXED_TEXT: Color32 = Color32::from_rgb(50, 50, 50);
    pub const FILLED_TEXT: Color32 = Color32::from_rgb(30, 100, 240);
    pub const ERROR_TEXT: Color32 = Color32::from_rgb(220, 40, 40);
    pub const GRID_LINE_THIN: Color32 = Color32::from_rgb(180, 185, 200);
    pub const GRID_LINE_THICK: Color32 = Color32::from_rgb(60, 60, 70);
    pub const SELECTED_BG: Color32 = Color32::from_rgba_premultiplied(35, 92, 169, 180);
    pub const HIGHLIGHT_BG: Color32 = Color32::from_rgba_premultiplied(76, 102, 134, 140);
    pub const BUTTON_BG: Color32 = Color32::from_rgb(40, 110, 230);
    pub const NUMPAD_BG: Color32 = Color32::from_rgb(225, 228, 238);
    pub const DIFF_ACTIVE: Color32 = Color32::from_rgb(40, 110, 230);
}

mod dark_theme {
    use super::Color32;

    // ── Design notes (dark background) ───────────────────────────
    // GRID_BG: deep charcoal base. Low-alpha highlights create subtle glow.
    // SELECTED_BG: premultiplied (45,110,220,α=85/255) → over dark ≈(65,132,253)
    // HIGHLIGHT_BG: premultiplied (40,75,160,α=35/255) → over dark ≈(66,103,203)

    pub const GRID_BG: Color32 = Color32::from_rgb(30, 33, 50);
    pub const TEXT: Color32 = Color32::from_rgb(215, 215, 225);
    pub const FIXED_TEXT: Color32 = Color32::from_rgb(170, 175, 195);
    pub const FILLED_TEXT: Color32 = Color32::from_rgb(80, 165, 255);
    pub const ERROR_TEXT: Color32 = Color32::from_rgb(255, 75, 75);
    pub const GRID_LINE_THIN: Color32 = Color32::from_rgb(75, 82, 105);
    pub const GRID_LINE_THICK: Color32 = Color32::from_rgb(165, 175, 205);
    pub const SELECTED_BG: Color32 = Color32::from_rgba_premultiplied(45, 110, 220, 85);
    pub const HIGHLIGHT_BG: Color32 = Color32::from_rgba_premultiplied(40, 75, 160, 35);
    pub const BUTTON_BG: Color32 = Color32::from_rgb(45, 130, 245);
    pub const NUMPAD_BG: Color32 = Color32::from_rgb(40, 43, 60);
    pub const DIFF_ACTIVE: Color32 = Color32::from_rgb(45, 130, 245);
}

#[derive(Debug, Clone, Copy)]
struct ThemeColors {
    text: Color32,
    fixed_text: Color32,
    filled_text: Color32,
    error_text: Color32,
    grid_line_thin: Color32,
    grid_line_thick: Color32,
    selected_bg: Color32,
    highlight_bg: Color32,
    button_bg: Color32,
    grid_bg: Color32,
    numpad_bg: Color32,
    diff_active: Color32,
}

impl ThemeColors {
    fn light() -> Self {
        ThemeColors {
            grid_bg: light_theme::GRID_BG,
            text: light_theme::TEXT,
            fixed_text: light_theme::FIXED_TEXT,
            filled_text: light_theme::FILLED_TEXT,
            error_text: light_theme::ERROR_TEXT,
            grid_line_thin: light_theme::GRID_LINE_THIN,
            grid_line_thick: light_theme::GRID_LINE_THICK,
            selected_bg: light_theme::SELECTED_BG,
            highlight_bg: light_theme::HIGHLIGHT_BG,
            button_bg: light_theme::BUTTON_BG,
            numpad_bg: light_theme::NUMPAD_BG,
            diff_active: light_theme::DIFF_ACTIVE,
        }
    }

    fn dark() -> Self {
        ThemeColors {
            grid_bg: dark_theme::GRID_BG,
            text: dark_theme::TEXT,
            fixed_text: dark_theme::FIXED_TEXT,
            filled_text: dark_theme::FILLED_TEXT,
            error_text: dark_theme::ERROR_TEXT,
            grid_line_thin: dark_theme::GRID_LINE_THIN,
            grid_line_thick: dark_theme::GRID_LINE_THICK,
            selected_bg: dark_theme::SELECTED_BG,
            highlight_bg: dark_theme::HIGHLIGHT_BG,
            button_bg: dark_theme::BUTTON_BG,
            numpad_bg: dark_theme::NUMPAD_BG,
            diff_active: dark_theme::DIFF_ACTIVE,
        }
    }
}

const THEME_KEY: &str = "sudoku_dark_theme";
const SAVE_KEY: &str = "sudoku_save";

fn label_for_difficulty(d: Difficulty) -> &'static str {
    match d {
        Difficulty::Easy => "简单",
        Difficulty::Medium => "中等",
        Difficulty::Hard => "困难",
        Difficulty::Expert => "专家",
    }
}

pub struct SudokuApp {
    state: Option<GameState>,
    selected_cell: i32,
    pencil_mode: bool,
    auto_marks: bool,
    difficulty: Difficulty,
    dark_theme_flag: bool,
    last_tick: std::time::Instant,
    show_settings: bool,
    theme_dirty: bool,
    board_changed: bool,
    game_dirty: bool,
}

impl SudokuApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let dark_theme_flag = cc
            .storage
            .and_then(|s| s.get_string(THEME_KEY))
            .map(|s| s == "true")
            .unwrap_or(true);

        let mut app = SudokuApp {
            state: None,
            selected_cell: -1,
            pencil_mode: false,
            auto_marks: false,
            difficulty: Difficulty::Medium,
            dark_theme_flag,
            last_tick: std::time::Instant::now(),
            show_settings: false,
            theme_dirty: false,
            board_changed: true,
            game_dirty: false,
        };

        // Try to restore saved game state
        let restored = cc
            .storage
            .and_then(|s| eframe::get_value::<GameState>(s, SAVE_KEY));
        if let Some(saved_state) = restored {
            app.difficulty = saved_state.difficulty.into();
            app.state = Some(saved_state);
        } else {
            app.start_new_game();
        }

        app
    }

    fn start_new_game(&mut self) {
        let (puzzle, solution) = crate::generator::generate(self.difficulty);
        let label = label_for_difficulty(self.difficulty);
        self.state = Some(GameState::new(
            puzzle,
            solution,
            label.to_string(),
            self.difficulty.into(),
        ));
        self.selected_cell = -1;
        self.pencil_mode = false;
        self.board_changed = true;
        self.game_dirty = true;
    }

    fn colors(&self) -> ThemeColors {
        if self.dark_theme_flag {
            ThemeColors::dark()
        } else {
            ThemeColors::light()
        }
    }

    fn format_time(dur: std::time::Duration) -> String {
        let total_secs = dur.as_secs();
        let m = total_secs / 60;
        let s = total_secs % 60;
        format!("{m:02}:{s:02}")
    }

    fn elapsed_duration(&self, state: &GameState) -> std::time::Duration {
        if state.timer_running {
            std::time::Duration::from_secs_f64(state.elapsed_secs) + self.last_tick.elapsed()
        } else {
            std::time::Duration::from_secs_f64(state.elapsed_secs)
        }
    }

    fn handle_text_input(&mut self, text: &str) {
        if self.selected_cell < 0 || self.state.is_none() {
            return;
        }
        if let Some(digit) = text.chars().next() {
            if (b'1'..=b'9').contains(&(digit as u8)) {
                let num = digit as u8 - b'0';
                let idx = self.selected_cell as usize;
                self.set_cell_value(idx, num);
            }
        }
    }

    fn handle_key(&mut self, key: egui::Key) {
        if self.selected_cell < 0 || self.state.is_none() {
            return;
        }
        let idx = self.selected_cell as usize;
        match key {
            egui::Key::ArrowUp => {
                if idx / SIZE > 0 {
                    self.selected_cell = (idx - SIZE) as i32;
                }
            }
            egui::Key::ArrowDown => {
                if idx / SIZE < SIZE - 1 {
                    self.selected_cell = (idx + SIZE) as i32;
                }
            }
            egui::Key::ArrowLeft => {
                if !idx.is_multiple_of(SIZE) {
                    self.selected_cell = idx as i32 - 1;
                }
            }
            egui::Key::ArrowRight => {
                if idx % SIZE < SIZE - 1 {
                    self.selected_cell = idx as i32 + 1;
                }
            }
            egui::Key::Escape => {
                self.selected_cell = -1;
            }
            egui::Key::Backspace | egui::Key::Delete => {
                self.clear_cell(idx);
            }
            _ => {}
        }
    }

    fn set_cell_value(&mut self, idx: usize, num: u8) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };
        if state.puzzle[idx].is_fixed() {
            return;
        }
        state.push_undo();
        state.current.cells[idx] = CellStatus::Filled(num);
        state.current.pencils[idx] = PencilMarks::default();
        self.board_changed = true;
        self.game_dirty = true;
    }

    fn clear_cell(&mut self, idx: usize) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };
        if state.puzzle[idx].is_fixed() {
            return;
        }
        state.push_undo();
        state.current.cells[idx] = CellStatus::Empty;
        self.board_changed = true;
        self.game_dirty = true;
    }

    fn toggle_pencil_mark(&mut self, idx: usize, num: u8) {
        let state = match &mut self.state {
            Some(s) => s,
            None => return,
        };
        if state.puzzle[idx].is_fixed() {
            return;
        }
        state.push_undo();
        let marks = state.current.pencils[idx];
        state.current.pencils[idx] = if marks.contains(num) {
            marks.remove(num)
        } else {
            marks.with_digit(num)
        };
        self.game_dirty = true;
    }

    fn draw_grid(
        &self,
        ctx: &egui::Context,
        painter: &egui::Painter,
        rect: egui::Rect,
        conflicts: &[bool; BOARD],
    ) {
        let colors = self.colors();
        let state = match &self.state {
            Some(s) => s,
            None => return,
        };
        let cell_size = (rect.width() / SIZE as f32).floor().max(1.0);
        let thick_line = 3.5 * ctx.pixels_per_point();
        let thin_line = 1.2 * ctx.pixels_per_point();

        for r in 0..SIZE {
            for c in 0..SIZE {
                let idx = r * SIZE + c;
                let x = rect.left() + (c as f32) * cell_size;
                let y = rect.top() + (r as f32) * cell_size;
                let cell_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::vec2(cell_size, cell_size),
                );

                let bg_color = if self.selected_cell == idx as i32 {
                    colors.selected_bg
                } else if self.selected_cell >= 0 {
                    let sr = (self.selected_cell / SIZE as i32) as usize;
                    let sc = (self.selected_cell % SIZE as i32) as usize;
                    if r == sr || c == sc || (r / 3 == sr / 3 && c / 3 == sc / 3) {
                        colors.highlight_bg
                    } else {
                        colors.grid_bg
                    }
                } else {
                    colors.grid_bg
                };

                painter.rect_filled(cell_rect, 0.0, bg_color);

                let right = x + cell_size;
                let bottom = y + cell_size;

                if c == 2 || c == 5 {
                    painter.line_segment(
                        [egui::Pos2::new(right, y), egui::Pos2::new(right, bottom)],
                        egui::Stroke::new(thick_line, colors.grid_line_thick),
                    );
                } else if c < 8 {
                    painter.line_segment(
                        [egui::Pos2::new(right, y), egui::Pos2::new(right, bottom)],
                        egui::Stroke::new(thin_line, colors.grid_line_thin),
                    );
                }

                if r == 2 || r == 5 {
                    painter.line_segment(
                        [egui::Pos2::new(x, bottom), egui::Pos2::new(right, bottom)],
                        egui::Stroke::new(thick_line, colors.grid_line_thick),
                    );
                } else if r < 8 {
                    painter.line_segment(
                        [egui::Pos2::new(x, bottom), egui::Pos2::new(right, bottom)],
                        egui::Stroke::new(thin_line, colors.grid_line_thin),
                    );
                }

                let value = state.puzzle[idx]
                    .value()
                    .or_else(|| state.current.cells[idx].value());
                if let Some(v) = value {
                    let is_fixed = state.puzzle[idx].is_fixed();
                    let is_wrong = conflicts[idx];
                    let text_color = if is_wrong {
                        colors.error_text
                    } else if is_fixed {
                        colors.fixed_text
                    } else {
                        colors.filled_text
                    };
                    let font_size = (cell_size * 0.55).round().max(8.0);
                    let text = v.to_string();
                    let galley = ctx.fonts(|f| {
                        f.layout_no_wrap(text, egui::FontId::proportional(font_size), text_color)
                    });
                    let text_pos = egui::Pos2::new(
                        x + (cell_size - galley.size().x) / 2.0,
                        y + (cell_size - galley.size().y) / 2.0,
                    );
                    painter.galley(text_pos, galley, text_color);
                } else {
                    let marks = state.current.pencils[idx];
                    let (mark_digits, mark_count) = marks.digits();
                    if mark_count > 0 && cell_size > 30.0 {
                        let font_size = (cell_size * 0.18).round().max(6.0);
                        let margin = cell_size * 0.08;
                        let inner_w = cell_size - 2.0 * margin;
                        let cols = 3usize;
                        for (i, &d) in mark_digits[..mark_count].iter().enumerate() {
                            let col_pos = i % cols;
                            let row_pos = i / cols;
                            let dx = margin + (col_pos as f32) * (inner_w / cols as f32);
                            let dy = margin + (row_pos as f32) * (inner_w / cols as f32);
                            let text = d.to_string();
                            let galley = ctx.fonts(|f| {
                                f.layout_no_wrap(
                                    text,
                                    egui::FontId::monospace(font_size),
                                    colors.text,
                                )
                            });
                            let pos = egui::Pos2::new(
                                x + dx + (inner_w / cols as f32 - galley.size().x) / 2.0,
                                y + dy + (inner_w / cols as f32 - galley.size().y) / 2.0,
                            );
                            painter.galley(pos, galley, colors.text);
                        }
                    }
                }
            }
        }

        painter.rect_stroke(
            rect,
            4.0,
            egui::Stroke::new(thick_line * 1.5, colors.grid_line_thick),
            egui::StrokeKind::Middle,
        );
    }

    fn draw_right_panel(&mut self, ui: &mut egui::Ui, error_count: usize) {
        let colors = self.colors();
        let state = match &self.state {
            Some(s) => s,
            None => return,
        };

        // ── Status section ────────────────────────────────────────
        ui.horizontal(|ui| {
            let filled: usize = state
                .current
                .cells
                .iter()
                .zip(state.puzzle.iter())
                .filter(|(c, p)| !c.is_empty() && !p.is_fixed())
                .count();

            ui.label(
                egui::RichText::new(format!("{filled}"))
                    .size(24.0)
                    .color(colors.text),
            );

            ui.separator();
            ui.label(
                egui::RichText::new(format!("错误 {error_count}"))
                    .size(12.0)
                    .color(colors.text),
            );

            ui.separator();
            let elapsed = self.elapsed_duration(state);
            ui.label(
                egui::RichText::new(format!("时间 {}", Self::format_time(elapsed)))
                    .size(12.0)
                    .color(colors.text),
            );
        });

        ui.separator();

        // ── Tool icons row ────────────────────────────────────────
        let tool_size = 40.0;
        ui.horizontal(|ui| {
            if ui
                .add_sized(
                    [tool_size, tool_size],
                    egui::Button::new("撤销").frame(true),
                )
                .clicked()
            {
                if let Some(ref mut st) = self.state {
                    st.undo();
                    self.game_dirty = true;
                }
            }

            if ui
                .add_sized(
                    [tool_size, tool_size],
                    egui::Button::new("重做").frame(true),
                )
                .clicked()
            {
                if let Some(ref mut st) = self.state {
                    st.redo();
                    self.game_dirty = true;
                }
            }

            if ui
                .add_sized(
                    [tool_size, tool_size],
                    egui::Button::new("清除").frame(true),
                )
                .clicked()
                && self.selected_cell >= 0
            {
                let idx = self.selected_cell as usize;
                self.clear_cell(idx);
            }

            let notes_text = if self.pencil_mode {
                "标注: ON"
            } else {
                "标注: OFF"
            };
            if ui
                .add_sized(
                    [tool_size, tool_size],
                    egui::Button::new(notes_text).frame(true),
                )
                .clicked()
            {
                self.pencil_mode = !self.pencil_mode;
            }

            let hint_count = self
                .state
                .as_ref()
                .map(|st| (0..BOARD).filter(|&i| st.can_hint(i)).count())
                .unwrap_or(0);
            if ui
                .add_sized(
                    [tool_size, tool_size],
                    egui::Button::new(format!("提示 {hint_count}")).frame(true),
                )
                .clicked()
                && self.selected_cell >= 0
            {
                let idx = self.selected_cell as usize;
                if let Some(ref mut st) = self.state {
                    if st.can_hint(idx) {
                        st.apply_hint(idx);
                        self.board_changed = true;
                        self.game_dirty = true;
                    }
                }
            }
        });

        ui.separator();

        // ── Number pad 1-9 (3×3 grid) ─────────────────────────────
        let btn_w = ui.available_width() / 3.0 - 4.0;
        for row in 0..3u8 {
            ui.horizontal(|ui| {
                for col in 0..3u8 {
                    let num = row * 3 + col + 1;
                    let label = num.to_string();
                    let btn = egui::Button::new(&label).fill(colors.numpad_bg).frame(true);
                    if ui.add_sized([btn_w.max(20.0), 44.0], btn).clicked()
                        && self.selected_cell >= 0
                    {
                        let idx = self.selected_cell as usize;
                        if self.pencil_mode {
                            self.toggle_pencil_mark(idx, num);
                        } else {
                            self.set_cell_value(idx, num);
                        }
                    }
                }
            });
        }

        ui.separator();

        // ── New Game button ───────────────────────────────────────
        if ui
            .add_sized(
                [ui.available_width(), 40.0],
                egui::Button::new("新游戏").fill(colors.button_bg),
            )
            .clicked()
        {
            self.start_new_game();
        }
    }

    fn draw_settings_popup(&mut self, ctx: &egui::Context) {
        egui::Area::new("settings_popup".into())
            .order(egui::Order::Foreground)
            .default_pos(ctx.screen_rect().center())
            .show(ctx, |ui| {
                let bg = if self.dark_theme_flag {
                    Color32::from_rgb(35, 38, 55)
                } else {
                    Color32::from_rgb(245, 245, 250)
                };
                let border = if self.dark_theme_flag {
                    Color32::from_rgb(80, 85, 110)
                } else {
                    Color32::from_rgb(180, 185, 200)
                };
                egui::Frame::NONE
                    .fill(bg)
                    .inner_margin(16.0)
                    .corner_radius(8.0)
                    .stroke(egui::Stroke::new(1.0, border))
                    .show(ui, |ui| {
                        ui.heading("设置");
                        ui.separator();

                        if let Some(ref st) = self.state {
                            ui.label(format!("难度: {}", st.difficulty_label));

                            let filled: usize = st
                                .current
                                .cells
                                .iter()
                                .zip(st.puzzle.iter())
                                .filter(|(c, p)| !c.is_empty() && !p.is_fixed())
                                .count();
                            ui.label(format!("已填: {filled}/81"));

                            let error_count = st.conflict_cells().iter().filter(|&&c| c).count();
                            ui.label(format!("错误: {error_count}"));

                            let elapsed = self.elapsed_duration(st);
                            ui.label(format!("用时: {}", Self::format_time(elapsed)));

                            if st.is_complete() {
                                ui.label("状态: 已完成");
                            }

                            ui.separator();
                            ui.checkbox(&mut self.auto_marks, "自动标注候选");
                        }

                        ui.separator();
                        if ui.button("关闭").clicked() {
                            self.show_settings = false;
                        }
                    });
            });
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme visuals
        if self.dark_theme_flag {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // Persist theme change
        if self.theme_dirty {
            self.theme_dirty = false;
            if let Some(storage) = _frame.storage_mut() {
                eframe::set_value(storage, THEME_KEY, &self.dark_theme_flag.to_string());
            }
        }

        // Persist game state
        if self.game_dirty {
            self.game_dirty = false;
            if let Some(storage) = _frame.storage_mut() {
                if let Some(ref state) = self.state {
                    eframe::set_value(storage, SAVE_KEY, state);
                }
            }
        }

        // ── Top bar (header) ──────────────────────────────────────
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            let colors = self.colors();
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("数独").color(colors.text).size(18.0));

                for &diff in &[
                    (Difficulty::Easy, "简单"),
                    (Difficulty::Medium, "中等"),
                    (Difficulty::Hard, "困难"),
                    (Difficulty::Expert, "专家"),
                ] {
                    let is_selected = self.difficulty == diff.0;
                    let mut btn = egui::Button::new(diff.1).min_size(egui::vec2(50.0, 24.0));
                    if is_selected {
                        btn = btn.fill(colors.diff_active);
                    }
                    if ui.add(btn).clicked() {
                        self.difficulty = diff.0;
                        self.start_new_game();
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("设置").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                    if ui
                        .button(if self.dark_theme_flag {
                            "浅色"
                        } else {
                            "深色"
                        })
                        .clicked()
                    {
                        self.dark_theme_flag = !self.dark_theme_flag;
                        self.theme_dirty = true;
                    }
                });
            });
        });

        if self.show_settings {
            self.draw_settings_popup(ctx);
        }

        // ── Central panel: grid (left) + controls (right) ─────────
        egui::CentralPanel::default().show(ctx, |ui| {
            // Compute conflicts once per frame
            let (conflicts, error_count) = self
                .state
                .as_ref()
                .map(|st| {
                    let c = st.conflict_cells();
                    let err = c.iter().filter(|&&x| x).count();
                    (c, err)
                })
                .unwrap_or_else(|| ([false; BOARD], 0));

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let right_panel_width = 250.0;
                    let spacing = 16.0;
                    let grid_max_w = ui.available_width() - right_panel_width - spacing;
                    let avail_h = ui.available_height();
                    let grid_size = grid_max_w.min(avail_h).max(520.0);

                    let (_id, grid_resp) = ui.allocate_exact_size(
                        egui::Vec2::new(grid_size, grid_size),
                        egui::Sense::click(),
                    );
                    if grid_resp.clicked() {
                        if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                            if pos.x >= grid_resp.rect.left() && pos.y >= grid_resp.rect.top() {
                                let col = ((pos.x - grid_resp.rect.left())
                                    / (grid_size / SIZE as f32))
                                    .floor() as usize;
                                let row = ((pos.y - grid_resp.rect.top())
                                    / (grid_size / SIZE as f32))
                                    .floor() as usize;
                                if col < SIZE && row < SIZE {
                                    self.selected_cell = (row * SIZE + col) as i32;
                                }
                            }
                        }
                    }
                    let painter = ui.painter();
                    self.draw_grid(ctx, painter, grid_resp.rect, &conflicts);
                });

                ui.add_space(16.0);

                ui.vertical(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(250.0, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            self.draw_right_panel(ui, error_count);
                        },
                    );
                });
            });

            // Update timer and auto-pencil-marks
            if let Some(ref mut state) = self.state {
                if state.timer_running {
                    let elapsed = self.last_tick.elapsed();
                    state.elapsed_secs += elapsed.as_secs_f64();
                    self.last_tick = std::time::Instant::now();
                    self.game_dirty = true;
                }

                if self.auto_marks && self.board_changed && !state.is_complete() {
                    state.auto_pencil_marks();
                    self.board_changed = false;
                    self.game_dirty = true;
                }
            }
        });

        // ── Keyboard input ────────────────────────────────────────
        ctx.input(|i| {
            for ev in &i.events {
                match ev {
                    egui::Event::Text(t) => {
                        self.handle_text_input(t);
                    }
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => match *key {
                        egui::Key::ArrowUp
                        | egui::Key::ArrowDown
                        | egui::Key::ArrowLeft
                        | egui::Key::ArrowRight
                        | egui::Key::Backspace
                        | egui::Key::Delete
                        | egui::Key::Escape => {
                            self.handle_key(*key);
                        }
                        egui::Key::Z if modifiers.ctrl => {
                            if let Some(ref mut st) = self.state {
                                st.undo();
                                self.game_dirty = true;
                            }
                        }
                        egui::Key::Y if modifiers.ctrl => {
                            if let Some(ref mut st) = self.state {
                                st.redo();
                                self.game_dirty = true;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        });
    }
}
