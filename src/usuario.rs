use std::io;

#[derive(Debug, PartialEq, Eq)]
pub struct Usuario {
    nombre: String,
    cedula: u64,
}

#[allow(dead_code, unused_variables)]
impl Usuario {
    pub fn new() -> Self {
        let mut nombre = String::new();
        let entrada = io::stdin().read_line(&mut nombre).unwrap();

        let mut cedula_mem = String::new();
        let entrada2 = io::stdin().read_line(&mut cedula_mem).unwrap();
        let cedula: u64 = cedula_mem.trim().parse().unwrap();
        Usuario { nombre, cedula }
    }
}
