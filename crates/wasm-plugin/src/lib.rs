wit_bindgen::generate!({
    world: "runner",

    exports: {
        world: crate::A,
    }

});

struct A;

impl Guest for A {
    fn run() {
        let now = std::time::Instant::now();
        log(&format!("now: {now:?}"));
        let game::plugin::version::V {
            major,
            minor,
            release,
        } = game::plugin::version::get_version();

        log(&format!("{major}.{minor}.{release}"));
    }
}
