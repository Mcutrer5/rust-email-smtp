extern crate systemstat;

use std::thread;
use std::time::Duration;
use systemstat::{Platform, System};

pub fn get_data() -> (
    systemstat::Memory,
    systemstat::Swap,
    systemstat::Duration,
    systemstat::OffsetDateTime,
    systemstat::CPULoad,
    systemstat::SocketStats,
) {
    let sys = System::new();

    let mem = match sys.memory() {
        Ok(mem) => mem,
        Err(x) => panic!("Memory error: {}", x),
    };

    let swap = match sys.swap() {
        Ok(swap) => swap,
        Err(x) => panic!("Swap error: {}", x),
    };

    let uptime = match sys.uptime() {
        Ok(uptime) => uptime,
        Err(x) => panic!("Uptime error: {}", x),
    };

    let boot = match sys.boot_time() {
        Ok(boot_time) => boot_time,
        Err(x) => panic!("Boot time error: {}", x),
    };

    let cpu_load = match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            // println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
            //     cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
            cpu
        }
        Err(x) => panic!("CPU load error: {}", x),
    };

    let socket = match sys.socket_stats() {
        Ok(stats) => stats,
        Err(x) => panic!("Socket stats error: {}", x),
    };

    (mem, swap, uptime, boot, cpu_load, socket)
}
