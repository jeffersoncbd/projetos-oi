use std::fs;

pub struct Command {
    pub name: String,
    pub script: String,
    pub allowed_chats: Vec<i64>,
}

pub struct Configurations {
    pub commands: Vec<Command>,
}

impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        if cfg!(debug_assertions) {
            let commands = vec![Command {
                name: String::from("/test"),
                script: String::from(
                    "/home/jefferson/projects/oi/shells/relatorio-das-cadeias.sh dev",
                ),
                allowed_chats: vec![5828637972],
            }];
            return Ok(Configurations { commands });
        }

        let mut commands: Vec<Command> = vec![];

        for arg in args {
            let parts: Vec<&str> = arg.split("|").collect();
            if parts.len() != 3 {
                let feedback = [
                    "a estrutura dos comandos devem ser separadas por \"|\":",
                    "   comando|ID,ID,ID|/path/to/script.sh",
                ];
                return Err(string_to_static::parse(feedback.join("\n")));
            };
            if parts[0] == "" {
                return Err("o comando deve ter um nome");
            }
            let mut allowed_chats: Vec<i64> = vec![];
            for id in parts[1].split(",") {
                let parsed: i64 = match id.parse() {
                    Ok(id) => id,
                    Err(_) => {
                        let feedback = [
                            "a lista de ID's devem ser de números inteiros separados por vírgula:",
                            "   comando|ID,ID,ID|/path/to/script.sh",
                        ];
                        return Err(string_to_static::parse(feedback.join("\n")));
                    }
                };
                allowed_chats.push(parsed);
            }
            if let Err(_) = fs::metadata(parts[2]) {
                let feedback = format!("o script \"{}\" não foi encontrado", parts[2]);
                return Err(string_to_static::parse(feedback));
            };
            commands.push(Command {
                name: String::from(parts[0]),
                script: String::from(parts[2]),
                allowed_chats,
            });
        }

        if commands.len() == 0 {
            let feedback = [
                "não foi informado nenhum comando, use o formato:",
                "   comando|ID,ID,ID|/path/to/script.sh",
            ];
            return Err(string_to_static::parse(feedback.join("\n")));
        }

        Ok(Configurations { commands })
    }
}
