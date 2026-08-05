pub const ORGANIZATION: &str = "embedded-alerts";
pub const SERVICES: &[&str] = &[
    "eal-interfaces",
    "eal-api",
    "eal-web-mash",
    "eal-web-leptos",
    "eal-web-dioxus",
    "eal-cli",
    "eal-sync",
    "eal-infra",
    "eal-clients",
    "eal-libs",
];

pub fn validate() -> Result<(), &'static str> {
    if SERVICES.is_empty() { return Err("service catalog is empty"); }
    if SERVICES.iter().any(|service| service.trim().is_empty()) { return Err("service name is empty"); }
    let mut sorted = SERVICES.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.len() != SERVICES.len() { return Err("duplicate service name"); }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn catalog_is_valid() { assert!(super::validate().is_ok()); }
}
