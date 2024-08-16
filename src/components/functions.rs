use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn write_xml_to_file (
    xml_string: impl AsRef<str>,
    file_path: impl AsRef<str>,
    overwrite: bool
) -> Result<(),String> {
    let xml_content = xml_string.as_ref();
    let path = Path::new(file_path.as_ref());

    if path.exists() && !overwrite {
        return Err(format!("File {} exists and overwrite is set to false.",path.display()));
    }

    match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
        Ok(mut file) => {
            file.write_all(xml_content.as_bytes()).map_err(|e| e.to_string())?;
            return Ok(());
        },
        Err(e) => return Err(e.to_string()),
    };
}