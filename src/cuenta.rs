use crate::usuario;

#[derive(Debug, PartialEq, Eq)]
pub struct Cuenta {
    num_cuenta: u64,
    fec_ven: String,
    cvv: u8,
    saldo: i64,
    usuario: Option<usuario::Usuario>,
}

#[allow(dead_code, unused_variables)]
impl Cuenta {
    pub fn new(
        num_cuenta: u64,
        fec_ven: String,
        cvv: u8,
        saldo: i64,
        usuario: Option<usuario::Usuario>,
    ) -> Self {
        Cuenta {
            num_cuenta,
            fec_ven,
            cvv,
            saldo,
            usuario,
        }
    }
}
