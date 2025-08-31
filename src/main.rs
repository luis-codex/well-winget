fn main() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "wayland");
    }
    well_winget::app::run();
}
