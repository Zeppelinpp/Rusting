use ab_glyph::{Font, FontRef, PxScale, ScaleFont, point};
use pixels::{Pixels, SurfaceTexture};
use qrcode::{Color, QrCode};
use std::time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 360;

fn blend_gamma(src: u8, dst: u8, a: f32) -> u8 {
    let src_lin = (src as f32 / 255.0).powf(2.2);
    let dst_lin = (dst as f32 / 255.0).powf(2.2);
    ((src_lin * a + dst_lin * (1.0 - a)).powf(1.0 / 2.2) * 255.0).min(255.0) as u8
}
const LEFT_WIDTH: u32 = WIDTH * 2 / 3;
const RIGHT_WIDTH: u32 = WIDTH - LEFT_WIDTH;

// 在这里填入你要生成二维码的 URL
const QR_URL: &str = "https://github.com/Zeppelinpp";

fn load_font() -> FontRef<'static> {
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
            let static_bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
            if let Ok(font) = FontRef::try_from_slice(static_bytes) {
                return font;
            }
        }
    }

    panic!("找不到可用字体。请在项目根目录放一个 font.ttf，或修改 load_font() 添加系统字体路径。");
}

struct App {
    window: Option<&'static Window>,
    pixels: Option<Pixels<'static>>,
    font: FontRef<'static>,
    qr_modules: Vec<bool>,
    qr_size: usize,
    start: Instant,
}

impl App {
    fn new() -> Self {
        let font = load_font();
        let qr = QrCode::new(QR_URL).expect("二维码生成失败");
        let qr_size = qr.width();
        let qr_modules = qr
            .to_colors()
            .into_iter()
            .map(|c| c == Color::Dark)
            .collect();

        Self {
            window: None,
            pixels: None,
            font,
            qr_modules,
            qr_size,
            start: Instant::now(),
        }
    }

    fn draw(&mut self) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();

        // 背景 (BGRA — pixels framebuffer 在原生平台为 Bgra8UnormSrgb)
        let bg = [0x22u8, 0x28, 0x27, 0xff];
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&bg);
        }

        // 计时器文本
        let elapsed = self.start.elapsed();
        let h = elapsed.as_secs() / 3600;
        let m = (elapsed.as_secs() % 3600) / 60;
        let s = elapsed.as_secs() % 60;
        let timer_text = format!("{:02}:{:02}:{:02}", h, m, s);

        let pad = 8.0f32;
        let (_, timer_h) = text_bounds(&self.font, &timer_text, 56.0);
        let (_, stream_h) = text_bounds(&self.font, "Streaming", 28.0);

        // Timer 严格居中于左侧区域
        let timer_center_y = HEIGHT as f32 / 2.0;
        let timer_area_y = timer_center_y - timer_h / 2.0;

        // Streaming 在 Timer 上方，间距 pad；保证不越界顶部
        let stream_area_y = (timer_area_y - pad - stream_h).max(pad);

        draw_text_centered(
            frame,
            &self.font,
            "Streaming",
            0,
            stream_area_y as u32,
            LEFT_WIDTH,
            stream_h as u32 + 1,
            28.0,
            [0xa9, 0xae, 0xb0, 0xff],
        );

        draw_text_centered(
            frame,
            &self.font,
            &timer_text,
            0,
            timer_area_y as u32,
            LEFT_WIDTH,
            timer_h as u32 + 1,
            56.0,
            [0xe1, 0xe6, 0xe8, 0xff],
        );

        // 右侧二维码（带 padding）
        let qr_pad = 12;
        draw_qr_centered(
            frame,
            &self.qr_modules,
            self.qr_size,
            LEFT_WIDTH + qr_pad,
            qr_pad,
            RIGHT_WIDTH - qr_pad * 2,
            HEIGHT - qr_pad * 2,
        );
    }
}

