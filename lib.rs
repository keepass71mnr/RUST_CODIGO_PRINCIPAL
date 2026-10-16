use std::fs;
use std::ptr;
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, VirtualProtect, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE,
    PAGE_EXECUTE_READ, PAGE_READWRITE,
};

/// Carga un archivo .bin con shellcode/código máquina en memoria y lo ejecuta.
pub fn exec_mem(file_path: &str) -> Result<(), String> {
    // 1. Leer el contenido del binario desde disco
    let code = fs::read(file_path)
        .map_err(|e| format!("Error al abrir el archivo '{}': {}", file_path, e))?;

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
            return Err("Falló la asignación de memoria con VirtualAlloc.".to_string());
        }

        ptr::copy_nonoverlapping(code.as_ptr(), addr as *mut u8, code.len());

        let mut old_protect = 0;
        let protect_res = VirtualProtect(
            addr,
            code.len(),
            PAGE_EXECUTE_READ,
            &mut old_protect,
        );

        if protect_res == 0 {
            VirtualFree(addr, 0, MEM_RELEASE);
            return Err("Falló VirtualProtect al establecer permisos de ejecución.".to_string());
        }

        let func: extern "system" fn() = std::mem::transmute(addr);
        func();

        VirtualFree(addr, 0, MEM_RELEASE);
    }

    Ok(())
}
