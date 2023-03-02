use crate::tools::{names_relationship, str_date_to_utc, string_to_static};

pub struct Configurations {
    pub csv_path: String,
    pub xml_path: String,
    pub column: String,
    pub job_name_in_control: String,
    pub job_name_in_database: String,
    pub execution_date: Option<String>,
    pub execution_number: Option<i32>,
}

impl Configurations {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            return Ok(Configurations {
                csv_path: String::from("model.csv"),
                xml_path: String::from("RAID_COLLECTIONS.xml"),
                column: String::from("STATUS"),
                job_name_in_control: String::from("RAIDC_AutoDistribCampaignGenerator"),
                job_name_in_database: String::from("CAMPANHA"),
                execution_date: Some(String::from("30/01/2023")),
                execution_number: None,
            });
        }

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o caminho para .csv ($1)"),
        };

        let xml_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o caminho para o .xml ($2)"),
        };

        let feedback = [
            "Informe o parâmetro de busca ($3):",
            "  -a      Todas as execuções do job",
            "  -i      Inicio do job",
            "  -f      Fim do job",
            "  -s      Status do job",
        ]
        .join("\n");

        let column = match args.next() {
            Some(flag) => match flag.as_str() {
                "-a" => String::from("ALL"),
                "-i" => String::from("INICIO"),
                "-f" => String::from("FIM"),
                "-s" => String::from("STATUS"),
                _ => return Err(string_to_static(feedback)),
            },
            None => return Err(string_to_static(feedback)),
        };

        let job_name_in_control = match args.next() {
            Some(job_name) => job_name,
            None => return Err("Informe o nome do job ($4)"),
        };

        let job_name_in_database =
            names_relationship::find_job_name(&job_name_in_control, &xml_path)?;

        let mut execution_date = None;
        let mut execution_number = None;

        for argument in args {
            if argument.contains("--dia=") {
                let key_and_value: Vec<&str> = argument.split("=").collect();
                if let Err(error) = str_date_to_utc(&format!("{} 12:00:00", key_and_value[1])) {
                    let feedback = format!(
                        "{} A data \"{}\" não respeitou o formato DD/MM/AAAA",
                        error, key_and_value[1]
                    );
                    return Err(string_to_static(feedback));
                }
                execution_date = Some(String::from(key_and_value[1]));
            } else if argument.contains("--execucao=") {
                let key_and_value: Vec<&str> = argument.split("=").collect();
                let number: i32 = match key_and_value[1].parse() {
                    Ok(number) => number,
                    Err(error) => {
                        let feedback = format!("O número da execução deve ser um inteiro válido, valor informado: \"{}\". {}", key_and_value[1], error);
                        return Err(string_to_static(feedback));
                    }
                };
                if number == 0 {
                    return Err("O número da execução não pode ser 0");
                }
                execution_number = Some(number);
            }
        }

        Ok(Configurations {
            csv_path,
            xml_path,
            column,
            job_name_in_control,
            job_name_in_database,
            execution_date,
            execution_number,
        })
    }
}
