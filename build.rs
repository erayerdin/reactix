// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

use std::{env, process};

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=frontend/build");

    {
        let dotenv_path =
            dotenvy::from_filename(".build.env").expect("Could not read `.build.env` file.");
        println!("cargo:rerun-if-changed={}", dotenv_path.display());

        dotenvy::dotenv_iter()
            .expect("Could not get env vars from `.build.env`.")
            .for_each(|var| {
                let (key, val) = var.expect("Could not read env var.");
                println!("cargo:rustc-env={key}={val}");
                env::set_var(key, val);
            });
    }

    {
        let project_root_dir = env::current_dir().expect("Could not get project root directory.");
        env::set_current_dir(project_root_dir.join(
            env::var("NODE_PROJECT_DIR").expect("Could not read `NODE_PROJECT_DIR` build env var."),
        ))
        .expect("Could not change cwd to `NODE_PROJECT_DIR`.");

        let child_process = process::Command::new("npm")
            .args(["run", "build"])
            .spawn()
            .expect("Could not build node project.");

        let output = child_process
            .wait_with_output()
            .expect("Could not wait for `npm run build` output.");
        println!(
            "{}",
            String::from_utf8(output.stdout)
                .expect("stdout of `npm run build` has invalid UTF-8 bytes.")
        );
        println!(
            "{}",
            String::from_utf8(output.stderr)
                .expect("stderr of `npm run build` has invalid UTF-8 bytes.")
        );

        env::set_current_dir(project_root_dir)
            .expect("Could not change cwd back to project root directory.");
    }
}
