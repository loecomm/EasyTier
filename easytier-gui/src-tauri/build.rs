fn main() {
	// enable thunk-rs when target os is windows and arch is i686
	#[cfg(target_os = "windows")]
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "i686" {
        thunk::thunk();
    }

    tauri_build::build();
}
