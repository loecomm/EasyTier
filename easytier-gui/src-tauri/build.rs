fn main() {
    #[cfg(all(windows, target_env = "msvc", target_arch = "x86"))]
    thunk::thunk();
    tauri_build::build();
}
