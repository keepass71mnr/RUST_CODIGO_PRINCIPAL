#![windows_subsystem = "windows"]

mod load;
mod exec_mem;

use crate::exec_mem::exec_mem as ether_exec_mem;
use load::Loader;

fn main(){
    Loader::terminator();
}