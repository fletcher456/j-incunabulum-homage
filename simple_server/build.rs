// Only run LALRPOP parser generation for native builds
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        lalrpop::process_root().unwrap();
    }
}