use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ gdk, Application, ApplicationWindow, EventControllerMotion };
use gtk4_layer_shell::{ Edge, KeyboardMode, Layer, LayerShell };
use std::cell::{ Cell, RefCell };
use std::rc::Rc;
use webkit6::{ prelude::*, WebView };

const BAR_HEIGHT: i32 = 400;
const BAR_WIDTH: i32 = 700;
// const PEEK: i32 = 2;
const PEEK: i32 = 2;
const ANIM_MS: u32 = 120;

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum BarPos {
    Top,
    Bottom,
}
const BAR_POS: BarPos = BarPos::Bottom;

fn bar_edge() -> Edge {
    match BAR_POS {
        BarPos::Top => Edge::Top,
        BarPos::Bottom => Edge::Bottom,
    }
}

fn main() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "wayland");
    }

    let app = Application::builder().application_id("dev.luis.simplebar").build();
    app.connect_activate(build_ui);
    app.run();
}

// ----------------- CSS GTK (toplevel transparente) -----------------
fn install_css() {
    let css =
        r#"
    window.transparent {
        background: transparent;
        box-shadow: none;
    }
    "#;

    let provider = gtk::CssProvider::new();
    provider.load_from_data(css);
    let display = gdk::Display::default().expect("No display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}

// ----------------- Layer Shell -----------------
fn setup_layer_shell(win: &ApplicationWindow) {
    win.init_layer_shell();
    win.set_namespace(Some("simplebar"));
    win.set_layer(Layer::Overlay);

    let edge = bar_edge();
    win.set_anchor(edge, true); // solo ancla en el borde elegido
    win.set_keyboard_mode(KeyboardMode::OnDemand);
    win.set_exclusive_zone(0); // flotante (no reserva espacio)
    win.set_height_request(BAR_HEIGHT);
    win.set_width_request(BAR_WIDTH);
    win.set_default_height(BAR_HEIGHT);
    win.set_default_width(BAR_WIDTH);
    win.add_css_class("transparent");
}

// ----------------- Comportamiento peek/slide -----------------
#[allow(dead_code)] // estos campos existen para mantener vivo el estado de animación
struct SlideHover {
    current_margin: Rc<Cell<i32>>,
    target_margin: Rc<Cell<i32>>,
    anim_source_id: Rc<RefCell<Option<glib::SourceId>>>,
    hidden_margin: i32,
}

impl SlideHover {
    fn attach(window: &ApplicationWindow, hover_surface: &impl IsA<gtk::Widget>) -> Self {
        let edge = bar_edge();
        let hidden_margin = -(BAR_HEIGHT - PEEK);
        window.set_margin(edge, hidden_margin);

        let current_margin = Rc::new(Cell::new(hidden_margin));
        let target_margin = Rc::new(Cell::new(hidden_margin));
        let anim_source_id: Rc<RefCell<Option<glib::SourceId>>> = Rc::new(RefCell::new(None));

        let animate = {
            let window = window.clone();
            let current_margin = current_margin.clone();
            let target_margin = target_margin.clone();
            let anim_source_id = anim_source_id.clone();

            move |to: i32| {
                target_margin.set(to);
                if anim_source_id.borrow().is_some() {
                    return;
                }
                let start = current_margin.get();
                let distance = (to - start).abs().max(1) as f32;
                let step = (distance / ((ANIM_MS as f32) / 16.0)).ceil().max(1.0) as i32;

                let id = glib::timeout_add_local(std::time::Duration::from_millis(16), {
                    let window = window.clone();
                    let current_margin = current_margin.clone();
                    let target_margin = target_margin.clone();
                    let anim_source_id = anim_source_id.clone();
                    let edge = edge;

                    move || {
                        let cur = current_margin.get();
                        let to = target_margin.get();

                        if cur == to {
                            *anim_source_id.borrow_mut() = None;
                            return glib::ControlFlow::Break;
                        }
                        let dir = if to > cur { 1 } else { -1 };
                        let mut next = cur + dir * step;
                        if (to - next).signum() != (to - cur).signum() {
                            next = to;
                        }

                        current_margin.set(next);
                        window.set_margin(edge, next);
                        glib::ControlFlow::Continue
                    }
                });
                *anim_source_id.borrow_mut() = Some(id);
            }
        };

        // Hover enter/leave sobre la propia ventana (cubre el WebView)
        let motion = EventControllerMotion::new();
        {
            let animate_in = animate.clone();
            motion.connect_enter(move |_, _, _| animate_in(0));
        }
        {
            let animate_out = animate.clone();
            motion.connect_leave(move |_| animate_out(hidden_margin));
        }
        hover_surface.add_controller(motion);

        Self { current_margin, target_margin, anim_source_id, hidden_margin }
    }
}

// ----------------- App -----------------
fn build_ui(app: &Application) {
    install_css();

    let display = gdk::Display::default().expect("No display");
    let monitors = display.monitors();
    let n_monitors = monitors.n_items();
    for i in 0..n_monitors {
        if let Some(monitor) = monitors.item(i).and_then(|obj| obj.downcast::<gdk::Monitor>().ok()) {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("simplebar")
                .decorated(false)
                .resizable(false)
                .build();
            setup_layer_shell(&window);
            window.set_monitor(Some(&monitor));

            let webview = WebView::new();
            // webview.grab_focus();
            // webview.connect_load_changed(|w, _| {
            //     w.grab_focus();
            // });

            webview.set_background_color(&gdk::RGBA::new(0.0, 0.0, 0.0, 0.0));
            if let Some(settings) = webkit6::prelude::WebViewExt::settings(&webview) {
                settings.set_allow_file_access_from_file_urls(true);
                settings.set_allow_universal_access_from_file_urls(true);
                settings.set_enable_developer_extras(true);
            }

            let port = std::env::var("APP_PORT").unwrap_or_else(|_| "2002".into());
            webview.load_uri(&format!("http://localhost:{}/", port));

            let _slide = SlideHover::attach(&window, &window);
            window.set_child(Some(&webview));
            window.present();
        }
    }
}
