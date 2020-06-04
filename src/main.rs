extern crate byteorder;

mod elf;
mod utility_functions;

use std::fs;
use structopt::StructOpt;
use dissect::argument_parsing::Cli;
use crate::elf::types::*;
use crate::elf::structures::*;
use crate::elf::enumerations::*;
use utility_functions::arrays::*;


//TODO REMOVE THIS MACRO WHEN FINISHED
#[allow(dead_code)]
fn main() {
  let args = Cli::from_args();
  let file_name: String = args.path.display().to_string();

  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  println!("DISSECT----------------------");
  println!("File to dissect: {}", file_name);

  let content: Vec<u8>  = fs::read(file_name).expect("Could not find file");

  // Create, populate, and verify new EIdentStruct from first 16 bytes of file.
  let e_ident = EIdentStruct::new(&content[0 .. 16]);
  if !e_ident.is_elf(){
      println!("File is not an ELF. Exiting");
      std::process::exit(1);
  }

  //TODO Account for endianness and scope (e_header dies after conditional)
  let e_header: Header;
  if e_ident.ei_class == 1 {
      e_header = Header::ELF32( ELFHeader32::new(&content[16 .. 52], e_ident) );

  }
  else {
      e_header = Header::ELF64( ELFHeader64::new(&content[16 .. 66], e_ident) );

  }
  match e_header {
    Header::ELF32(header) => println!("{}", header.fmt_cli_out()),
    _ => println!("Not implemented"),
  }
}


