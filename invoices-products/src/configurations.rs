pub struct Configurations {
    pub csv_path: String,
    pub filtering_period: Option<Vec<String>>,
    pub telegram_token: String,
    pub destiny_id: String,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: "result.csv".to_string(),
                filtering_period: None,
                telegram_token: String::from("5600687475:AAFtM9o3pcnECnDCMeM42OC0Rb1etknjk_Q"),
                destiny_id: String::from("5828637972"),
            });
        }

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("deve ser informado o caminho para o .csv (em $1)"),
        };

        let telegram_token = match args.next() {
            Some(token) => token,
            None => return Err("Informe to token do telegram (em $2)"),
        };

        let destiny_id = match args.next() {
            Some(id) => id,
            None => return Err("Informe o ID do chat destino (em $3)"),
        };

        let filtering_period = match args.next() {
            Some(values) => Some(values.split(",").map(|v| String::from(v)).collect()),
            None => None,
        };

        Ok(Configurations {
            csv_path,
            filtering_period,
            telegram_token,
            destiny_id,
        })
    }
}
