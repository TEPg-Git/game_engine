fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();

        res.set_icon("assets/logo/Logo.ico");

        res.compile().unwrap();
    }
}
