fn main() {
    if let Err(error) = eal_catalog::validate() {
        eprintln!("invalid service catalog: {error}");
        std::process::exit(2);
    }
    println!("organization={}", eal_catalog::ORGANIZATION);
    for service in eal_catalog::SERVICES { println!("service={service}"); }
}
