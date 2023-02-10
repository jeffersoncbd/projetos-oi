pub struct Configurations {}
impl Configurations {
    pub fn from(mut args: impl Iterator<Item = String>) -> Result<Configurations, &'static str> {
        args.next();

        let configurations = Configurations {};
        Ok(configurations)
    }
}
