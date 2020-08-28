#[path = "../../prelude.rs"]
mod prelude;

use prelude::*;

#[no_mangle]
unsafe extern "C" fn on_click(obj: &mut Object) {
    println!("Test from inside the dll");
    if let Some(t) = &mut obj.transform {
        t.position += Vector::new(0.1, 0.0);
    }
}
