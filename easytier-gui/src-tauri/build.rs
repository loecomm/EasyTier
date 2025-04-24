fn main() {
	#[cfg(target_os = "windows")]
    if env::var("TARGET").unwrap() == "x86_64-pc-windows-msvc" {
        thunk::thunk();
    }

    tauri_build::build();
}
