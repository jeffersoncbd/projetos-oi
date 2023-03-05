#[derive(Debug)]
pub struct Structure {
    pub venciment: String,
    pub status: String,
    pub tipo: String,
    pub produto: String,
    pub quantidade: String,
}
impl Structure {
    pub fn from(content: String) -> Vec<Structure> {
        let mut data = Vec::new();
        let rows: Vec<&str> = content.split("\n").collect();
        for row in rows[1..].iter() {
            if row == &"" {
                continue;
            }
            let cells: Vec<&str> = row.split(";").map(|cell| cell.trim()).collect();
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
