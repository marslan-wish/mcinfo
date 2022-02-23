use mcinfo::{error::MCError, human_size, human_time, opts::Opts, stats::Stats, Result};
use structopt::StructOpt;

use std::time::Duration;

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    str::from_utf8,
};

// print stats
fn print_stats(s: Vec<u8>, o: &Opts) -> Result<()> {
    let text = from_utf8(&s)?;
    let stats = text.parse::<Stats>()?;

    let server_cmd_set = stats
        .stats
        .get("cmd_set")
        .ok_or_else(|| MCError::Stat("cmd_set".to_string()))?
        .parse::<f64>()?;
    let server_cmd_get = stats
        .stats
        .get("cmd_get")
        .ok_or_else(|| MCError::Stat("cmd_get".to_string()))?
        .parse::<f64>()?;
    let item_size_max = stats
        .stats
        .get("item_size_max")
        .ok_or_else(|| MCError::Stat("item_size_max".to_string()))?
        .parse::<usize>()?;
    let max_bytes = stats
        .stats
        .get("maxbytes")
        .unwrap_or(&String::from("0"))
        .parse::<usize>()?;

    println!(
        "  # Item_Size    Max_Age  Pages   Count      Used_Mem     To_Store    Efficiency    Set_Usage    Evicted   Evict_Time   OOM"
    );

    let mut total_memory = 0;
    let mut total_requested_memory = 0;

    for (slab, stat) in &stats.slabs {
        let total_pages = stat.get("total_pages").unwrap_or(&0);

        let used_mem = total_pages * item_size_max;
        total_memory += used_mem;

        let mem_requested = *stat.get("mem_requested").unwrap_or(&0);
        total_requested_memory += mem_requested;

        println!(
            "{:>3} {:>9} {:>10} {:>6} {:>7} {:>13} {:>12} {:>11.2} % {:10.2} % {:>10} {:>12} {:>5}",
            slab,
            human_size(o.short, stat.get("chunk_size").unwrap_or(&0)),
            human_time(o.short, stat.get("age").unwrap_or(&0)),
            total_pages,
            stat.get("number").unwrap_or(&0),
            human_size(o.short, &used_mem),
            human_size(o.short, &mem_requested),
            mem_requested as f64 * 100.0 / used_mem as f64,
            *stat.get("cmd_set").unwrap_or(&0) as f64 * 100.0 / server_cmd_set,
            stat.get("evicted").unwrap_or(&0),
            human_time(o.short, stat.get("evicted_time").unwrap_or(&0)),
            stat.get("outofmemory").unwrap_or(&0)
        );
    }

    println!();
    println!(
        "{:<25}: {} (uptime {})",
        "Memcached Version",
        stats
            .stats
            .get("version")
            .ok_or_else(|| MCError::Stat("version not sent".to_string()))?,
        human_time(
            true,
            &stats
                .stats
                .get("uptime")
                .ok_or_else(|| MCError::Stat("uptime not sent".to_string()))?
                .parse::<usize>()?
        )
    );
    println!(
        "{:<25}: {}/{}",
        "Current/Max Connections",
        stats
            .stats
            .get("curr_connections")
            .ok_or_else(|| MCError::Stat("curr_connections not sent".to_string()))?,
        stats
            .stats
            .get("max_connections")
            .ok_or_else(|| MCError::Stat("max_connections not sent".to_string()))?
    );
    println!(
        "{:<25}: {} (provided by -m)",
        "Max Memory Allocation",
        human_size(true, &max_bytes)
    );
    println!(
        "{:<25}: {} (for {} of stored data)",
        "Total Used Memory",
        human_size(true, &total_memory),
        human_size(true, &total_requested_memory)
    );
    println!(
        "{:<25}: {:4.2} % ({} / {})",
        "Global Efficiency",
        total_requested_memory as f64 * 100.0 / total_memory as f64,
        human_size(true, &total_requested_memory),
        human_size(true, &total_memory)
    );
    println!(
        "{:<25}: {:4.2} %",
        "Global GET hits",
        stats
            .stats
            .get("get_hits")
            .unwrap_or(&String::from("0"))
            .parse::<f64>()?
            * 100.0
            / server_cmd_get
    );
    println!(
        "{:<25}: {:4.2} %",
        "Global GET misses",
        stats
            .stats
            .get("get_misses")
            .unwrap_or(&String::from("0"))
            .parse::<f64>()?
            * 100.0
            / server_cmd_get
    );

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::from_args();
    let mchost: SocketAddr = match format!("{}:{}", opts.host, opts.port).to_socket_addrs() {
        Ok(mut v) => v.next().expect("need a resolved IP"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match TcpStream::connect_timeout(&mchost, Duration::from_secs(opts.timeout.into())) {
        Ok(mut stream) => {
            stream
                .write_all(b"stats\r\nstats slabs\r\nstats items\r\nstats settings\r\nquit\r\n")?;

            let mut data = vec![];
            match stream.read_to_end(&mut data) {
                Ok(_) => {
                    print_stats(data, &opts)?;
                }
                Err(e) => {
                    eprintln!("Error receiving response from the server: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error connecting: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}
