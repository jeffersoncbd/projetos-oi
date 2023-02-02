pub struct Configurations {
    pub csv_path: String,
    pub filter: String,
    pub output_path: String,
    pub telegram_token: String,
}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: String::from(""),
                filter: String::from(""),
                output_path: String::from(""),
                telegram_token: String::from(""),
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
            None => {
                return Err("Defina a pasta onde será gerado os arquivos de output (parâmetro 3)")
            }
        };

        let telegram_token = match args.next() {
            Some(token) => token,
            None => return Err("Informe to token do telegram (parâmetro 4)"),
        };

        Ok(Configurations {
            csv_path,
            filter,
            output_path,
            telegram_token,
        })
    }
}
