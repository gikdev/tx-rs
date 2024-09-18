use crate::{case_converter, templates};
use std::{fs, io};

pub fn create(name: &str) -> io::Result<()> {
    // Create the component file
    let file_name = format!("{}.component.jsx", case_converter::pascal_to_kebab(name));
    let file_path = format!("./src/components/{}", file_name);
    let file_content = templates::COMPONENT.replace("COMPONENT_NAME", name);
    fs::write(file_path, file_content)?;

    // Add it to the index file!
    let export_statement = format!(
        "export * from \"./{}.component\"\n",
        case_converter::pascal_to_kebab(name)
    );
    let mut existing_index_content = fs::read_to_string("./src/components/index.js")?;
    existing_index_content.push_str(&export_statement);
    fs::write("./src/components/index.js", existing_index_content)
        .expect("Failed to write to the components' index file");

    Ok(())
}
