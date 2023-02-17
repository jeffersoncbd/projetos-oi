pub struct Configurations {
    pub csv_path: String,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        /* if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: String::from(""),
            });
        } */

        let csv_path = match args.next() {
            Some(path) => {
                if &path[path.len() - 4..] != ".csv" {
                    return Err("Deve ser informado um arquivo .csv com os dados (em $1)");
                }
                path
            }
            None => return Err("Deve ser informado um arquivo .csv com os dados (em $1)"),
        };

        Ok(Configurations { csv_path })
    }
}
