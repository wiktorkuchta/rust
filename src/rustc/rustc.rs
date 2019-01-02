#![feature(link_args)]

// Set the stack size at link time on Windows. See rustc_driver::in_rustc_thread
// for the rationale.
#[allow(unused_attributes)]
#[cfg_attr(all(windows, target_env = "msvc"), link_args = "/STACK:16777216")]
// We only build for msvc and gnu now, but we use a exhaustive condition here
// so we can expect either the stack size to be set or the build fails.
#[cfg_attr(all(windows, not(target_env = "msvc")), link_args = "-Wl,--stack,16777216")]
// Also, don't forget to set this for rustdoc.
extern {}

fn main() {
    // Pull in jemalloc when enabled.
    //
    // Note that we're pulling in a static copy of jemalloc which means that to
    // pull it in we need to actually reference its symbols for it to get
    // linked. The two crates we link to here, std and rustc_driver, are both
    // dynamic libraries. That means to pull in jemalloc we need to actually
    // reference allocation symbols one way or another (as this file is the only
    // object code in the rustc executable).
    //
    // As such, here's an interprative dance to slip under LLVM's radar yet
    // please the linker.
    #[cfg(feature = "jemalloc-sys")] {
        if std::env::var("__RUSTC_NEVER_PRESENT_ENV_VAR").is_ok() {
            unsafe {
                jemalloc_sys::calloc(1, 4);
                jemalloc_sys::posix_memalign(&mut std::ptr::null_mut(), 1, 10);
                jemalloc_sys::aligned_alloc(1, 10);
                let a = jemalloc_sys::malloc(3);
                let b = jemalloc_sys::realloc(a, 100);
                println!("{:?}", b);
                jemalloc_sys::free(b);
            }
        }
    }

    rustc_driver::set_sigpipe_handler();
    rustc_driver::main()
}
