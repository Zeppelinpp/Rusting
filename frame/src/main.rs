use eframe::egui;
use qrcode::{Color, QrCode};
use std::time::{Duration, Instant};

const WIDTH: f32 = 480.0;
const HEIGHT: f32 = 360.0;
const QR_URL: &str = "https://github.com/Zeppelinpp";

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([WIDTH, HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Streaming Timer",
        options,
        Box::new(|cc| {
            let mut app = App::new();
            app.try_load_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

struct App {
    start: Instant,
    qr_modules: Vec<bool>,
    qr_size: usize,
    fonts_loaded: bool,
}

impl App {
    fn new() -> Self {
        let qr = QrCode::new(QR_URL).expect("二维码生成失败");
        let qr_size = qr.width();
        let qr_modules = qr
            .to_colors()
            .into_iter()
            .map(|c| c == Color::Dark)
            .collect();
        Self {
            start: Instant::now(),
            qr_modules,
            qr_size,
            fonts_loaded: false,
        }
    }

    fn try_load_custom_fonts(&mut self, ctx: &egui::Context) {
        if self.fonts_loaded {
            return;
        }
        self.fonts_loaded = true;

        let home = std::env::var("HOME").unwrap_or_default();
        let candidates = [
            format!("{}/Library/Fonts/FiraCodeNerdFont-Regular.ttf", home),
            format!(
                "{}/Library/Fonts/Fira Code Regular Nerd Font Complete.ttf",
                home
            ),
            "./font.ttf".to_string(),
            "/System/Library/Fonts/Helvetica.ttc".to_string(),
            "/System/Library/Fonts/Supplemental/Arial.ttf".to_string(),
            "/Library/Fonts/Arial.ttf".to_string(),
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".to_string(),
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf".to_string(),
        ];

        for path in &candidates {
            if let Ok(bytes) = std::fs::read(path) {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "custom_font".to_owned(),
                    std::sync::Arc::new(egui::FontData::from_owned(bytes)),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "custom_font".to_owned());
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "custom_font".to_owned());
                ctx.set_fonts(fonts);
                return;
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let bg = egui::Color32::from_rgb(0x27, 0x28, 0x22);
        let stream_color = egui::Color32::from_rgb(0xb0, 0xae, 0xa9);
        let timer_color = egui::Color32::from_rgb(0xe8, 0xe6, 0xe1);
        let qr_color = timer_color;

        let elapsed = self.start.elapsed();
        let h = elapsed.as_secs() / 3600;
        let m = (elapsed.as_secs() % 3600) / 60;
        let s = elapsed.as_secs() % 60;
        let timer_text = format!("{:02}:{:02}:{:02}", h, m, s);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(bg).inner_margin(0.0))
            .show(ctx, |ui| {
                let total_w = ui.available_width();
                let total_h = ui.available_height();
                let left_w = total_w * 2.0 / 3.0;
                let right_w = total_w - left_w;

                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                ui.horizontal(|ui| {
                    // 左侧文本
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(left_w, total_h),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.vertical_centered(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                                let available = ui.available_size_before_wrap();
                                let pad = 8.0;
                                let stream_h = 28.0 * 1.15;
                                let timer_h = 56.0 * 1.15;
                                let total_content_h = stream_h + pad + timer_h;
                                let top_space = (available.y - total_content_h) / 2.0;

                                ui.add_space(top_space.max(pad));
                                ui.label(
                                    egui::RichText::new("Streaming")
                                        .size(28.0)
                                        .color(stream_color),
                                );
                                ui.add_space(pad);
                                ui.label(
                                    egui::RichText::new(timer_text)
                                        .size(56.0)
                                        .color(timer_color),
                                );
                            });
                        },
                    );

                    // 右侧二维码
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(right_w, total_h),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            let rect = ui.max_rect();
                            let pad = 12.0;
                            let max_draw =
                                (rect.width() - pad * 2.0).min(rect.height() - pad * 2.0);
                            let module_size = (max_draw / self.qr_size as f32).floor();
                            let qr_draw_size = (module_size * self.qr_size as f32).round();
                            let offset_x = (rect.center().x - qr_draw_size / 2.0).round();
                            let offset_y = (rect.center().y - qr_draw_size / 2.0).round();
                            let painter = ui.painter();
                            for (i, &is_dark) in self.qr_modules.iter().enumerate() {
                                if is_dark {
                                    let mx = (i % self.qr_size) as f32;
                                    let my = (i / self.qr_size) as f32;
                                    let x = (offset_x + mx * module_size).round();
                                    let y = (offset_y + my * module_size).round();
                                    painter.rect_filled(
                                        egui::Rect::from_min_size(
                                            egui::pos2(x, y),
                                            egui::Vec2::splat(module_size),
                                        ),
                                        0.0,
                                        qr_color,
                                    );
                                }
                            }
                        },
                    );
                });
            });

        ctx.request_repaint_after(Duration::from_millis(500));
    }
}
