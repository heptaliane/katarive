use std::env;
use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let suffix = if target.contains("WINDOWS") {
        ".exe"
    } else {
        ""
    };

    // Build sidecars
    let sidecars = vec!["fetcher"];
    let tauri_root_dir = env::current_dir().unwrap();
    let prj_root_dir = tauri_root_dir.parent().unwrap();
    for sidecar in sidecars {
        let sidecar_root_dir = prj_root_dir.join("sidecars").join(sidecar);
        let sidecar_build_dir = prj_root_dir.join("target").join("sidecars").join(sidecar);
        let sidecar_out_dir = tauri_root_dir.join("binaries").join(sidecar);
        let entries = fs::read_dir(&sidecar_root_dir).expect("Failed to read sidecars");

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let name = path.file_name().unwrap().to_str().unwrap();
            let target_root_dir = path.canonicalize().unwrap();
            let target_build_dir = sidecar_build_dir.join(name);
            let target_out_dir = sidecar_out_dir.join(name);

            let mut cargo_cmd = Command::new("cargo");
            cargo_cmd
                .arg("build")
                .arg("-p")
                .arg(&name)
                .current_dir(&target_root_dir)
                .env("CARGO_TARGET_DIR", &target_build_dir);
            if profile == "release" {
                cargo_cmd.arg("--release");
            }
            if target != host {
                cargo_cmd.arg("--target").arg(&target);
            }
            let status = cargo_cmd.status().expect("Failed to build sidecar");
            if !status.success() {
                panic!("Sidecar '{}' build failed", name);
            }

            fs::create_dir_all(&target_out_dir).unwrap();
            let binary_name = format!("{}{}", name, suffix);
            let target_binary_name = format!("{}-{}{}", name, target, suffix);
            let build_path = if target == host {
                target_build_dir.join(&profile).join(&binary_name)
            } else {
                target_build_dir
                    .join(&target)
                    .join(&profile)
                    .join(&binary_name)
            };

            println!(
                "cargo:warning=Copying sidecar from: {}",
                build_path.display()
            );
            fs::copy(&build_path, target_build_dir.join(target_binary_name))
                .expect(&format!("Failed to copy sidecar binary: {}", name));
        }
    }

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &["../proto/v1/fetcher.proto", "../proto/v1/speaker.proto"],
            &["../proto/v1"],
        )?;
    Ok(())
}
