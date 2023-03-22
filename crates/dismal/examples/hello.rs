use dismal::{FnPtr, Library};
use std::io;

#[inline(never)]
#[no_mangle]
pub extern "C" fn foo() {
    bar();
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn bar() {
    println!("hello");
}

fn main() -> io::Result<()> {
    let result = (foo as extern "C" fn()).disassemble();

    println!("{result:?}");

    let library = unsafe { Library::open("/usr/lib64/libGL.so")? };
    let ptr = library.get::<extern "C" fn()>(dismal::cstr!("glXGetProcAddress\0"))?;
    let result = ptr.disassemble();

    println!("{result:?}");

    let ptr = library.get::<u8>(dismal::cstr!("glXGetProcAddress\0"))?;
    let result = ptr.disassemble();

    println!("{result:?}");

    Ok(())
}
