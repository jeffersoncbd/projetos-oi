pub struct Configurations {
    pub csv_path: String,
    pub telegram_token: String,
    pub destiny_id: String,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: String::from("model.csv"),
                telegram_token: String::from("5600687475:AAFtM9o3pcnECnDCMeM42OC0Rb1etknjk_Q"),
                destiny_id: String::from("5828637972"),
            });
        }

        let csv_path = match args.next() {
            Some(path) => {
                if &path[path.len() - 4..] != ".csv" {
                    return Err("Deve ser informado um arquivo .csv com os dados (em $1)");
                }
                path
            }
            None => return Err("Deve ser informado um arquivo .csv com os dados (em $1)"),
        };

        let telegram_token = match args.next() {
            Some(token) => token,
            None => return Err("Informe to token do telegram (em $2)"),
        };

        let destiny_id = match args.next() {
            Some(id) => id,
            None => return Err("Informe o ID da conversa que irá receber o relatório (em $3)"),
        };

        Ok(Configurations {
            csv_path,
            telegram_token,
            destiny_id,
        })
    }
}
