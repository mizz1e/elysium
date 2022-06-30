use elysium_mem::Shared;

extern "C" {
    fn printf(fmt: *const u8, ...);
}

fn main() {
    let shared = Shared::new();
    let module = shared.module_of(printf as usize as *const u8);

    println!("printf is in {module:?}");
}
