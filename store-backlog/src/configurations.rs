pub struct Configurations {
    pub csv_path: String,
    pub destiny_path: String,
}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: "model.csv".to_string(),
                destiny_path: "history".to_string(),
            });
        }

        args.next();

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("Deve ser informado o caminho para o csv original (em $1)"),
        };

        let destiny_path =
            match args.next() {
                Some(path) => path,
                None => return Err(
                    "Deve ser informado o caminho da pasta onde os arquivos serão salvos (em $2)",
                ),
            };

        Ok(Configurations {
            csv_path,
            destiny_path,
        })
    }
}
