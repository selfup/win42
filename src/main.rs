use fut::write_file;
use serde_json::json;
use std::env;
use std::fs;
use walkdir::WalkDir;

fn main() {
    let mut dir: String = String::from("");
    let mut size: String = String::from("");
    let mut json: String = String::from("");

    let mut files = vec![];
    let mut errors = vec![];

    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        dir = args[1].clone();
        size = args[2].clone();
    } else if args.len() == 4 {
        dir = args[1].clone();
        size = args[2].clone();
        json = args[3].clone();
    } else {
        panic!("bad args: {} {} {}", dir, size, json);
    }

    let file_size: u64;

    match size.parse::<u64>() {
        Ok(bytes) => {
            file_size = bytes;
        }
        Err(err) => panic!("failed to parse size: {}", err),
    }

    for entry in WalkDir::new(dir) {
        match &entry {
            Ok(entry) => {
                let path = entry.path().display();
                let path_str = format!("{}", path);
                let path_str_clone = path_str.clone();

                match fs::metadata(path_str) {
                    Ok(meta) => {
                        if meta.len() > file_size {
                            files.push(path_str_clone);
                        }
                    }
                    Err(err) => {
                        let err_msg = format!("META ERR: {:?}", err);
                        errors.push(err_msg);
                    }
                };
            }
            Err(err) => {
                let err_msg = format!("UWRP ERR: {:?}", err);
                errors.push(err_msg);
            }
        }
    }

    if json == String::from("json") {
        let result = json!({
            "files": files,
            "errors": errors,
        });

        let result_json = format!("{}", result);

        write_file(result_json, "win42.json");
    } else {
        println!("errors:");
        println!("{}", errors.join("\n"));
        println!("files:");
        println!("{}", files.join("\n"));
    }
}
