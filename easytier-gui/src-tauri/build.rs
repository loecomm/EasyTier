fn main() {
	#[cfg(target_os = "windows")]
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64-pc-windows-msvc" {
        thunk::thunk();
    }

    tauri_build::build();
}
