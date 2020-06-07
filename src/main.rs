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
  println!("File name: {}", file_name);

  let content: Vec<u8>  = fs::read(file_name).expect("Could not find file");

  // Create, populate, and verify new EIdentStruct from first 16 bytes of file.

  if content.len() < EIdent::EIdent_Len {
      println!("Not enough data to populate EIdent portion of ELF Header");
      println!("File is not large enough to be an ELF binary.");
      std::process::exit(1);
  }
  let e_ident = EIdent::new(&content[0 .. 16]);
  if !e_ident.is_elf(){
      println!("File is not an ELF. Exiting");
      std::process::exit(1);
  }

  //TODO Account for endianness and scope (e_header dies after conditional)
  //let e_header: Headers;
  if e_ident.ei_class == 1 {
      let header: Header<Elf32> = Header::<Elf32>::new_partial(&content[16 .. 52], e_ident);
      print!("{}", header);
  }
  else {
      let header: Header<Elf64> = Header::<Elf64>::new(&content[0 ..=Header::<Elf64>::HEADER_LEN]);
      print!("{}", header);
  }
}
