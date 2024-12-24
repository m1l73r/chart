use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

struct File
{
    pub name: String,
}

struct Folder
{
    pub path: PathBuf,
    pub folders: Vec<Folder>,
    pub files: Vec<File>,
}
mod change
{
    pub fn change_chars_at(indent_str: String, indexes: &Vec<usize>, skip_last: bool) -> String
    {
        let mut vec_of_chars: Vec<char> = indent_str.chars().collect();
        for (index, &val) in indexes.iter().enumerate()
        {
            if skip_last && index + 1 == indexes.len()
            {
                break;
            }
            vec_of_chars[val] = '│';
        }
        vec_of_chars.into_iter().collect()
    }
}
macro_rules! get_if {
    ($bool:expr, $v1:expr, $v2:expr) => {
        if $bool
        {
            $v1
        }
        else
        {
            $v2
        }
    };
}
#[macro_export]
macro_rules! ignore {
    ( $( $value:expr ),* $(,)? ) => {{
        let mut set = std::collections::HashSet::new();
        $( set.insert($value); )*
        set
    }};
}
pub struct Connectors<'a>
{
    pub empty_folder: &'a str,
    pub vertical_line: &'a str,
    pub right: &'a str,
    pub left: &'a str,
    pub more_items: &'a str,
}
impl<'a> Default for Connectors<'a>
{
    fn default() -> Self
    {
        Self {
            empty_folder: "──",
            vertical_line: "│",
            right: "─┐",
            left: "└─",
            more_items: "├─",
        }
    }
}
impl Folder
{
    fn generate_tree_recursive(
        &self,
        connectors: &Connectors,
        ignore: &Option<HashSet<&str>>,
        mut indent: usize,
        more_items: bool,
        extra_lines: &Vec<usize>,
    ) -> String
    {
        //Here we clone the parameters extra_line, because we arent interested in change the original extra_lines, because of the way the tree works, but we do want to mutate it
        let mut extra_lines = extra_lines.clone();

        //We want to check if the current_folder is root, so that we can set the root folder to empty_folder when it opens
        let is_first = indent == 0;

        //We want to get the connector length, so that we dynamically can change the length of each folder or file
        let connector_len = connectors.empty_folder.chars().count();

        //We want to store the folders length because it is used to dynamically resize the tree
        let folders_len = self.folders.len();

        //We want to check if it contains folders, because it depends on how we are going to handle the rest of the tree
        let contains_folders = !self.folders.is_empty();

        //We want to store the files length because it is used to dynamically resize the tree
        let files_len = self.files.len();

        //We want to check both
        let contains_items = contains_folders || !self.files.is_empty();

        //We use the folder name a lot which is why we store it in a variable
        let folder_name = self.path.to_str().unwrap();

        //We use the folder length a lot which is why we store it in a variable
        let folder_len = folder_name.len();

        //The indent_str is what is making the indentation of the tree. This is the primary variable used to store the indentation
        //Here we change specific charactors in the string, so we get the actual tree branches (old and new ones), we skip the last, because the last is the folder itself
        let mut indent_str = change::change_chars_at(" ".repeat(indent), &extra_lines, true);

        //The actual tree
        let mut tree = String::new();

        //Connectors is whats binding the tree items together. The connectors can be specified dynamically
        let connector = get_if!(contains_items, connectors.right, connectors.empty_folder);
        let connector2 = get_if!(
            more_items,
            get_if!(is_first, connectors.empty_folder, connectors.more_items),
            connectors.left
        );

        //Here we push in the folder
        tree.push_str(&format!(
            "{}{} {} {}\n",
            indent_str, connector2, folder_name, connector
        ));

        //Since the files indentation is bigger than the folder above, we want to increase the indentation by connector + folder + connect (for example: "├─ Apps ─┐")
        indent += connector_len + folder_len + connector_len;

        //If there is no more items after this one (meaning its the last folder), we want to pop the extra line of the extra_lines vector
        if !more_items
        {
            extra_lines.pop();
        }

        //Here we change specific charactors in the string, so we get the actual tree branches (old and new ones)
        indent_str = change::change_chars_at(" ".repeat(indent + 1), &extra_lines, false);

        //If the current folder contains folders, we want to push the extra line in
        if !self.folders.is_empty()
        {
            extra_lines.push(indent + 1);
        }

        //We enumerate through it here, because we want to check if its the last one
        for (i, file) in self.files.iter().enumerate()
        {
            //We check if its the last one
            let is_last = i + 1 == files_len;
            //We create the connector and push it in
            let connector = get_if!(
                is_last && !contains_folders,
                connectors.left,
                connectors.more_items
            );
            tree.push_str(&format!("{}{} {}\n", indent_str, connector, file.name));
        }

        //We enumerate through it here, because we want to check if theres is any more items
        let is_ignore = |ignores: &Option<HashSet<&str>>, folder: &Folder| {
            //If we want to ignore some folders, this is where it continues if we find any folders that matches our ignore-folders
            ignores
                .as_ref()
                .map_or(false, |set| set.contains(folder.path.to_str().unwrap()))
        };
        for (i, folder) in self.folders.iter().enumerate()
        {
            if is_ignore(ignore, folder)
            {
                continue;
            }

            //Check if theres any more times after this
            let more_items = i + 1 != folders_len
                && (ignore.is_none()
                    || self.folders[i + 1..]
                        .iter()
                        .any(|folder| !is_ignore(ignore, folder)));

            //Do recursion so we get everything in that nice tree structure
            tree.push_str(&folder.generate_tree_recursive(
                connectors,
                ignore,
                indent + 1,
                more_items,
                &extra_lines,
            ));
        }

        //Return tree
        tree
    }

