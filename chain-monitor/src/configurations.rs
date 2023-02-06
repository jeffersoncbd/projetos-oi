use crate::tools::string_to_static;

pub struct Configurations {
    pub csv_path: String,
    pub column: String,
    pub job_name: String,
    pub chain: Option<String>,
}

impl Configurations {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        let csv_path = match args.next() {
            Some(path) => path,
            None => return Err("Informe o .csv ($1)"),
        };

        let feedback = [
            "Informe o parâmetro de busca ($2):",
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

        let job_name = match args.next() {
            Some(job_name) => job_name,
            None => return Err("Informe o nome do job ($3)"),
        };

        let chain = args.next();

        Ok(Configurations {
            csv_path,
            column,
            job_name,
            chain,
        })
    }
}
