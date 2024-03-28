/*
 * It is necessary to hardcode certain constants into MSL source code but dynamically generate the
 * code so that the Rust binary that runs a shader can insert said constants.
 *
 * Shader lifecycle:
 *
 * MSL source -> Compiled .metallib file -> Loaded by program -> Sent to GPU
 *
 * xcrun -sdk macosx metal -c <path.metal> -o <path.ir>
 * xcrun -sdk macosx metallib <path.ir> -o <path.metallib>
 */

use std::string::String;
use std::path::PathBuf;
use std::process::Command;

pub fn compile_metal(
    path_from_cargo_manifest_dir: &str,
    input_filename: &str
) -> String {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path_from_cargo_manifest_dir).join(input_filename);
    let c = input_path.clone().into_os_string().into_string().unwrap();

    let ir = input_path.clone().into_os_string().into_string().unwrap();
    let ir = format!("{}.ir", ir);

    let exe = Command::new("xcrun")
        .args(["-sdk", "macosx", "metal", "-c", c.as_str(), "-o", ir.as_str()])
        .output()
        .expect("failed to compile");

    if exe.stderr.len() != 0 {
        panic!("{}", String::from_utf8(exe.stderr).unwrap());
    }

    let lib = input_path.clone().into_os_string().into_string().unwrap();
    let lib = format!("{}.lib", lib);

    let exe = Command::new("xcrun")
        .args(["-sdk", "macosx", "metal", ir.as_str(), "-o", lib.as_str()])
        .output()
        .expect("failed to compile");

    if exe.stderr.len() != 0 {
        panic!("{}", String::from_utf8(exe.stderr).unwrap());
    }

    lib
}

//#[cfg(test)]
//pub mod tests {
    //use crate::shader::compile_metal;

    //#[test]
    //pub fn test_compile() {
        //let lib_filepath = compile_metal("../metal/tests", "bigint_add_unsafe.metal");
        //println!("{}", lib_filepath);
    //}
//}
