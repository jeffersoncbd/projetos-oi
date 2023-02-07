use crate::tools::{names_relationship, string_to_static};

pub struct Configurations {
    pub csv_path: String,
    pub xml_path: String,
    pub column: String,
    pub job_name_in_control: String,
    pub job_name_in_database: String,
    pub chain: Option<String>,
}

impl Configurations {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

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
            "  -i      Inicio do job",
            "  -f      Fim do job",
        ]
        .join("\n");

        let column = match args.next() {
            Some(flag) => match flag.as_str() {
                "-i" => String::from("INICIO"),
                "-f" => String::from("FIM"),
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

        let chain = args.next();

        Ok(Configurations {
            csv_path,
            xml_path,
            column,
            job_name_in_control,
            job_name_in_database,
            chain,
        })
    }
}
