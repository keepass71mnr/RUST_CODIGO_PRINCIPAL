// src/main.rs

// 1. Esto le dice a Rust: "Busca un archivo llamado cuenta.rs o una carpeta cuenta/mod.rs y compílalo"
mod cuenta;

// 2. Esto importa la estructura específica para usarla directamente
use cuenta::CuentaBancaria;

fn main() {
    let mut mi_cuenta = CuentaBancaria::new("Carlos", 100.0);

    println!("Titular: {}", mi_cuenta.titular);
    println!("Saldo inicial: {}", mi_cuenta.consultar_saldo());

    mi_cuenta.depositar(50.0);
}
