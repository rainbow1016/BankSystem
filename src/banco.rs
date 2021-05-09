#[allow(dead_code)]
pub mod services_bank {
    use crate::{banco::services_bank, cuenta, usuario};
    use std::io;
    #[allow(unused_variables, dead_code)]
    pub fn crear_cuenta(user: usuario::Usuario, lista: &mut Vec<cuenta::Cuenta>) {
        lista.push(cuenta::Cuenta::new(user));
    }

    #[allow(unused_variables, dead_code)]
    pub fn consignar(lista_de_cuents: &mut Vec<cuenta::Cuenta>) {
        // ingrese de id de la cuenta que ingresara el dinero
        println!("Ingrese el id de la cuenta a la que consignara el dinero");
        let mut numero_id_origen = String::new();
        let entrada1 = io::stdin().read_line(&mut numero_id_origen).unwrap();
        let id_origen: u32 = numero_id_origen.trim().parse().unwrap();

        for val in lista_de_cuents.iter_mut() {
            // ingreso y comprobacion de fondos
            if val.id_cuenta == id_origen {
                println!("Ingrese el monto a consignar");
                let mut monto = String::new();
                let entrada3 = io::stdin().read_line(&mut monto).unwrap();
                let cantidad_dinero: i64 = monto.trim().parse().unwrap();

                cuenta::Cuenta::consignar_saldo(val, cantidad_dinero);
            } else {
                println!("Cuenta no encontrada");
            }
        }
    }

    #[allow(unused_variables, dead_code)]
    pub fn retirar(lista_de_cuents: &mut Vec<cuenta::Cuenta>) {
        // ingrese de id de la cuenta que ingresara el dinero
        println!("Ingrese el id de la cuenta que retirara el dinero");
        let mut numero_id_origen = String::new();
        let entrada1 = io::stdin().read_line(&mut numero_id_origen).unwrap();
        let id_origen: u32 = numero_id_origen.trim().parse().unwrap();

        for val in lista_de_cuents.iter_mut() {
            // ingreso y comprobacion de fondos
            println!("Ingrese el monto a retirar");
            let mut monto = String::new();
            let entrada3 = io::stdin().read_line(&mut monto).unwrap();
            let cantidad_dinero: i64 = monto.trim().parse().unwrap();

            if val.saldo < cantidad_dinero {
                println!("Fondos insuficientes");
            } else {
                cuenta::Cuenta::retirar_saldo(val, cantidad_dinero);
            }
        }
    }

    #[allow(unused_variables, dead_code)]
    pub fn transferir(lista_de_cuents: &mut Vec<cuenta::Cuenta>) {
        // ingrese de id de la cuenta que ingresara el dinero
        println!("Ingrese el id de la cuenta de origen");
        let mut numero_id_origen = String::new();
        let entrada1 = io::stdin().read_line(&mut numero_id_origen).unwrap();
        let id_origen: u32 = numero_id_origen.trim().parse().unwrap();

        // ingreso de id de la cuenta que recibira el dinero
        println!("Ingrese el id de la cuenta de destino");
        let mut numero_id_destino = String::new();
        let entrada2 = io::stdin().read_line(&mut numero_id_destino).unwrap();
        let id_destino: u32 = numero_id_destino.trim().parse().unwrap();

        let mut temp_list: Vec<cuenta::Cuenta> = lista_de_cuents.clone();

        for val in lista_de_cuents.iter_mut() {
            if val.id_cuenta == id_origen {
                // ingreso y comprobacion de fondos
                println!("Ingrese el monto a transferir");
                let mut monto = String::new();
                let entrada3 = io::stdin().read_line(&mut monto).unwrap();
                let cantidad_dinero: i64 = monto.trim().parse().unwrap();

                if val.saldo >= cantidad_dinero {
                    // se itera sobre la lista temporal para cambiar el saldo de la cuenta X
                    for val2 in temp_list.iter_mut() {
                        if val2.id_cuenta == id_destino {
                            cuenta::Cuenta::gestionar_saldo(val, cantidad_dinero);
                        }
                    }
                }
            }
        }
    }

    #[allow(unused_variables, dead_code)]
    pub fn consultar_cuenta(lista_de_cuents: &mut Vec<cuenta::Cuenta>) {
        // ingrese de id de la cuenta que ingresara el dinero
        println!("Ingrese el id del usuario que desea consultar");
        let mut numero_id_cuenta = String::new();
        let entrada = io::stdin().read_line(&mut numero_id_cuenta).unwrap();
        let id_origen: u32 = numero_id_cuenta.trim().parse().unwrap();

        for val in lista_de_cuents.iter() {
            cuenta::Cuenta::consultar_cuenta(val);
        }
    }

    #[allow(unused_variables, dead_code)]
    pub fn app() {
        let mut _cuentas_usuario: Vec<cuenta::Cuenta> = Vec::new();

        loop {
            println!("Escoga una opcion");
            println!("1. Crear una cuenta");
            println!("2. Consignar dinero");
            println!("3. Retirar dinero");
            println!("4. Transferir dinero");
            println!("5. Consultar cuenta");
            println!("6. Salir");
            println!(">");

            let mut entrada = String::new();
            let _entry = io::stdin().read_line(&mut entrada).unwrap();
            let eleccion: u8 = entrada.trim().parse().unwrap();
            if eleccion == 6 {
                break;
            }
            match eleccion {
                1 => {
                    let nuevo: usuario::Usuario = usuario::Usuario::new();
                    services_bank::crear_cuenta(nuevo, &mut _cuentas_usuario);
                }
                2 => {
                    services_bank::consignar(&mut _cuentas_usuario);
                }
                3 => {
                    services_bank::retirar(&mut _cuentas_usuario);
                }
                4 => {
                    services_bank::transferir(&mut _cuentas_usuario);
                }
                5 => {
                    services_bank::consultar_cuenta(&mut _cuentas_usuario);
                }
                _ => {
                    println!("Numero equivocado, vuelva a intentar");
                }
            }
        }
    }
}
