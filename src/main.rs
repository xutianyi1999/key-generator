use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();

    let list: Vec<String> = args.collect();
    let cert = rcgen::generate_simple_self_signed(list)?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    fs::write("./cert.der", cert_der)?;
    fs::write("./priv_key", priv_key)?;
    Ok(())
}
