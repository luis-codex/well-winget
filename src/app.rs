use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ gdk, Application, ApplicationWindow };
use webkit6::{ prelude::*, WebView };

use crate::animation::SlideHover;
use crate::ui::{ install_css, setup_layer_shell };
use gtk4_layer_shell::LayerShell;

pub fn build_ui(app: &Application) {
    install_css();

    let display = gdk::Display::default().expect("No display");
    let monitors = display.monitors();
    let n_monitors = monitors.n_items();
    for i in 0..n_monitors {
        if let Some(monitor) = monitors.item(i).and_then(|obj| obj.downcast::<gdk::Monitor>().ok()) {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("well-winget")
                .decorated(false)
                .resizable(false)
                .build();
            setup_layer_shell(&window);
            window.set_monitor(Some(&monitor));

            let webview = WebView::new();

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

pub fn run() {
    let app = Application::builder().application_id("dev.well-winget.qbytes").build();
    app.connect_activate(build_ui);
    app.run();
}
