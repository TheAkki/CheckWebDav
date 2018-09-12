extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;
use std::path::Path;
use std::fs;


struct ProgramOptions{
    verbose     : bool,
    debug       : bool,
    source_path : String,
    target_path : String,
}

impl Default for ProgramOptions {
    fn default() -> ProgramOptions {
        ProgramOptions {
            verbose     : false,
            debug       : false,
            source_path : String::new(),
            target_path : String::new(),
        }
    }
}

#[derive(Debug)]
struct ProgressStatistik
{
    last_file_name  : String,
    last_file_size  : u64,
    total_size      : u64,
    total_files     : u64,
    finished        : bool
}

impl Default for ProgressStatistik {
    fn default() -> ProgressStatistik {
        ProgressStatistik {
            last_file_name  : String::new(),
            last_file_size  : 0,
            total_size      : 0,
            total_files     : 0,
            finished        : false
        }
    }
}

fn copy_file(options: &ProgramOptions, source: &Path, target: &Path, actual : &Path) -> u64
{
    let source_object = actual.clone();
    let relative_object = actual.strip_prefix(source).expect("First part of path not cutable");
    let filename = actual.file_name().unwrap().to_os_string().into_string().unwrap();    
    let target_object = target.join(relative_object);    
    //let target_path = target.join(relative_object);
    let target_object1 = target_object.clone();
    let target_path = target_object1.parent().expect("Can't get parent");

    
    if options.debug { println!("Source: {}", source.to_str().unwrap().to_string()); }
    if options.debug { println!("Target: {}", target.to_str().unwrap().to_string()); }
    if options.debug { println!("Object: {}", actual.to_str().unwrap().to_string()); }
    if options.debug { println!("Relative: {}", relative_object.to_str().unwrap().to_string()); }
    if options.debug { println!("TObject: {}", target_object.to_str().unwrap().to_string()); }
    if options.debug { println!("TPath: {}", target_path.to_str().unwrap().to_string()); }
    if options.debug { println!(""); }
    
    fs::create_dir_all(target_path.clone()).expect("Can not create parent folders");
    
    fs::copy(source_object, target_object).expect("Coping file not success")
}


fn copy(options: &ProgramOptions, mut progress: &mut ProgressStatistik, source: &Path, target: &Path, actual : &Path) -> bool
{
    let metadata = actual.metadata().expect("Read metadata failed");
    progress.last_file_size = metadata.len();
    progress.last_file_name = actual.to_str().unwrap().to_string();
    
    // copy here
    let size_copy = copy_file(&options, &source, &target, &actual);
    if size_copy != progress.last_file_size
    {
        eprintln!("File '{}' not correctly copied. Expected size: {}, Copied size: {}", progress.last_file_name, progress.last_file_size, size_copy);
        return false;    
    }
    
    
    progress.total_size = progress.total_size + progress.last_file_size;
    progress.total_files = progress.total_files + 1;
    
    if options.debug { println!("{:?}", progress); }
    
    true
}


fn test_get_dav_entries(options: &ProgramOptions) -> bool
{
    true
}


fn run(options: &ProgramOptions, mut progress: &mut ProgressStatistik, source: &Path, target: &Path, actual : &Path) -> bool
{
    //println!("Path: {}", actual.display() );

    let paths = fs::read_dir(actual).expect("Path not readable");

    //println!("{:?}", paths);
    
    for path in paths
    {
        let path = path.unwrap().path();
    
        if options.debug { print!("{} ", path.display() ); }
        
        if path.is_dir()
        {
            if options.debug { println!(" DIR"); }
            
            run(&options, &mut progress, &source, &target, &path);
        }
        else if path.is_file()
        {
            if options.debug { println!(" FILE"); }
            
            let result_copy = copy(&options, &mut progress, &source, &target, &path);
            if result_copy == false
            {
                return false;
            }
            
            let result_test = test_get_dav_entries(&options);
            if result_test == false 
            {
                return false;
            }
        }
    }
    
    true
}


fn check_and_run(options : &ProgramOptions) -> ProgressStatistik
{
    let mut result = ProgressStatistik{ ..Default::default() };
    let source_path = Path::new(&options.source_path);
    let target_path = Path::new(&options.target_path);
    
    let source = source_path.canonicalize().expect("Source path not valid");
    let target = target_path.canonicalize().expect("Target path not valid");
    
    let mut result_run : bool;
    {
        result_run = run(&options, &mut result, &source, &target, &source);
    }
    result.finished = result_run;
    
    result
}



fn main()
{
    let mut options = ProgramOptions{ ..Default::default() };
    
    {   // Scope
        let mut ap = ArgumentParser::new();
        ap.set_description("Copy one by one file from one directory to an other. After each file test something and create statitic of coping.");
        ap.refer(&mut options.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut options.debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Print also debug information");
        ap.refer(&mut options.source_path)
            .add_option(&["--source"], Store, "Source path");
        ap.refer(&mut options.target_path)
            .add_option(&["--target"], Store, "Target path");
        ap.parse_args_or_exit();        
    }
    
    if options.source_path.is_empty()
    {
        eprintln!("Source is empty");
        process::exit(1);
    }
    
    if options.target_path.is_empty()
    {
        eprintln!("Target is empty");
        process::exit(2);    
    }
    
    
    if options.verbose
    {
        println!("Source: {}", options.source_path);
        println!("Target: {}", options.target_path);
    }
    
    let res = check_and_run(&options);
    if options.verbose { println!("{:?}", res); }
    
}