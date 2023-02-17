use std::fs;

pub fn read(path: &String) -> Result<String, &'static str> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            let feedback = format!("Falha ao tentar ler o arquivo xml: {}", error);
            return Err(string_to_static::parse(feedback));
        }
    };

    Ok(content)
}