fn text_bounds(font: &FontRef<'static>, text: &str, size: f32) -> (f32, f32) {
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    let mut x = 0.0f32;
    let mut prev_id = None;

    for c in text.chars() {
        let glyph_id = scaled_font.glyph_id(c);
        if let Some(prev) = prev_id {
            x += scaled_font.kern(prev, glyph_id);
        }
        let glyph = ab_glyph::Glyph {
            id: glyph_id,
            scale,
            position: point(x, 0.0),
        };
        x += scaled_font.h_advance(glyph_id);
        prev_id = Some(glyph_id);
        glyphs.push(glyph);
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for glyph in &glyphs {
        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            let b = outlined.px_bounds();
            min_x = min_x.min(b.min.x);
            max_x = max_x.max(b.max.x);
            min_y = min_y.min(b.min.y);
            max_y = max_y.max(b.max.y);
        }
    }

    if min_x.is_infinite() {
        (0.0, 0.0)
    } else {
        (max_x - min_x, max_y - min_y)
    }
}

fn draw_text_centered(
    frame: &mut [u8],
    font: &FontRef<'static>,
    text: &str,
    area_x: u32,
    area_y: u32,
    area_w: u32,
    area_h: u32,
    size: f32,
    color: [u8; 4],
) {
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    let mut x = 0.0f32;
    let mut prev_id = None;

    for c in text.chars() {
        let glyph_id = scaled_font.glyph_id(c);
        if let Some(prev) = prev_id {
            x += scaled_font.kern(prev, glyph_id);
        }
        let glyph = ab_glyph::Glyph {
            id: glyph_id,
            scale,
            position: point(x, 0.0),
        };
        x += scaled_font.h_advance(glyph_id);
        prev_id = Some(glyph_id);
        glyphs.push(glyph);
    }

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for glyph in &glyphs {
        if let Some(outlined) = font.outline_glyph(glyph.clone()) {
            let b = outlined.px_bounds();
            min_x = min_x.min(b.min.x);
            max_x = max_x.max(b.max.x);
            min_y = min_y.min(b.min.y);
            max_y = max_y.max(b.max.y);
        }
    }

    if min_x.is_infinite() {
        return;
    }

    let text_w = max_x - min_x;
    let text_h = max_y - min_y;
    let offset_x = area_x as f32 + (area_w as f32 - text_w) / 2.0 - min_x;
    let offset_y = area_y as f32 + (area_h as f32 - text_h) / 2.0 - min_y;

    for glyph in glyphs {
        if let Some(outlined) = font.outline_glyph(glyph) {
            let b = outlined.px_bounds();
            outlined.draw(|x, y, c| {
                let px = (offset_x + b.min.x + x as f32) as i32;
                let py = (offset_y + b.min.y + y as f32) as i32;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let idx = ((py as u32 * WIDTH + px as u32) * 4) as usize;
                    frame[idx] = blend_gamma(color[0], frame[idx], c);
                    frame[idx + 1] = blend_gamma(color[1], frame[idx + 1], c);
                    frame[idx + 2] = blend_gamma(color[2], frame[idx + 2], c);
                    frame[idx + 3] = 255;
                }
            });
        }
    }
}

fn draw_qr_centered(
    frame: &mut [u8],
    qr_modules: &[bool],
    qr_size: usize,
    area_x: u32,
    area_y: u32,
    area_w: u32,
    area_h: u32,
) {
    let size = qr_size as u32;
    let max_draw = area_w.min(area_h);
    let module_size = max_draw / size;
    if module_size == 0 {
        return;
    }
    let qr_draw_size = module_size * size;
    let offset_x = area_x + (area_w - qr_draw_size) / 2;
    let offset_y = area_y + (area_h - qr_draw_size) / 2;

    let color = [0xe8u8, 0xe6, 0xe1, 0xff];

    for (i, &is_dark) in qr_modules.iter().enumerate() {
        if is_dark {
            let mx = (i % qr_size) as u32;
            let my = (i / qr_size) as u32;
            for dy in 0..module_size {
                for dx in 0..module_size {
                    let px = offset_x + mx * module_size + dx;
                    let py = offset_y + my * module_size + dy;
                    let idx = ((py * WIDTH + px) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&color);
                }
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title("Streaming Timer")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));
        let window = event_loop.create_window(attrs).unwrap();
        let window: &'static Window = Box::leak(Box::new(window));

        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);

        event_loop.set_control_flow(ControlFlow::Wait);
        if let Some(w) = self.window {
            w.request_redraw();
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(w) = self.window {
            w.request_redraw();
        }
        event_loop.set_control_flow(ControlFlow::wait_duration(Duration::from_millis(500)));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
                if self.pixels.as_ref().unwrap().render().is_err() {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;
    Ok(())
}
