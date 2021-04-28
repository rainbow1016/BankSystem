/*
    Pequeño sistema de banco, capaz de soportar cuentas y transsacciones sencillas alojadas en memeoria
    autor: Geardo Suarez
*/
// Modulos necesarios para crear el proyecto
mod banco;
mod cuenta;
mod usuario;

fn main() {
    banco::services_bank::new_program();
}
