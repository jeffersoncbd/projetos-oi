use std::fs;

pub fn read(path: &String) -> Result<String, &'static str> {
    if !path.ends_with(".csv") {
        return Err("é necessário informar um arquivo .csv");
    }
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(error) => {
            let feedback = format!("falha ao tentar ler o arquivo \"{}\" ({})", path, error);
            return Err(string_to_static::parse(feedback));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::read;
    use chrono::Utc;
    use pretty_assertions::assert_eq;

    #[test]
    fn deve_retornar_erro_caso_o_caminho_informado_nao_seja_de_um_arquivo_csv() {
        let result = read(&"/any/file".to_string());
        assert_eq!(Err("é necessário informar um arquivo .csv"), result);
    }

    #[test]
    fn deve_retornar_erro_caso_o_arquivo_csv_nao_exista() {
        let result = read(&"/any/file.csv".to_string());
        assert_eq!(Err("falha ao tentar ler o arquivo \"/any/file.csv\" (No such file or directory (os error 2))"), result);
    }

    #[test]
    fn deve_retornar_com_sucesso_o_conteudo_do_arquivo_csv_valido() {
        let now = Utc::now().to_string();
        let csv_name = String::from("/tmp/now.csv");
        fs::write(&csv_name, &now).unwrap();
        let result = read(&csv_name);
        assert_eq!(Ok(now), result);
    }
}
