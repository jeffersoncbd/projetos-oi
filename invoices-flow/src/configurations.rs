pub struct Configurations {
    pub history_folder_path: String,
}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                history_folder_path: "history".to_string(),
            });
        }

        let history_folder_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o caminho da pasta com histórico de execuções (em $1)"),
        };

        let configurations = Configurations {
            history_folder_path,
        };
        Ok(configurations)
    }
}
