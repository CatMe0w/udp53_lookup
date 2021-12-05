use std::io::{Read, stdin, stdout, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::process;
use trust_dns_resolver::config::{NameServerConfigGroup, ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

fn pseudo_pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress [Enter] to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn graceful_shutdown(code: i32) {
    pseudo_pause();
    process::exit(code);
}

fn main() {
    println!("Testing now...");
    let resolver = Resolver::new(
        ResolverConfig::from_parts(
            None,
            vec![],
            NameServerConfigGroup::from_ips_clear(
                &[IpAddr::V4(Ipv4Addr::new(120, 24, 82, 54))],
                53,
                true),
        ),
        ResolverOpts::default()).unwrap();
    let response = resolver.lookup_ip("play.pixelparadise.gg.").unwrap();
    let address = response.iter().next().expect("No address returned!");
    if address != IpAddr::V4(Ipv4Addr::new(120, 24, 82, 54)) {
        println!("UDP 53 test failed. Legitimate DNS responses are being modified. You are out of luck and even a DNS tunnel could not help.");
    } else {
        println!("UDP 53 test ok. You may want to deploy an OpenVPN server or DNS tunnel to bypass Web Auth.");
    }
    graceful_shutdown(0);
}
