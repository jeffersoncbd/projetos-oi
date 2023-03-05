pub struct Configurations {
    pub csv_path: String,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: "result.csv".to_string(),
            });
        }

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("deve ser informado o caminho para o .csv (em $1)"),
        };

        Ok(Configurations { csv_path })
    }
}
