// Copyright 2022 Eray Erdin.
// Use of this source code is governed by the WTFPL
// license that can be found in the LICENSE file.

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    let dotenv_path =
        dotenvy::from_filename(".build.env").expect("Could not read `.build.env` file.");
    println!("cargo:rerun-if-changed={}", dotenv_path.display());

    dotenvy::dotenv_iter()
        .expect("Could not get env vars from `.build.env`.")
        .for_each(|var| {
            let (key, val) = var.expect("Could not read env var.");
            println!("cargo:rustc-env={key}={val}");
        });
}
