use macroquad::prelude::*;

// ── Game states ──
#[derive(PartialEq, Clone)]
enum GameState {
    Menu,
    Playing,
    GameOver,
}

// ── Bin categories ──
#[derive(PartialEq, Clone, Copy, Debug)]
enum BinType {
    Recycle,
    Compost,
    Donate,
    Landfill,
}

impl BinType {
    fn color(&self) -> Color {
        match self {
            BinType::Recycle => Color::new(0.22, 0.53, 0.78, 1.0),  // blue
            BinType::Compost => Color::new(0.27, 0.63, 0.28, 1.0),  // green
            BinType::Donate => Color::new(0.85, 0.68, 0.20, 1.0),   // yellow/gold
            BinType::Landfill => Color::new(0.55, 0.55, 0.55, 1.0), // gray
        }
    }

    fn label(&self) -> &str {
        match self {
            BinType::Recycle => "Recycle",
            BinType::Compost => "Compost",
            BinType::Donate => "Donate",
            BinType::Landfill => "Landfill",
        }
    }

    fn key_label(&self) -> &str {
        match self {
            BinType::Recycle => "1",
            BinType::Compost => "2",
            BinType::Donate => "3",
            BinType::Landfill => "4",
        }
    }
}

// ── Clothing item ──
#[derive(Clone)]
struct ClothingItem {
    name: &'static str,
    emoji: &'static str,
    correct_bin: BinType,
    reason: &'static str,
}

