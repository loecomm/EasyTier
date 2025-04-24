fn main() {
	let target = std::env::var("TARGET").unwrap();
    if target == "x86_64-pc-windows-msvc" {
        thunk::thunk();
    }

    tauri_build::build();
}
