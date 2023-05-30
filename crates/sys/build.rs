use {
    bindgen::{CargoCallbacks, NonCopyUnionStyle},
    std::{env, path::PathBuf},
};

fn main() {
    let bindings = bindgen::Builder::default()
        .array_pointers_in_arguments(true)
        .default_non_copy_union_style(NonCopyUnionStyle::ManuallyDrop)
        .explicit_padding(true)
        .header("src/bindings.hpp")
        .parse_callbacks(Box::new(CargoCallbacks))
        .sort_semantically(true)
        .use_core()
        .vtable_generation(true)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unablr to write bindings");
}
