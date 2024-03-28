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
use std::fs;

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

pub fn write_constants(
    filepath: &str,
    num_limbs: usize,
    log_limb_size: u32,
    n0: u32,
    nsafe: usize
) {
    let two_pow_word_size = 2u32.pow(log_limb_size);
    let mask = two_pow_word_size - 1u32;

    let mut data = "// THIS FILE IS AUTOGENERATED BY shader.rs\n".to_owned();
    data += format!("#define NUM_LIMBS {}\n", num_limbs).as_str();
    data += format!("#define NUM_LIMBS_WIDE {}\n", num_limbs + 1).as_str();
    data += format!("#define LOG_LIMB_SIZE {}\n", log_limb_size).as_str();
    data += format!("#define TWO_POW_WORD_SIZE {}\n", two_pow_word_size).as_str();
    data += format!("#define MASK {}\n", mask).as_str();
    data += format!("#define N0 {}\n", n0).as_str();
    data += format!("#define NSAFE {}\n", nsafe).as_str();

    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(filepath).join("constants.metal");
    fs::write(output_path, data).expect("Unable to write constants file");
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
