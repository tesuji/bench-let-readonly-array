use rand::Rng;
use std::fmt::Write as _;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    let path = Path::new("src/medium_array.rs");

    // if path.is_file() {
    //     return;
    // }

    let mut rng = rand::thread_rng();

    let mut out = String::with_capacity(4096);
    let sizes = [
        // size < 16, there is no difference as llvm inlines all arrays
        // 1, 2, 4, 8, 16,
        32, 64, 128, 256,
    ];
    for size in sizes {
        // use the same backing array for rodata and stack variant.
        // LLVM don't merge the same array across different fns.
        // see <https://rust.godbolt.org/z/Y8qfW16Y7>.
        let vec1: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();

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
                let arr: [u8; {size}] = *&{vec1:?};
                arr[x]
            }}
        "
        );
    }
    let mut file = File::create(path).unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
