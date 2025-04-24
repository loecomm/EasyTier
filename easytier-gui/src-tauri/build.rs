fn main() {
	// enable thunk-rs when target os is windows and arch is i686
	let target = std::env::var("TARGET").unwrap_or_default();
	#[cfg(target_os = "windows")]
    if target.contains("i686"){
        thunk::thunk();
    }

    tauri_build::build();
}
