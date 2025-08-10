use std::slice::Iter;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub save_frames: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let mut config = Config {
            width: 200,
            height: 100,
            scale: 4,
            save_frames: false
        };

        let mut args_iter = args.iter();
        args_iter.next(); // skip executable name
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--width" => {
                    config.width = Config::parse_next(&mut args_iter)?;
                }
                "--height" => {
                    config.height = Config::parse_next(&mut args_iter)?;
                }
                "--scale" => {
                    config.scale = Config::parse_next(&mut args_iter)?;
                }
                "--save-frames" => {
                    config.save_frames = true;
                }
                other => {
                    return Err(format!("unknown option {}", other));
                }
            }
        }

        Ok(config)
    }

    fn parse_next(args: &mut Iter<'_, String>) -> Result<u32, &'static str> {
        args.next()
            .ok_or_else(|| "Missing value for flag")?
            .parse::<u32>()
            .map_err(|_| "Could not parse value")
    }
}