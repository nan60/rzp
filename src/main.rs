use clap::{command, Arg, ArgAction};
use std::fs::File;

mod util;
mod unzip;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup CLI parsing with clap
    let args = command!()
        .name("rzp")
        .about("rzp: a command line archiver written in Rust")
        .arg(Arg::new("files").index(1).required(true).help("File(s) input path").num_args(0..))
        .arg(Arg::new("list").short('l').long("list").action(ArgAction::SetTrue).help("List files in archive"))
        .arg(Arg::new("extract").short('x').long("extract").action(ArgAction::SetTrue).help("Extract files"))
        .arg(Arg::new("output").short('o').long("output").default_value("").hide_default_value(true).help("Output location"))
        .arg(Arg::new("force").short('f').long("force").action(ArgAction::SetTrue).help("Override existing files"))
        .arg(Arg::new("verbose").short('v').long("verbose").action(ArgAction::SetTrue).help("Be verbose"))
        .arg(Arg::new("directory").short('d').long("directory").action(ArgAction::SetTrue).help("Extract each zip file into its own directory"))
        .arg(Arg::new("human_readable").short('H').long("human-readable").action(ArgAction::SetTrue).help("Output bytes in human readable format"))
        .get_matches();

    // Open files in a string of vecs (array size must be known at compile time in Rust, so we use vectors instead)
    let file: Vec<_> = args.get_many::<String>("files").unwrap().collect(); 
    let output: String = args.get_one::<String>("output").unwrap().to_string();
    let mut print_index = 0;
    for i in 0..file.len(){ // Loop for n files in 'file' vec
        if !util::check_path(file[i]){
            continue; // If we return false, there was an error and it was printed; Next file
        }
        let zip_file = File::open(file[i]).expect("Unable to open file"); // Open our .zip file
        if args.get_flag("list") {
            unzip::list_zip_contents(&zip_file, file[i], print_index, args.get_flag("human_readable"))?;
            print_index += 1;
        }
        if args.get_flag("extract"){
            unzip::extract_zip(
            &zip_file, 
            &output, 
            args.get_flag("force"), 
            args.get_flag("verbose"), 
            file[i], 
            args.get_flag("directory"))?;
        }
    }
    Ok(())
}
