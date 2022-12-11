#![deny(warnings)]
#![feature(slice_split_at_unchecked)]
#![feature(strict_provenance)]

use goblin::elf::Elf;
use std::{fs, result};

pub use assembly::disassemble;
pub use error::Error;
pub use module::{Code, Module, Search};
pub use pattern::Pattern;

mod error;

pub mod assembly;
pub mod module;
pub mod pattern;
pub mod symbol;

pub type Result<T> = result::Result<T, Error>;

unsafe fn callback_inner(
    info: *mut libc::dl_phdr_info,
    _size: libc::size_t,
    data: *mut libc::c_void,
) -> Result<()> {
    let base_addr = (*info).dlpi_addr as usize;
    let path = module::determine_path((*info).dlpi_name)?;
    let bytes = fs::read(&path)?;
    let elf = Elf::parse(&bytes)?;
    let mut code = Vec::new();

    for header in &elf.section_headers {
        if !header.is_executable() {
            continue;
        }

        let offset = header.sh_addr as usize;
        let len = header.sh_size as usize;
        let name = elf
            .shdr_strtab
            .get_at(header.sh_name)
            .unwrap_or_default()
            .into();

        code.push(Code {
            base_addr,
            offset,
            len,
            name,
        });
    }

    /*for sym in &elf.syms {
        let Some(st_name) = elf.strtab.get_at(sym.st_name) else {
            continue;
        };

        let st_name = st_name
            .split_once('@')
            .map(|(symbol, _version)| symbol)
            .unwrap_or_else(|| st_name);

        let st_name = symbol::try_demangle(st_name).unwrap_or_else(|| st_name.into());
        let is_function = sym.is_function().then_some(" function").unwrap_or_default();
    }*/

    let modules = &mut *(data as *mut Vec<Module>);

    modules.push(Module { path, code });

    Ok(())
}

unsafe extern "C" fn callback(
    info: *mut libc::dl_phdr_info,
    size: libc::size_t,
    data: *mut libc::c_void,
) -> libc::c_int {
    let _ = callback_inner(info, size, data);

    0
}

pub fn modules() -> Vec<Module> {
    let mut modules = Vec::new();

    unsafe {
        libc::dl_iterate_phdr(
            Some(callback),
            &mut modules as *mut Vec<Module> as *mut libc::c_void,
        );
    }

    modules
}

pub fn get_module(name: &str) -> Option<Module> {
    for module in modules() {
        let Some(file_name) = module.path.file_name() else {
            continue;
        };

        if file_name == name {
            return Some(module);
        }
    }

    None
}
