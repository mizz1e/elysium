//! interfaced youre mother

use elysium_sdk::Interface;

#[inline]
pub fn load_interfaces() -> elysium_sdk::Interfaces {
    unsafe {
        elysium_sdk::Interfaces::from_loader(|kind| {
            let path = kind.library().path();
            let name = kind.name();

            let result = link::load_module(path);
            let module = match result {
                Ok(module) => module,
                Err(error) => panic!("{error:?}"),
            };

            let address = module
                .symbol("s_pInterfaceRegs")
                .expect("interface registry")
                .symbol
                .address as *const *const Interface;

            let interfaces = &**address;
            let interface = interfaces.get(name);

            interface
        })
    }
}
