use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let mode = args.next();

    if let Some(mode) = mode {
        let list: Vec<String> = args.collect();
        let cert = rcgen::generate_simple_self_signed(list)?;

        match mode.as_str() {
            "pem" => {
                let cert_pem = cert.cert.pem();
                let priv_key = cert.key_pair.serialize_pem();

                fs::write("./cert.pem", cert_pem)?;
                fs::write("./priv.key", priv_key)?;
                println!("Success");
            }
            "der" => {
                let cert_der = cert.cert.der().as_ref();
                let priv_key = cert.key_pair.serialize_der();

                fs::write("./cert.der", cert_der)?;
                fs::write("./priv.key", priv_key)?;
                println!("Success");
            }
            _ => eprintln!("Command error")
        }
    } else {
        eprintln!("Command error")
    }
    Ok(())
}
