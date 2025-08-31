use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{ gdk, ApplicationWindow };
use gtk4_layer_shell::{ KeyboardMode, Layer, LayerShell };

use crate::config::{ bar_edge, BAR_HEIGHT, BAR_WIDTH };

pub fn install_css() {
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

pub fn setup_layer_shell(win: &ApplicationWindow) {
    win.init_layer_shell();
    win.set_namespace(Some("well-winget"));
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
