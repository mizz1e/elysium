fn main() {
    link::iterate_modules(callback);

    println!(
        "callback = {:?}",
        link::query_address(callback as *const ())
    );

    println!(
        "printf = {:?}",
        link::query_address(libc::printf as *const ())
    );

    println!("is libc loaded = {:?}", link::is_module_loaded("libc.so.6"));

    println!(
        "is libc loaded = {:?}",
        link::is_module_loaded("/usr/x86_64-pc-linux-gnu/lib/libc.so.6\0")
    );

    println!(
        "is libc loaded = {:?}",
        link::is_module_loaded("/usr/x86_64-pc-linux-gnu/lib/libc.so.6\0\0\0\0\0")
    );

    let result = unsafe { link::load_module("/usr/x86_64-pc-linux-gnu/lib/libc.so.6\0\0\0\0\0") };

    println!("libc = {result:?}");

    let module = result.expect("libc");

    unsafe {
        let bytes = module.bytes();

        println!("{:02X?}", &bytes[..16]);

        let symbol = module.symbol("malloc");

        println!("{symbol:?}");
    }
}

fn callback(info: link::Module) {
    println!("{info:?}");
}