// ── Falling item ──
struct FallingItem {
    clothing: ClothingItem,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

// ── Feedback message ──
struct Feedback {
    text: String,
    color: Color,
    timer: f32,
}

// ── All possible items ──
fn all_items() -> Vec<ClothingItem> {
    vec![
        ClothingItem { name: "Cotton T-Shirt", emoji: "T", correct_bin: BinType::Compost, reason: "Cotton is biodegradable!" },
        ClothingItem { name: "Polyester Jacket", emoji: "J", correct_bin: BinType::Recycle, reason: "Polyester can be melted & remade!" },
        ClothingItem { name: "Torn Nylon Bag", emoji: "B", correct_bin: BinType::Landfill, reason: "Torn nylon can't be recycled yet." },
        ClothingItem { name: "Wool Sweater", emoji: "W", correct_bin: BinType::Donate, reason: "Wool lasts long - someone else can wear it!" },
        ClothingItem { name: "Silk Scarf", emoji: "S", correct_bin: BinType::Donate, reason: "Silk holds value - pass it on!" },
        ClothingItem { name: "Linen Pants", emoji: "L", correct_bin: BinType::Compost, reason: "Linen is a natural fiber & compostable!" },
        ClothingItem { name: "Acrylic Beanie", emoji: "A", correct_bin: BinType::Recycle, reason: "Acrylic is a recyclable synthetic!" },
        ClothingItem { name: "Stained PVC Coat", emoji: "P", correct_bin: BinType::Landfill, reason: "PVC is hard to recycle safely." },
        ClothingItem { name: "Hemp Tote", emoji: "H", correct_bin: BinType::Compost, reason: "Hemp is 100% biodegradable!" },
        ClothingItem { name: "Denim Jeans", emoji: "D", correct_bin: BinType::Donate, reason: "Durable denim has a second life!" },
        ClothingItem { name: "Rayon Dress", emoji: "R", correct_bin: BinType::Recycle, reason: "Rayon fibers can be recovered!" },
        ClothingItem { name: "Broken Zipper Vest", emoji: "V", correct_bin: BinType::Landfill, reason: "Mixed materials with broken parts = landfill." },
        ClothingItem { name: "Bamboo Socks", emoji: "K", correct_bin: BinType::Compost, reason: "Bamboo fabric is biodegradable!" },
        ClothingItem { name: "Fleece Hoodie", emoji: "F", correct_bin: BinType::Recycle, reason: "Fleece (recycled plastic) can be recycled again!" },
        ClothingItem { name: "Leather Boots", emoji: "O", correct_bin: BinType::Donate, reason: "Quality leather can be repaired & reused!" },
        ClothingItem { name: "Spandex Leggings", emoji: "X", correct_bin: BinType::Landfill, reason: "Spandex blends are hard to separate." },
    ]
}

fn spawn_item(items: &[ClothingItem], sw: f32) -> FallingItem {
    let idx = rand::gen_range(0, items.len());
    let clothing = items[idx].clone();
    let w = 120.0_f32.min(sw * 0.18);
    let h = 60.0_f32.min(sw * 0.09);
    let x = rand::gen_range(10.0, sw - w - 10.0);
    FallingItem { clothing, x, y: -h, width: w, height: h }
}

// ── Draw rounded rect helper ──
fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    // body
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
    // corners
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

#[macroquad::main("Factory Sort - Rewoven")]
async fn main() {
    let items_pool = all_items();

    let mut state = GameState::Menu;
    let mut score: i32 = 0;
    let mut lives: i32 = 3;
    let mut fall_speed: f32 = 1.5;
    let mut current_item: Option<FallingItem> = None;
    let mut feedback: Option<Feedback> = None;
    let mut items_sorted: u32 = 0;
    let mut combo: u32 = 0;
    let mut best_combo: u32 = 0;

    loop {
        let sw = screen_width();
        let sh = screen_height();

        clear_background(Color::new(0.04, 0.055, 0.10, 1.0));

        match state {
            // ═══════════════════ MENU ═══════════════════
            GameState::Menu => {
                // Background gradient overlay
                draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.04, 0.055, 0.10, 1.0));

                // Decorative circles
                draw_circle(sw * 0.8, sh * 0.2, 180.0, Color::new(0.06, 0.45, 0.35, 0.08));
                draw_circle(sw * 0.15, sh * 0.75, 120.0, Color::new(0.06, 0.45, 0.35, 0.06));

                // Title
                let title = "Factory Sort";
                let title_size = (sw * 0.08).clamp(36.0, 64.0);
                let title_dim = measure_text(title, None, title_size as u16, 1.0);
                draw_text(title, (sw - title_dim.width) / 2.0, sh * 0.22, title_size, Color::new(0.06, 0.73, 0.51, 1.0));

                // Subtitle
                let sub = "Textile Recycling Facility";
                let sub_size = (sw * 0.03).clamp(16.0, 24.0);
                let sub_dim = measure_text(sub, None, sub_size as u16, 1.0);
                draw_text(sub, (sw - sub_dim.width) / 2.0, sh * 0.28, sub_size, Color::new(0.58, 0.64, 0.70, 1.0));

                // Instructions box
                let box_w = (sw * 0.6).clamp(300.0, 500.0);
                let box_h = 200.0;
                let box_x = (sw - box_w) / 2.0;
                let box_y = sh * 0.34;
                draw_rounded_rect(box_x, box_y, box_w, box_h, 12.0, Color::new(0.08, 0.10, 0.18, 0.9));

                let inst_size = (sw * 0.022).clamp(13.0, 17.0);
                let line_h = inst_size * 1.6;
                let tx = box_x + 24.0;
                let mut ty = box_y + 32.0;

                let instructions = [
                    "Sort falling textile items into the correct bins:",
                    "",
                    "  [1] Recycle    [2] Compost",
                    "  [3] Donate     [4] Landfill",
                    "",
                    "  Left/Right or A/D to move items",
                    "  3 lives - speed increases over time!",
                    "",
                    "  Correct sort = +10 pts (combos = bonus!)",
                ];
                for line in &instructions {
                    draw_text(line, tx, ty, inst_size, Color::new(0.75, 0.80, 0.85, 1.0));
                    ty += line_h;
                }

                // Bin preview
                let bin_w = (sw * 0.1).clamp(50.0, 80.0);
                let bin_gap = 12.0;
                let total_w = 4.0 * bin_w + 3.0 * bin_gap;
                let bins_x = (sw - total_w) / 2.0;
                let bins_y = sh * 0.68;
                for (i, bt) in [BinType::Recycle, BinType::Compost, BinType::Donate, BinType::Landfill].iter().enumerate() {
                    let bx = bins_x + i as f32 * (bin_w + bin_gap);
                    draw_rounded_rect(bx, bins_y, bin_w, 40.0, 6.0, bt.color());
                    let lbl = bt.label();
                    let lbl_dim = measure_text(lbl, None, 13, 1.0);
                    draw_text(lbl, bx + (bin_w - lbl_dim.width) / 2.0, bins_y + 26.0, 13.0, WHITE);
                }

                // Play button
                let btn_w = 200.0;
                let btn_h = 50.0;
                let btn_x = (sw - btn_w) / 2.0;
                let btn_y = sh * 0.82;
                draw_rounded_rect(btn_x, btn_y, btn_w, btn_h, 12.0, Color::new(0.06, 0.73, 0.51, 1.0));
                let play_text = "PLAY";
                let play_dim = measure_text(play_text, None, 24, 1.0);
                draw_text(play_text, (sw - play_dim.width) / 2.0, btn_y + 33.0, 24.0, WHITE);

                // Check for start
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                    score = 0;
                    lives = 3;
                    fall_speed = 1.5;
                    current_item = None;
                    items_sorted = 0;
                    combo = 0;
                    best_combo = 0;
                    feedback = None;
                }

                // Mouse click on play button
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                        state = GameState::Playing;
                        score = 0;
                        lives = 3;
                        fall_speed = 1.5;
                        current_item = None;
                        items_sorted = 0;
                        combo = 0;
                        best_combo = 0;
                        feedback = None;
                    }
                }
            }

            // ═══════════════════ PLAYING ═══════════════════
            GameState::Playing => {
                let dt = get_frame_time();

                // Spawn item if needed
                if current_item.is_none() {
                    current_item = Some(spawn_item(&items_pool, sw));
                }

                // Bin dimensions
                let bin_h = (sh * 0.12).clamp(60.0, 100.0);
                let bin_area_y = sh - bin_h - 10.0;
                let bin_gap = 8.0;
                let bin_w = (sw - 5.0 * bin_gap) / 4.0;
                let bin_types = [BinType::Recycle, BinType::Compost, BinType::Donate, BinType::Landfill];

                // Move item
                if let Some(ref mut item) = current_item {
                    let move_speed = 300.0 * dt;
                    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                        item.x = (item.x - move_speed).max(5.0);
                    }
                    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                        item.x = (item.x + move_speed).min(sw - item.width - 5.0);
                    }

                    // Fall
                    item.y += fall_speed * 60.0 * dt;

                    // Check bin sorting via keys
                    let mut chosen_bin: Option<BinType> = None;
                    if is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Kp1) {
                        chosen_bin = Some(BinType::Recycle);
                    }
                    if is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Kp2) {
                        chosen_bin = Some(BinType::Compost);
                    }
                    if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Kp3) {
                        chosen_bin = Some(BinType::Donate);
                    }
                    if is_key_pressed(KeyCode::Key4) || is_key_pressed(KeyCode::Kp4) {
                        chosen_bin = Some(BinType::Landfill);
                    }

                    // Check bin clicking
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let (mx, my) = mouse_position();
                        if my >= bin_area_y && my <= bin_area_y + bin_h {
                            for (i, bt) in bin_types.iter().enumerate() {
                                let bx = bin_gap + i as f32 * (bin_w + bin_gap);
                                if mx >= bx && mx <= bx + bin_w {
                                    chosen_bin = Some(*bt);
                                    break;
                                }
                            }
                        }
                    }

                    if let Some(chosen) = chosen_bin {
                        let correct = chosen == item.clothing.correct_bin;
                        if correct {
                            combo += 1;
                            if combo > best_combo { best_combo = combo; }
                            let bonus = if combo >= 3 { (combo as i32 - 2) * 5 } else { 0 };
                            score += 10 + bonus;
                            let mut msg = format!("Correct! {}", item.clothing.reason);
                            if combo >= 3 {
                                msg = format!("{}x COMBO! {} (+{} bonus)", combo, item.clothing.reason, bonus);
                            }
                            feedback = Some(Feedback {
                                text: msg,
                                color: Color::new(0.06, 0.73, 0.51, 1.0),
                                timer: 2.0,
                            });
                        } else {
                            lives -= 1;
                            combo = 0;
                            feedback = Some(Feedback {
                                text: format!("Wrong! {} goes in {}. {}", item.clothing.name, item.clothing.correct_bin.label(), item.clothing.reason),
                                color: Color::new(0.90, 0.30, 0.30, 1.0),
                                timer: 2.5,
                            });
                        }
                        items_sorted += 1;
                        current_item = None;

                        // Speed up every 5 items
                        if items_sorted % 5 == 0 {
                            fall_speed += 0.3;
                        }

                        if lives <= 0 {
                            state = GameState::GameOver;
                        }
                    }

                    // Item fell off screen without sorting
                    if current_item.is_some() && current_item.as_ref().unwrap().y > bin_area_y + bin_h {
                        lives -= 1;
                        combo = 0;
                        let item_ref = current_item.as_ref().unwrap();
                        feedback = Some(Feedback {
                            text: format!("Missed! {} should go in {}.", item_ref.clothing.name, item_ref.clothing.correct_bin.label()),
                            color: Color::new(0.90, 0.60, 0.20, 1.0),
                            timer: 2.0,
                        });
                        items_sorted += 1;
                        current_item = None;
                        if lives <= 0 {
                            state = GameState::GameOver;
                        }
                    }
                }

                // ── Draw bins ──
                for (i, bt) in bin_types.iter().enumerate() {
                    let bx = bin_gap + i as f32 * (bin_w + bin_gap);
                    let col = bt.color();
                    // Bin body
                    draw_rounded_rect(bx, bin_area_y, bin_w, bin_h, 8.0, col);
                    // Rim
                    draw_rounded_rect(bx - 2.0, bin_area_y, bin_w + 4.0, 8.0, 4.0, Color::new(col.r + 0.1, col.g + 0.1, col.b + 0.1, 1.0));

                    // Label
                    let lbl = bt.label();
                    let lbl_size = (bin_w * 0.18).clamp(12.0, 18.0);
                    let lbl_dim = measure_text(lbl, None, lbl_size as u16, 1.0);
                    draw_text(lbl, bx + (bin_w - lbl_dim.width) / 2.0, bin_area_y + bin_h * 0.5, lbl_size, WHITE);

                    // Key hint
                    let key = bt.key_label();
                    let key_size = lbl_size * 0.85;
                    let key_dim = measure_text(key, None, key_size as u16, 1.0);
                    draw_text(key, bx + (bin_w - key_dim.width) / 2.0, bin_area_y + bin_h * 0.75, key_size, Color::new(1.0, 1.0, 1.0, 0.6));
                }

                // ── Draw falling item ──
                if let Some(ref item) = current_item {
                    // Item card
                    draw_rounded_rect(item.x, item.y, item.width, item.height, 8.0, Color::new(0.14, 0.18, 0.28, 1.0));
                    draw_rounded_rect(item.x + 2.0, item.y + 2.0, item.width - 4.0, item.height - 4.0, 6.0, Color::new(0.18, 0.22, 0.34, 1.0));

                    // Emoji/letter indicator
                    let em_size = (item.height * 0.45).clamp(14.0, 28.0);
                    draw_text(item.clothing.emoji, item.x + 10.0, item.y + item.height * 0.6, em_size, Color::new(0.06, 0.73, 0.51, 1.0));

                    // Name
                    let name_size = (item.width * 0.11).clamp(11.0, 15.0);
                    draw_text(item.clothing.name, item.x + 10.0 + em_size + 4.0, item.y + item.height * 0.55, name_size, WHITE);
                }

                // ── HUD ──
                // Score
                let hud_y = 30.0;
                let score_text = format!("Score: {}", score);
                draw_text(&score_text, 16.0, hud_y, 22.0, Color::new(0.06, 0.73, 0.51, 1.0));

                // Lives
                let lives_text = format!("Lives: {}", "* ".repeat(lives as usize).trim());
                let lives_dim = measure_text(&lives_text, None, 22, 1.0);
                draw_text(&lives_text, sw - lives_dim.width - 16.0, hud_y, 22.0, Color::new(0.90, 0.30, 0.30, 1.0));

                // Combo
                if combo >= 2 {
                    let combo_text = format!("Combo: {}x", combo);
                    let combo_dim = measure_text(&combo_text, None, 18, 1.0);
                    draw_text(&combo_text, (sw - combo_dim.width) / 2.0, hud_y, 18.0, Color::new(0.95, 0.75, 0.15, 1.0));
                }

                // Speed indicator
                let speed_text = format!("Speed: {:.1}x", fall_speed / 1.5);
                draw_text(&speed_text, 16.0, hud_y + 22.0, 14.0, Color::new(0.58, 0.64, 0.70, 0.8));

                // ── Feedback ──
                if let Some(ref mut fb) = feedback {
                    fb.timer -= dt;
                    if fb.timer > 0.0 {
                        let alpha = fb.timer.min(1.0);
                        let fb_size = (sw * 0.025).clamp(14.0, 20.0);
                        let fb_dim = measure_text(&fb.text, None, fb_size as u16, 1.0);
                        let fb_x = (sw - fb_dim.width) / 2.0;
                        let fb_y = sh * 0.45;

                        // Background
                        draw_rounded_rect(fb_x - 16.0, fb_y - fb_size - 4.0, fb_dim.width + 32.0, fb_size + 16.0, 8.0,
                            Color::new(0.0, 0.0, 0.0, 0.7 * alpha));
                        draw_text(&fb.text, fb_x, fb_y, fb_size, Color::new(fb.color.r, fb.color.g, fb.color.b, alpha));
                    }
                }
                if feedback.as_ref().map_or(false, |f| f.timer <= 0.0) {
                    feedback = None;
                }
            }

            // ═══════════════════ GAME OVER ═══════════════════
            GameState::GameOver => {
                draw_circle(sw * 0.5, sh * 0.4, 200.0, Color::new(0.90, 0.20, 0.20, 0.04));

                let go_text = "Game Over";
                let go_size = (sw * 0.07).clamp(32.0, 56.0);
                let go_dim = measure_text(go_text, None, go_size as u16, 1.0);
                draw_text(go_text, (sw - go_dim.width) / 2.0, sh * 0.25, go_size, Color::new(0.90, 0.30, 0.30, 1.0));

                // Stats box
                let box_w = (sw * 0.5).clamp(260.0, 400.0);
                let box_h = 180.0;
                let box_x = (sw - box_w) / 2.0;
                let box_y = sh * 0.32;
                draw_rounded_rect(box_x, box_y, box_w, box_h, 12.0, Color::new(0.08, 0.10, 0.18, 0.9));

                let stat_size = (sw * 0.028).clamp(16.0, 22.0);
                let line_h = stat_size * 2.0;
                let sx = box_x + 32.0;
                let mut sy = box_y + 40.0;

                let stats = [
                    format!("Final Score: {}", score),
                    format!("Items Sorted: {}", items_sorted),
                    format!("Best Combo: {}x", best_combo),
                    format!("Max Speed: {:.1}x", fall_speed / 1.5),
                ];
                for s in &stats {
                    draw_text(s, sx, sy, stat_size, Color::new(0.85, 0.88, 0.92, 1.0));
                    sy += line_h;
                }

                // Play again button
                let btn_w = 220.0;
                let btn_h = 50.0;
                let btn_x = (sw - btn_w) / 2.0;
                let btn_y = sh * 0.72;
                draw_rounded_rect(btn_x, btn_y, btn_w, btn_h, 12.0, Color::new(0.06, 0.73, 0.51, 1.0));
                let again_text = "PLAY AGAIN";
                let again_dim = measure_text(again_text, None, 22, 1.0);
                draw_text(again_text, (sw - again_dim.width) / 2.0, btn_y + 33.0, 22.0, WHITE);

                // Menu button
                let menu_btn_y = btn_y + 65.0;
                draw_rounded_rect(btn_x, menu_btn_y, btn_w, btn_h, 12.0, Color::new(0.25, 0.28, 0.35, 1.0));
                let menu_text = "MAIN MENU";
                let menu_dim = measure_text(menu_text, None, 22, 1.0);
                draw_text(menu_text, (sw - menu_dim.width) / 2.0, menu_btn_y + 33.0, 22.0, WHITE);

                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    state = GameState::Playing;
                    score = 0;
                    lives = 3;
                    fall_speed = 1.5;
                    current_item = None;
                    items_sorted = 0;
                    combo = 0;
                    best_combo = 0;
                    feedback = None;
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= btn_x && mx <= btn_x + btn_w {
                        if my >= btn_y && my <= btn_y + btn_h {
                            state = GameState::Playing;
                            score = 0;
                            lives = 3;
                            fall_speed = 1.5;
                            current_item = None;
                            items_sorted = 0;
                            combo = 0;
                            best_combo = 0;
                            feedback = None;
                        } else if my >= menu_btn_y && my <= menu_btn_y + btn_h {
                            state = GameState::Menu;
                        }
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
