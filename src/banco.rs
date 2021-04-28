#[allow(dead_code)]
pub mod services_bank {
    use std::io;

    use crate::cuenta;

    pub fn crear_cuenta() {
        todo!()
    }
    pub fn eliminar_cuenta() {
        todo!()
    }
    pub fn consignar() {
        todo!()
    }
    pub fn retirar() {
        todo!()
    }
    pub fn transferir() {
        todo!()
    }
    pub fn consultar_saldo() {
        todo!()
    }
    pub fn new_program() {
        let _cuentas_usuario: Vec<cuenta::Cuenta> = Vec::new();

        loop {
            println!("Escoga una opcion");
            println!("1. Crear una cuenta");
            println!("2. Eliminar una cuenta");
            println!("3. Consignar dinero");
            println!("4. Retirar dinero");
            println!("5. Transferir dinero");
            println!("6. Consultar saldo");
            println!("7. Salir");
            println!(">");

            let mut entrada = String::new();
            let _entry = io::stdin().read_line(&mut entrada).unwrap();
            let eleccion: u8 = entrada.trim().parse().unwrap();
            if eleccion == 7 {
                break;
            }
        }
    }
}
