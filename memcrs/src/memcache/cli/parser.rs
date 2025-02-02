use byte_unit::{Byte};
use clap::{command, Parser, ValueEnum};
use std::{net::IpAddr, ops::RangeInclusive, fmt::Debug};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum RuntimeType {
    /// work handled withing current thread runtime
    CurrentThread,
    /// work stealing threadpool runtime
    MultiThread,
}

impl RuntimeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RuntimeType::CurrentThread => "Work handled withing current thread runtime",
            RuntimeType::MultiThread => "Work stealing threadpool runtime",
        }
    }
}

const DEFAULT_PORT: u16 = 11211;
const DEFAULT_ADDRESS: &str = "127.0.0.1";
const CONNECTION_LIMIT: u32 = 1024;
const LISTEN_BACKLOG: u32 = 1024;
const MEMORY_LIMIT: &str = "64MiB";
const MAX_ITEM_SIZE: &str = "1MiB";

fn get_default_threads_number() -> usize {
    num_cpus::get_physical().to_string().parse().unwrap()
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
/// memcached compatible server implementation in Rust
pub struct MemcrsArgs {
    #[arg(short, long, value_name = "PORT", value_parser = port_in_range, default_value_t = DEFAULT_PORT)]
    /// TCP port to listen on
    pub port: u16,

    #[arg(short, long, value_name = "CONNECTION-LIMIT", default_value_t = CONNECTION_LIMIT)]
    /// max simultaneous connections
    pub connection_limit: u32,

    #[arg(short, long, value_name = "LISTEN-BACKLOG", default_value_t = LISTEN_BACKLOG)]
    /// set the backlog queue limit
    pub backlog_limit: u32,

    #[arg(short, long, value_name = "MEMORY-LIMIT", value_parser = parse_memory_mb, default_value = MEMORY_LIMIT)]
    /// memory limit in megabytes
    pub memory_limit: u64,

    #[arg(short, long, value_name = "MAX-ITEM-SIZE", default_value_t = Byte::from_str(MAX_ITEM_SIZE).unwrap())]
    ///  adjusts max item size (min: 1k, max: 1024m)
    pub item_size_limit: Byte,

    #[arg(short, long, value_name = "THREADS", default_value_t = get_default_threads_number())]
    /// number of threads to use
    pub threads: usize,

    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 1)]
    /// sets the level of verbosity
    pub verbose: u8,

    #[arg(short, long, value_name = "listen", default_value_t = String::from(DEFAULT_ADDRESS).parse::<IpAddr>().unwrap())]
    /// interface to listen on
    pub listen_address: IpAddr,

    #[arg(short, long, value_name = "RUNTIME-TYPE", default_value_t = RuntimeType::CurrentThread, value_enum)]
    ///  runtime type to use
    pub runtime_type: RuntimeType,
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn parse_memory_mb(s: &str) -> Result<u64, String> {
    match Byte::from_str(s) {
        Ok(bytes) => {
            Ok(bytes.get_bytes().try_into().unwrap())
        },
        Err(byte_error) => {
            Err(format!(
                "{}",
                byte_error
            ))
        }
    }
}

impl MemcrsArgs {
    fn from_args(args: Vec<String>) -> Result<MemcrsArgs, String> {
        let memcrs_args = MemcrsArgs::parse_from(args.iter());
        Ok(memcrs_args)
    }
}

pub fn parse(args: Vec<String>) -> Result<MemcrsArgs, String> {
    MemcrsArgs::from_args(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn verify_cli() {
        MemcrsArgs::command().debug_assert()
    }
}
