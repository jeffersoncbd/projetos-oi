use std::fs;

use roxmltree::Node;

use super::string_to_static;

pub fn find_job_name(job_name: &str, xml_path: &String) -> Result<String, &'static str> {
    let xml_content = match fs::read_to_string(xml_path) {
        Ok(content) => content,
        Err(error) => {
            return Err(string_to_static(format!(
                "o arquivo RAID_COLLECTIONS.xml não foi encontrado.\n{}",
                error
            )))
        }
    };

    match roxmltree::Document::parse(&xml_content) {
        Ok(doc) => {
            let deftable = doc
                .descendants()
                .find(|n| n.has_tag_name("DEFTABLE"))
                .unwrap();
            let folders: Vec<Node> = deftable
                .descendants()
                .filter(|n| n.has_tag_name("FOLDER"))
                .collect();
            for folder in folders {
                let job = folder
                    .descendants()
                    .find(|n| n.attribute("JOBNAME") == Some(job_name));
                if let None = job {
                    continue;
                }

                let variable = job
                    .unwrap()
                    .descendants()
                    .find(|n| n.has_tag_name("VARIABLE") && n.attribute("NAME") == Some("%%PARM1"))
                    .unwrap();
                let database_name = variable.attribute("VALUE").unwrap();
                return Ok(String::from(database_name));
            }
        }
        Err(_) => (),
    };

    let feedback = format!(
        "O job \"{}\" não foi encontrado na coleção do RAID (.xml)",
        job_name
    );
    Err(string_to_static(feedback))
}
