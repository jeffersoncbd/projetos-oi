pub struct Configurations {
    pub csv_path: String,
    pub filter: String,
    pub output_path: String,
}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: String::from("model.csv"),
                filter: String::from("whatsapp"),
                output_path: String::from("/home/jefferson/projects/oi/ged-parser/"),
            });
        }

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o caminho do arquivo .csv (parâmetro 1)"),
        };

        let filter = match args.next() {
            Some(filter) => match filter.as_str() {
                "e-mail" => filter,
                "whatsapp" => filter,
                _ => return Err("Os filtros aceitos são:\n- whatsapp\n- e-mail"),
            },
            None => return Err("Defina o filtro (parâmetro 2)"),
        };

        let output_path = match args.next() {
            Some(path) => path,
            None => return Err("Defina a pasta onde será gerado os arquivos de output"),
        };

        Ok(Configurations {
            csv_path,
            filter,
            output_path,
        })
    }
}
