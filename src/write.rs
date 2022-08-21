use color_eyre::{eyre::Context, Result};
use edit::{edit, edit_file, Builder};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create wip file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    file.write_all(TEMPLATE)?;

    edit_file(filepath)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    dbg!(contents);
    todo!()
}
