mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test() {}

    #[test]
    fn generate_dir_tree()
    {
        //file_tree
        let current_dir = std::env::current_dir().unwrap();
        let mut output_dir = current_dir.clone();
        output_dir.push("docs");
        
        file_tree::create_dir_tree_file(
            &file_tree::Connectors::default(),
            &Some(ignore!["target", "test", ".git"]),
            &current_dir,
            &output_dir,
        );
    }
}
