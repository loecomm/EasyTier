fn main() {
	#[cfg(all(
        target_os = "windows",
        any(target_arch = "x86_64", target_arch = "x86")
    ))]
    thunk::thunk();
    tauri_build::build();
}
