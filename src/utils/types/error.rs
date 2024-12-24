#[derive(Debug)]
pub enum Error
{
    NoReadMeFile(String),
    CouldNotParse,
}
