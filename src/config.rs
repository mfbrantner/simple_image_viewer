use crate::Args;

#[derive(Debug)]
pub enum InvocationMode {
    File,
    Directory,
}

#[derive(Debug)]
pub struct Config {
    pub args: Args,
    pub invoked_as: InvocationMode,
}

impl Config {
    /// Creates `Config` struct from an `Args` struct (which will be consumed).
    ///
    /// Returns `Ok(Config)`, if the path in args points to a file or a directory.
    ///
    /// Otherwise returns `Err(&'static str)`
    pub fn from(args: Args) -> Result<Config, &'static str> {
        let mode = match std::fs::metadata(&args.path) {
            Ok(md) if md.is_dir() => InvocationMode::Directory,
            Ok(md) if md.is_file() => InvocationMode::File,
            Ok(_) => return Err("Path is neither file nor directory? This should never happen."),
            Err(_) => return Err("Could not fetch metadata for supplied path. Does the path exist? Do you have the correct permissions?")
        };
        Ok(Config {
            args,
            invoked_as: mode,
        })
    }
}
