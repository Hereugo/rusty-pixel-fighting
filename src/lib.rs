pub struct Config {
    pub width: u32,
    pub height: u32,
    pub glyph: char,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the first argument, which is the name of the program
        args.next();

        let width = args
            .next()
            .unwrap_or("80".to_string())
            .parse::<u32>()
            .unwrap();
        let height = args
            .next()
            .unwrap_or("40".to_string())
            .parse::<u32>()
            .unwrap();
        let glyph = args
            .next()
            .unwrap_or("█".to_string())
            .chars()
            .next()
            .unwrap();

        Ok(Config {
            width,
            height,
            glyph,
        })
    }
}
