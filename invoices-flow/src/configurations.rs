use crate::tools::string_to_static;

pub struct Configurations {
    pub history_folder_path: String,
    pub telegram_token: String,
    pub destiny_id: String,
    pub executions_amount: u32,
}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        /* if cfg!(debug_assertions) {
            return Ok(Configurations {
                history_folder_path: String::from(""),
                telegram_token: String::from(""),
                destiny_id: String::from(""),
            });
        } */

        let history_folder_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o caminho da pasta com histórico de execuções (em $1)"),
        };

        let executions_amount: u32 = match args.next() {
            Some(amount) => match amount.parse() {
                Ok(amount) => amount,
                Err(error) => {
                    let feedback =
                        format!("Informe um número inteiro e positivo (em $2): {}", error);
                    return Err(string_to_static(feedback));
                }
            },
            None => return Err("Informe o caminho da pasta com histórico de execuções (em $1)"),
        };

        let telegram_token = match args.next() {
            Some(token) => token,
            None => return Err("Informe to token do telegram (em $2)"),
        };

        let destiny_id = match args.next() {
            Some(id) => id,
            None => return Err("Informe o ID da conversa que irá receber o relatório (em $3)"),
        };

        let configurations = Configurations {
            history_folder_path,
            telegram_token,
            destiny_id,
            executions_amount,
        };
        Ok(configurations)
    }
}
