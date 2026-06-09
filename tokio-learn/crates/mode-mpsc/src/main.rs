use gpui::{
    App, AppContext, Application, Bounds, Context, InteractiveElement, IntoElement, ParentElement,
    PathPromptOptions, Render, SharedString, StatefulInteractiveElement, Styled, Window,
    WindowBounds, WindowOptions, colors::Colors, div, px, rgb, size,
};
use mode_mpsc::{init_service, process_batch, transform::TransformationType};
use tokio::runtime::Handle;

struct AppState {
    paths: Vec<String>,
    status: SharedString,
    rt: Handle,
}

impl AppState {
    fn new(rt: Handle) -> Self {
        Self {
            paths: vec![],
            status: "No file selected".into(),
            rt,
        }
    }
}

impl Render for AppState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let paths_display = if self.paths.is_empty() {
            "No file selected".to_string()
        } else if self.paths.len() == 1 {
            self.paths[0].clone()
        } else {
            format!("{} (and {} more)", self.paths[0], self.paths.len() - 1)
        };

        let colors = Colors::for_appearance(window);
        let text_color = colors.text;

        let select_btn = div()
            .id("select-files")
            .px_3()
            .py_1()
            .bg(rgb(0xe0e0e0))
            .text_color(rgb(0x000000))
            .rounded_sm()
            .cursor_pointer()
            .child("Select Files")
            .on_click(cx.listener(|_this, _, window, cx| {
                let entity = cx.entity();
                window
                    .spawn(cx, async move |cx| {
                        let rx = cx
                            .update(|_window, app| {
                                app.prompt_for_paths(PathPromptOptions {
                                    files: true,
                                    directories: false,
                                    multiple: true,
                                    prompt: Some("Select video files".into()),
                                })
                            })
                            .unwrap();

                        if let Ok(Ok(Some(paths))) = rx.await {
                            let paths: Vec<String> = paths
                                .into_iter()
                                .map(|p| p.to_string_lossy().to_string())
                                .collect();
                            cx.update_entity(&entity, |state, _cx| {
                                state.paths = paths;
                                state.status = "Files selected".into();
                            })
                            .ok();
                        }
                    })
                    .detach();
            }));

        let submit_btn = div()
            .id("submit")
            .px_3()
            .py_1()
            .bg(rgb(0x4a90d9))
            .text_color(rgb(0xffffff))
            .rounded_sm()
            .cursor_pointer()
            .child("Submit")
            .on_click(cx.listener(|this, _, window, cx| {
                if this.paths.is_empty() {
                    return;
                }
                let paths = this.paths.clone();
                let entity = cx.entity();
                let rt = this.rt.clone();
                window
                    .spawn(cx, async move |cx| {
                        let result = rt
                            .spawn(async move {
                                process_batch(paths, TransformationType::Video2Wav).await;
                            })
                            .await;

                        let status = match result {
                            Ok(_) => "Success!".to_string(),
                            Err(e) => format!("Error: {}", e),
                        };

                        cx.update_entity(&entity, |state, _cx| {
                            state.status = status.into();
                        })
                        .ok();
                    })
                    .detach();
            }));

        div()
            .flex()
            .flex_col()
            .p_4()
            .gap_3()
            .bg(rgb(0x2c2e2f))
            .text_color(text_color)
            .w(px(500.0))
            .h(px(300.0))
            .child("Video2Wav Converter")
            .child(div().child(paths_display))
            .child(select_btn)
            .child(submit_btn)
            .child(div().child(self.status.clone()))
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let rt_handle = rt.handle().clone();
    let _guard = rt.enter();

    init_service(4);

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(300.0)), cx);
        let handle = rt_handle.clone();

        cx.on_window_closed(|cx| {
            mode_mpsc::shutdown_service();
            cx.quit();
        })
        .detach();

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |_, cx| cx.new(move |_| AppState::new(handle.clone())),
        )
        .unwrap();
        cx.activate(true);
    });
}
