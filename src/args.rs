use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version)]
pub(crate) struct ClapInfoHostArgs {
    pub path: String,
}