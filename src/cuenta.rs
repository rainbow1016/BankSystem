use crate::usuario;
use rand::Rng;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cuenta {
    pub id_cuenta: u32,
    num_cuenta: u64,
    pub fec_ven: String,
    cvv: u32,
    pub saldo: i64,
    pub usuario: Option<usuario::Usuario>,
}

#[allow(dead_code, unused_variables)]
impl Cuenta {
    pub fn new(user: usuario::Usuario) -> Cuenta {
        // Generacion de un numero de cuenta
        let mut rango = rand::thread_rng();
        let num_cuenta = rango.gen_range(1000..10000);

        // Ingreso de fecha de vencimiento
        println!("Ingrese la fecha de vencimiento de la cuenta");
        let mut fec_ven = String::new();
        let entrada = io::stdin().read_line(&mut fec_ven).unwrap();

        // Generacion automatica de cvv
        let cvv = rango.gen_range(100..999);

        println!("Cantidad de saldo inicial: ");
        let mut in_saldo = String::new();
        let entrada = io::stdin().read_line(&mut in_saldo).unwrap();
        let saldo = in_saldo.trim().parse().unwrap();

        println!("ingrese el id de la cuenta ");
        let mut in_id = String::new();
        let entrada2 = io::stdin().read_line(&mut in_id).unwrap();
        let id_cuenta = in_id.trim().parse().unwrap();

        Cuenta {
            id_cuenta,
            num_cuenta,
            fec_ven: fec_ven.lines().next().unwrap().to_string(),
            cvv,
            saldo,
            usuario: Some(user),
        }
    }
    pub fn gestionar_saldo(cuenta: &mut Cuenta, saldo: i64) {
        if saldo >= 0 {
            cuenta.saldo -= saldo
        } else {
            cuenta.saldo += saldo
        }
    }
    pub fn retirar_saldo(cuenta: &mut Cuenta, cantidad: i64) {
        cuenta.saldo -= cantidad;
        println!(
            "Usted a retirado:{}\nLe resta de saldo:{:?}",
            cantidad, cuenta.saldo
        );
    }
    pub fn consignar_saldo(cuenta: &mut Cuenta, cantidad: i64) {
        cuenta.saldo += cantidad;
        println!(
            "Usted a consignado:{}\nLe resta de saldo:{:?}",
            cantidad, cuenta.saldo
        );
    }
    pub fn consultar_cuenta(cuenta: &Cuenta) {
        println!(
            "El id de la cuenta es:{}
            \nLa fecha de vencimiento es:{}
            \nEl saldo disponible es:{}
            \nLe pertenece al usuario:{:?}",
            cuenta.id_cuenta,
            cuenta.fec_ven.lines().next().unwrap(),
            cuenta.saldo,
            cuenta.usuario
        )
    }
}
