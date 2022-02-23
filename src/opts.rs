use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    /// Memcached host to connect
    #[structopt(short, long, default_value = "localhost")]
    pub host: String,

    /// Memcached port to connect
    #[structopt(short, long, default_value = "11211")]
    pub port: u16,

    /// Connect timeout in seconds
    #[structopt(short, long, default_value = "1")]
    pub timeout: u8,

    /// Pretty prints time and byte size
    #[structopt(short, long)]
    pub short: bool,
}
