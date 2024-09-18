use crate::{case_converter, templates};
use std::{fs, io};

pub fn create(name: &str) -> io::Result<()> {
    // Create the route file
    let file_name = format!("{}.route.jsx", case_converter::pascal_to_kebab(name));
    let folder_name = case_converter::pascal_to_kebab(name);
    let folder_path = format!("./src/routes/{}", folder_name);
    let file_path = format!("./src/routes/{}/{}", folder_name, file_name);
    let file_content = templates::ROUTE.replace("ROUTE_NAME", name);
    fs::create_dir(folder_path).expect("Sth went wrong while creating the route's folder");
    fs::write(file_path, file_content)?;

    Ok(())
}
