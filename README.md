# rzp
**Note: This was made as a high school final project and as a result, probably isn't very good**
### A command line archiver written in Rust
Note: Due to the dependencies this project uses, It doesn't support Windows. 
Features:
- Blazing fast (over 3x as fast a p7zip and unzip)
- Support for operating on multiple files at once
- Helpful and minimal output
- Support for the following compression formats: stored (i.e. none), deflate, bzip2, zstd (Thanks to [zip-rs](https://github.com/zip-rs/zip))
- List file attributes
- Simple syntax
- Deletion support
---
## Usage
<div>

Arguments:
  [files]...  File(s) input path

Options:
  
  -l, --list             
  List files in archive

  -x, --extract          
  Extract files

  -o, --output <output>  
  Output location

  -f, --force            
  Override existing files

  -v, --verbose          
  Be verbose

  --remove           
  Delete zip file after extraction

  -d, --directory        
  Extract each zip file into its own directory

  -H, --human-readable    
  Output bytes in human readable format

  -q, --quiet   
  Suppress output

  -h, --help             
  Print help

  -V, --version          
  Print version
</div>
