use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let mut out = String::with_capacity(4096);
    let sizes = [32, 64, 128, 256];
    let value = 42;
    for size in sizes {
        let fn_name_stack = format!("index_array_repeat_stack_{size}");
        let fn_name_rodata = format!("index_array_repeat_rodata_{size}");
        let _ = writeln!(
            &mut out,
            "
            pub fn {fn_name_stack}(x: usize)-> u8 {{
                let arr: [u8; {size}] = [{value}; {size}];
                arr[x]
            }}

            pub fn {fn_name_rodata}(x: usize)-> u8 {{
                let arr: [u8; {size}] = *&[{value}; {size}];
                arr[x]
            }}
        "
        );
    }
    let path = Path::new("src/medium_array.rs");
    let mut file = File::create(path).unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
