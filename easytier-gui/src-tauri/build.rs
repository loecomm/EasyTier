fn main() {
	#[cfg(target_os = "windows")]
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "i686" {
        thunk::thunk();
    }

    tauri_build::build();
}
