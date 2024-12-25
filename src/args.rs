#[derive(Debug)]
pub struct Args {
    pub path: String
}

impl Args {
    /// Creates `Args` struct using `std::env::args()`
    /// 
    /// Returns `Ok(Args)`, iff 2 args (including the implict name of the binary) were supplied.
    /// 
    /// Otherwise returns `Result(&'static str)`
    pub fn parse() -> Result<Args, &'static str> {
        if std::env::args().len() != 2 {
            return Err("Wrong number of args. Usage: ./binary path/to/image/or/folder");
        }
        
        if let Some(path) = std::env::args().nth(1) {
            return Ok(Args { path });
        }
        
        Err("No argument supplied even though we just checked?")
        
    }
}
