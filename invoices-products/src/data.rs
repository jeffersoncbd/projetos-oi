#[derive(Debug)]
pub struct Structure {
    pub venciment: String,
    pub status: String,
    pub tipo: String,
    pub produto: String,
    pub quantidade: String,
}
impl Structure {
    pub fn from(content: String, filtering_period: &Option<Vec<String>>) -> Vec<Structure> {
        let mut data = Vec::new();
        let rows: Vec<&str> = content.split("\n").collect();
        for row in rows[1..].iter() {
            if row == &"" {
                continue;
            };
            let cells: Vec<&str> = row.split(";").map(|cell| cell.trim()).collect();

            // filter
            if let Some(filtering_period) = filtering_period {
                let part_of_date: Vec<String> =
                    cells[1].split("/").map(|p| String::from(p)).collect();
                if !filtering_period.contains(&part_of_date[0]) {
                    continue;
                };
            }

            data.push(Structure {
                venciment: String::from(cells[1]),
                status: String::from(cells[2]),
                tipo: String::from(cells[3]),
                produto: String::from(cells[4]),
                quantidade: String::from(cells[5]),
            });
        }
        data
    }
}
