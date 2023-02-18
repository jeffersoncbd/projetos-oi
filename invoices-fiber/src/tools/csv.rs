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

pub struct Row<'a> {
    pub data_venc: &'a str,
    pub status_notificacao: &'a str,
    pub tipo_pro: &'a str,
    pub count: &'a str,
}

pub fn estruture(csv_content: &String) -> Vec<Row> {
    let mut csv = Vec::new();

    let rows: Vec<&str> = csv_content.split("\n").filter(|r| !r.is_empty()).collect();
    for row in &rows[1..] {
        let cells: Vec<&str> = row.split(";").map(|c| c.trim()).collect();
        csv.push(Row {
            data_venc: cells[0],
            status_notificacao: cells[1],
            tipo_pro: cells[2],
            count: cells[3],
        })
    }

    csv
}
