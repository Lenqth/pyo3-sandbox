use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut file = File::create("./build.log").unwrap();
    let args: Vec<String> = env::args().collect();
    let envs: HashMap<String, String> = env::vars().collect();

    //    writeln!(file, "{:#?}", args).unwrap();
    writeln!(file, "{:#?}", envs).unwrap();
    file.flush().unwrap();
    let cargo_pkg_name = envs.get("CARGO_PKG_NAME").unwrap();

    if let Some(outdir) = envs.get("OUT_DIR") {
        let mut path = Path::new(outdir);
        while let Some(new_path) = path.parent() {
            path = new_path;
            if Some("release".as_ref()) == path.file_name() {
                writeln!(file, "{}/{}.dll", path.to_str().unwrap(), cargo_pkg_name).unwrap();
                std::fs::copy(
                    format!("{}/{}.dll", path.to_str().unwrap(), cargo_pkg_name),
                    format!("./{}.pyd", cargo_pkg_name),
                )
                .ok();
                break;
            }
            if path.parent() == Some(path) {
                break;
            }
        }
    }
}
