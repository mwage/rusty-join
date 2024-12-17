pub mod encoder;
pub mod versions;

use std::process::Command;

pub fn sample_program(mut args: Vec<String>) {
    args.remove(0);
    Command::new("/usr/ftp/pub/anton/lvas/effizienz-aufgabe24/myjoin")
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}