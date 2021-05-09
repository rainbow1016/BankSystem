use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub cedula: u64,
}

#[allow(dead_code, unused_variables)]
impl Usuario {
    pub fn new() -> Usuario {
        // Ingreso de nombre usuario
        println!("Ingrese el nombre del usuario");
        let mut nombre = String::new();
        let entrada = io::stdin().read_line(&mut nombre).unwrap();

        // ingreso de su cedula
        println!("Ingrese la cedula del usuario");
        let mut cedula_mem = String::new();
        let entrada2 = io::stdin().read_line(&mut cedula_mem).unwrap();
        let cedula: u64 = cedula_mem.trim().parse().unwrap();
        Usuario {
            nombre: nombre.lines().next().unwrap().to_string(),
            cedula,
        }
    }
}
