use tg_api::{Client, Configurations};

pub fn new_client(token: String) -> Client {
    let configurations = Configurations::new(token, None);
    Client::new(configurations)
}
