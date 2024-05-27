use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
    use rand::Rng;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    let mut rng = rand::thread_rng();

    let mut out = String::with_capacity(4096);
    let sizes = [32, 64, 128, 256];
    for size in sizes {
        let vec1: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();
        let vec2: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();

        let fn_name_stack = format!("index_array_random_stack_{size}");
        let fn_name_rodata = format!("index_array_random_rodata_{size}");
        let _ = writeln!(
            &mut out,
            "
            pub fn {fn_name_stack}(x: usize)-> u8 {{
                let arr: [u8; {size}] = {vec1:?};
                arr[x]
            }}

            pub fn {fn_name_rodata}(x: usize)-> u8 {{
                let arr: [u8; {size}] = *&{vec2:?};
                arr[x]
            }}
        "
        );
    }
    let path = Path::new("src/medium_array.rs");
    let mut file = File::create(path).unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
