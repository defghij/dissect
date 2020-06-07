pub mod argument_parsing {
  use structopt::StructOpt;
  #[derive(StructOpt)]
  pub struct Cli{
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
  }
}
