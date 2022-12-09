pub fn try_demangle(symbol: &str) -> Option<String> {
    match rustc_demangle::try_demangle(symbol) {
        Ok(symbol) => Some(symbol.to_string()),
        Err(_error) => match cpp_demangle::Symbol::new(symbol) {
            Ok(symbol) => Some(symbol.to_string()),
            Err(_error) => None,
        },
    }
}
