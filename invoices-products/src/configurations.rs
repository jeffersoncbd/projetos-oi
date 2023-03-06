pub struct Configurations {
    pub csv_path: String,
    pub filtering_period: Option<Vec<String>>,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: "result.csv".to_string(),
                filtering_period: Some(vec![String::from("16")]),
            });
        }

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("deve ser informado o caminho para o .csv (em $1)"),
        };

        let filtering_period = match args.next() {
            Some(values) => Some(values.split(",").map(|v| String::from(v)).collect()),
            None => None,
        };

        Ok(Configurations {
            csv_path,
            filtering_period,
        })
    }
}
