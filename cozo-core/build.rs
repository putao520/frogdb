// System/host sqlite link bridge for storage-sqlite without storage-sqlite-src.
// sqlite is depended with default-features=false so linkage→sqlite3-src is cut
// (avoids cargo `links = "sqlite3"` clash with libsqlite3-sys). FFI still needs
// a provider: emit -lsqlite3 (or Linux soname fallback) at build time.
// When storage-sqlite-src is on, sqlite3-src/bundled supplies the archive instead.

fn main() {
    #[cfg(all(feature = "storage-sqlite", not(feature = "storage-sqlite-src")))]
    link_host_sqlite();
}

#[cfg(all(feature = "storage-sqlite", not(feature = "storage-sqlite-src")))]
fn link_host_sqlite() {
    println!("cargo:rerun-if-env-changed=FROGDB_SQLITE_LIB");
    if let Ok(spec) = std::env::var("FROGDB_SQLITE_LIB") {
        // Explicit override, e.g. "sqlite3" or ":libsqlite3.so.0"
        if spec.starts_with(':') {
            println!("cargo:rustc-link-arg=-l{spec}");
        } else {
            println!("cargo:rustc-link-lib={spec}");
        }
        return;
    }

    // Common lib dirs (Linux + multiarch). Prefer unversioned .so (libsqlite3-dev);
    // fall back to soname .so.0 when only the runtime package is installed.
    const CANDIDATES: &[&str] = &[
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib/aarch64-linux-gnu",
        "/usr/lib64",
        "/usr/lib",
        "/lib/x86_64-linux-gnu",
        "/lib",
    ];

    for dir in CANDIDATES {
        let unversioned = format!("{dir}/libsqlite3.so");
        if std::path::Path::new(&unversioned).exists() {
            println!("cargo:rustc-link-search=native={dir}");
            println!("cargo:rustc-link-lib=dylib=sqlite3");
            return;
        }
    }

    for dir in CANDIDATES {
        let soname = format!("{dir}/libsqlite3.so.0");
        if std::path::Path::new(&soname).exists() {
            println!("cargo:rustc-link-search=native={dir}");
            // Exact soname: works without libsqlite3-dev's unversioned symlink.
            println!("cargo:rustc-link-arg=-l:libsqlite3.so.0");
            return;
        }
    }

    // Last resort: let the linker search the default paths.
    println!("cargo:rustc-link-lib=dylib=sqlite3");
}
