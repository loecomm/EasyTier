fn main() {
    let target_os = std::env::consts::OS;
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_arch = std::env::consts::ARCH;
	if target_os == "windows" && target_env == "msvc" && target_arch == "x86" 
    {
        thunk::thunk();
    }

    tauri_build::build();
}
