#[allow(dead_code)]
pub mod services_bank {
    use crate::{cuenta, usuario};
    use std::io;
    #[allow(unused_variables, dead_code)]
    pub fn crear_cuenta(user: usuario::Usuario) {
        cuenta::Cuenta::new(user);
    }

    #[allow(unused_variables, dead_code)]
    pub fn eliminar_cuenta(lista_de_cuents: &mut Vec<cuenta::Cuenta>) {
        //ingreso del id de la cuenta para ser comparado
        println!("Ingrese el id de su cuenta");
        let mut numero_id = String::new();
        let entrada = io::stdin().read_line(&mut numero_id).unwrap();
        let id: u32 = numero_id.trim().parse().unwrap();
        let mut count = 0;

        // TODO POR AQUI QUEDE
    }
    #[allow(unused_variables, dead_code)]
    pub fn consignar() {
        todo!()
    }
    #[allow(unused_variables, dead_code)]
    pub fn retirar() {
        todo!()
    }
    #[allow(unused_variables, dead_code)]
    pub fn transferir() {
        todo!()
    }
    #[allow(unused_variables, dead_code)]
    pub fn consultar_saldo() {
        todo!()
    }
    #[allow(unused_variables, dead_code)]
    pub fn app() {
        let mut _cuentas_usuario: Vec<cuenta::Cuenta> = Vec::new();

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