    fn generate_tree_in_file(
        &self,
        connectors: &Connectors,
        ignore: &Option<HashSet<&str>>,
        output: &Path,
    )
    {
        let p = output.to_str().unwrap();
        let tree = self.generate_tree_recursive(&connectors, ignore, 0, true, &Vec::new());
        let mut file = fs::File::create(format!("{p}\\tree.txt")).unwrap();
        _ = file.write_all(tree.as_bytes());
    }
    fn generate_tree_as_string(
        &self,
        connectors: &Connectors,
        ignore: &Option<HashSet<&str>>,
    ) -> String
    {
        let tree = self.generate_tree_recursive(&connectors, ignore, 0, true, &Vec::new());
        tree
    }
    fn new(path_buf: PathBuf) -> Self
    {
        Folder {
            path: path_buf,
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
    fn clean(&mut self)
    {
        self.path =
            Path::new(self.path.to_str().unwrap().rsplit_once('\\').unwrap().1).to_path_buf();
        for folder in &mut self.folders
        {
            folder.clean();
        }
    }
}

fn find_files_in_dir(dir: &Path) -> io::Result<Folder>
{
    let mut root_folder = Folder::new(dir.to_path_buf());
    find_files_in_dir_recursive(dir, &mut root_folder)?;
    root_folder.clean();
    Ok(root_folder)
}

fn find_files_in_dir_recursive(current_dir: &Path, current_folder: &mut Folder) -> io::Result<()>
{
    for entry in fs::read_dir(current_dir)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
        {
            let mut subfolder = Folder::new(path.clone());
            find_files_in_dir_recursive(&path, &mut subfolder)?;
            current_folder.folders.push(subfolder);
        }
        else if path.is_file()
        {
            let file = File {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
            };
            current_folder.files.push(file);
        }
    }
    Ok(())
}

pub fn create_dir_tree_file(
    connectors: &Connectors,
    ignore: &Option<HashSet<&str>>,
    input_path: &Path,
    output_path: &Path,
)
{
    let root_folder = find_files_in_dir(input_path).expect("Can't get files");
    root_folder.generate_tree_in_file(connectors, ignore, output_path);
}
pub fn get_dir_tree_string(
    connectors: &Connectors,
    ignore: &Option<HashSet<&str>>,
    input_path: &Path,
) -> String
{
    let root_folder = find_files_in_dir(input_path).expect("Can't get files");
    let tree = root_folder.generate_tree_as_string(connectors, ignore);
    tree
}
