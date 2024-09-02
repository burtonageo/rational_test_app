use cfg_if::cfg_if;
use std::{
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let workspace_root = Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .map_err(Box::<dyn Error + Send + Sync + 'static>::from)
        .and_then(|output| {
            if !output.status.success() {
                let msg = format!(
                    "cargo locate-project failed: {}",
                    String::from_utf8_lossy(&output.stderr),
                );
                return Err(From::from(msg));
            }

            let mut stdout = output.stdout;
            if stdout
                .last()
                .map(|ch| ch.is_ascii_whitespace())
                .unwrap_or_default()
            {
                stdout.pop(); // probably a trailing '\n', pop it
            }

            let mut wspace_root = {
                let s = make_osstring(stdout)?;
                PathBuf::from(s)
            };

            if !wspace_root.is_dir() {
                wspace_root.pop(); // pop the "Cargo.toml"
            }

            Ok(wspace_root)
        })?;

    let crate_name = "rational_impl";

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let profile = env::var("PROFILE").unwrap_or_default();

    let crate_dir = workspace_root.join("crates").join(crate_name);

    rerun_cargo_if_dir_changed(&crate_dir)?;

    let mut out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace_root.join("target"))
        .join(crate_name);

    let mut cargo_build = Command::new(cargo);
    cargo_build
        .arg("build")
        .env("CARGO_BUILD_TARGET_DIR", &out_dir)
        .current_dir(&crate_dir);

    if env::var("CARGO_FEATURE_LINK_STATIC").is_ok() {
        cargo_build.args(&["--features", "link_static"]);
    }

    if let Ok(triple) = env::var("TARGET") {
        cargo_build.args(&["--target", &triple]);
        out_dir.push(&triple);
    }
    out_dir.push(&profile);

    match profile.as_str() {
        "debug" => (),
        "release" => {
            cargo_build.arg("--release");
        }
        profile => {
            cargo_build.args(&["--profile", &profile]);
        }
    }

    let output = cargo_build.spawn()?.wait_with_output()?;
    if !output.status.success() {
        return Err(From::from(String::from_utf8_lossy(&output.stderr)));
    }

    let crate_type = if cfg!(feature = "link_static") {
        "static"
    } else {
        "dylib"
    };

    println!(
        "cargo:rustc-link-search={}",
        out_dir.display()
    );
    println!("cargo:rustc-link-lib={}={}", crate_type, crate_name);

    Ok(())
}

fn make_osstring(stdout: Vec<u8>) -> Result<OsString, Box<dyn Error + Send + Sync + 'static>> {
    cfg_if! {
        if #[cfg(unix)] {
            use std::os::unix::ffi::OsStringExt;
            Ok(OsString::from_vec(stdout))
        } else if #[cfg(target_os = "wasi")] {
            use std::os::wasi::ffi::OsStringExt;
            Ok(OsString::from_vec(output))
        } else {
            let s = String::from_utf8(stdout)?;
            Ok(From::from(s))
        }
    }
}

#[inline]
fn rerun_cargo_if_dir_changed<P: ?Sized + AsRef<Path>>(path: &P) -> io::Result<()> {
    fn inner(path: &Path) -> io::Result<()> {
        for item in fs::read_dir(path)? {
            let item = item?;
            let path = item.path();
            if path.is_dir() {
                rerun_cargo_if_dir_changed(&path)?;
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }

        Ok(())
    }

    inner(path.as_ref())
}
