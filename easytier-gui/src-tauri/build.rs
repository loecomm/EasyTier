fn main() {
    #[cfg(all(
      windows,
      target_env = "msvc",
      any(
          target_arch = "x86",
          target_arch = "x86_64"
      )
    ))]
    thunk::thunk();

    tauri_build::build();
}
