extern crate byteorder;

//TODO REMOVE THIS MACRO WHEN FINISHED
#[allow(dead_code)]

mod elf;
mod utility_functions;

use std::fs;
use structopt::StructOpt;
use dissect::argument_parsing::Cli;
use crate::elf::types::*;
use crate::elf::structures::*;
use utility_functions::arrays::*;


#[allow(dead_code)]
fn main() {
  let args = Cli::from_args();
  let file_name: String = args.path.display().to_string();

  println!("File to dissect: {}", file_name);

  let content: Vec<u8>  = fs::read(file_name).expect("Could not find file");  // Read in contents of a file
                                                                                // and store them
                                                                                // into a Vec<u8>

  // Create, populate, and verify new EIdentStruct from first 16 bytes of file.
  let e_ident = EIdentStruct::new(&content[0 .. 16]);
  if !e_ident.is_elf(){
      println!("File is not an ELF. Exiting");
      std::process::exit(1);
  }

  
  //TODO Account for endianness and scope (e_header dies after conditional)
  if e_ident.ei_class == 1 {
      let e_header = ELFHeader32::new(&content[16 .. 52], e_ident);

      //TODO Actually do error checking on this print.
      println!("{}", e_header.fmt_cli_out());
  }
  else {
      let e_header = ELFHeader64::new(&content[16 .. 66], e_ident);

  }
}


