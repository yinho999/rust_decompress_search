use std::{fs::File, path::Path};

pub fn unzip_file(path: &Path) {
    let file = File::open(path).expect("The zip file should be able to open");

    let mut zip = zip::ZipArchive::new(file).expect("unable to read zip file");

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();

        // get the file path name
        let output_path = match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        // If file is commented
        if file.comment().is_empty() {
            println!("File {} comment: {}", file.name(), file.comment());
        }

        // If file is a directory
        if file.is_dir() {
            println!("Directory {} extract to \"{}\"", i, output_path.display());
            std::fs::create_dir_all(output_path).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_path.display(),
                file.size()
            );

            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut output_file = File::create(&path).unwrap();
            std::io::copy(&mut file, &mut output_file).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
