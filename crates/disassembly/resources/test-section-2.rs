#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[no_mangle]
pub extern "C" fn inc(x: i32) -> i32 {
    x + 1
}

#[no_mangle]
pub extern "C" fn show() {
    //
}
