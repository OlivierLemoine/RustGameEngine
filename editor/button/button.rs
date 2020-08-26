#[no_mangle]
unsafe extern "C" fn on_click() {
    println!("Test from inside the dll");
}
