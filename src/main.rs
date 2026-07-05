use gpui::*;
use gpui_component::color_picker::{ColorPicker, ColorPickerEvent, ColorPickerState};
use gpui_component::*;

pub struct AppState {
    color: Hsla,
    color_picker: Entity<ColorPickerState>,
}

impl AppState {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let color_picker = cx.new(|cx| ColorPickerState::new(window, cx).default_value(white()));

        cx.subscribe(&color_picker, |this, _, ev, cx| match ev {
            ColorPickerEvent::Change(color) => {
                if let Some(color) = color {
                    this.color = color.clone();
                    cx.notify()
                }
            }
        })
        .detach();

        Self {
            color: Hsla {
                h: 0.0,
                s: 1.0,
                l: 1.0,
                a: 1.0,
            },
            color_picker: color_picker,
        }
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .bg(self.color)
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Color Picker")
            .child(ColorPicker::new(&self.color_picker))
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| AppState::new(window, cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
