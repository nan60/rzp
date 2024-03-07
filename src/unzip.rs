use std::{
    fs::{self, File},
    io::{self, prelude::*, BufWriter},
    path::PathBuf,
    process,
};
use owo_colors::OwoColorize;
use zip::read::ZipFile;
use super::util;

pub fn list_zip_contents(reader: impl Read + Seek, file_path: &String, print_index: i32, human: bool) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipArchive::new(reader)?; // Open our zip file
    let input_md = fs::metadata(file_path)?; // Get its metadata
    if print_index >= 1{
        println!(); // Prints a new line; Produces cleaner output. We don't want to print new line for the first file
    }
    let mut file_count = 0;
    let mut dir_count = 0;
    if zip.is_empty() {
        println!("{}", "File is empty".red().bold());
        return Ok(());
    } 
    else {
        for i in 0..zip.len() { // Loop for n files in archive
            let file = zip.by_index(i)?; // Open each file

            if file.is_dir(){
                print_props(&file, true, human);
                dir_count += 1;
            }
            else if file.is_file(){ 
                print_props(&file, false, human);
                file_count += 1;
            }
            else{
                println!("{}", "Found non dir/file!".red().bold());
                process::exit(11)
            }
        }
    }
    println!(
        "{:<20} {:<12} {:<12} {:<12} {:<12} {}",
        "Date".on_black(),
        "Size".on_black(),
        "Compressed".on_black(),
        "Ratio".on_black(),
        "CRC".on_black(),
        "Name         ".on_black()
    );
    println!("-----------------------------------------------------------------------");
    println!(
        "{} ({}) {} File(s), {} Folder(s)",
        file_path,
        if human { // Checks if -H was specified
            util::convert_bytes(input_md.len() as usize)
        }
        else {
            input_md.len().to_string() + " B"
        },
        file_count,
        dir_count,
    );

    Ok(())
}

// Print file properties
fn print_props(file: &ZipFile, is_dir: bool, human: bool){
        let time = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            file.last_modified().year(),
            file.last_modified().month(),
            file.last_modified().day(),
            file.last_modified().hour(),
            file.last_modified().minute(),
            file.last_modified().second()
        );
        if is_dir{ // Treat output differently if its a directory
            println!(
                "{:<20} {:<51} {}",
                time,
                "", // Use empty string since we don't want output here
                file.name().on_cyan()
            );
        }
        else{
            let compress_ratio = util::percent_change(file.size() as usize, file.compressed_size() as usize).unwrap_or(0.0);
            println!(
                "{:<20} {:<12} {:<12} {:<12.3} {:<12} {}",
                time,
                if human{
                    util::convert_bytes(file.size() as usize)
                }
                else{
                    file.size().to_string()
                },
                if human{
                    util::convert_bytes(file.compressed_size() as usize)
                }
                else{
                    file.compressed_size().to_string()
                }, // This needs to be refactored!
                compress_ratio,
                file.crc32(),
                file.name()
            );
        }
}

pub fn extract_zip(reader: impl Read + Seek, 
    output_dir: &String, force: bool, 
    verbose: bool, 
    file_path: &String, 
    create_dir: bool) -> zip::result::ZipResult<()> {

    let mut zip = zip::ZipArchive::new(reader)?;
    let zip_length = &mut zip.len(); 
    for i in 0..*zip_length { // Loop for n files in zip
        let mut file = zip.by_index(i)?;
        let mut outpath = PathBuf::new(); // Create empty Path Buffer
        if create_dir{ // Check '-d' flag
            let file_path_noext = file_path.replace(".zip", "");
            outpath.push(file_path_noext); // Add to the end of the PathBuf
        }
        // tl;dr initpath = path
        let initpath = match file.enclosed_name(){
            Some(path) => path.to_owned(),
            None => continue,
        };
        // If output dir doesn't exist, create it
        if !output_dir.is_empty(){
            outpath.push(output_dir);
        }
        // Always push initpath, but do it last
        outpath.push(initpath);

        // If file is a dir, create it
        if (file.name()).ends_with('/'){
            fs::create_dir_all(&outpath).unwrap();
        }
        else{
            // Create directory structure for a file if it doesn't exist
            if let Some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(p).unwrap();
                }
            }
            // Skip files that already exist unless using '-f'
            if !force && outpath.exists(){
                println!("{}{}", outpath.to_string_lossy().yellow().bold(), " already exists; Skipping".yellow().bold());
                continue;
            }
            else{
                let mut outfile = BufWriter::new(File::create(&outpath)?); // Create new buffer
                io::copy(&mut file, &mut outfile)?; // Copy file to its buffer (and actually write it)
                if verbose { // verbose output with '-v'
                    println!("{} -> {}", file.name().bright_blue().bold(), outpath.to_string_lossy().bright_blue().bold());
                }
            }
        }
    }
    if verbose{
        print!("\r{} ({})", "Everything Ok".cyan().bold(), file_path);
    }
    Ok(())
}
