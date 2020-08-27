// #[path = "../../src/engine/mod.rs"]
// mod engine;

#[no_mangle]
unsafe extern "C" fn on_click() {
    println!("Test from inside the dll");
}
