//! Build script for the kcapi-sys crate, responsible for building the libkcapi library and generating Rust bindings.
//!
//! This script performs the following steps:
//! 1. Creates a temporary build directory within the OUT_DIR to avoid cluttering the main OUT_DIR with build artifacts.
//! 2. Copies the libkcapi source code to the temporary build directory.
//! 3. Configures and builds the libkcapi library using autotools.
//! 4. Cleans up the temporary build directory after the build is complete.
//! 5. Tells Cargo to link against the built library and where to find it.
//! 6. Generates Rust bindings for the kcapi library using bindgen, pointing to the header file of the built library.
//!
//! **Note**: This build script assumes that the necessary build tools (like autotools) are installed and available in the environment where the build is being performed.
//! Users should ensure that they have the required tools and permissions to build the library when running this script.
//!
//! Overall, this build script provides a convenient way to automate the process of building the libkcapi library and generating Rust bindings, allowing users to easily integrate the functionality of the kcapi library into their Rust applications.
//!
//! # Errors
//! Users of this build script should handle any errors that may occur during the build process, such as issues with copying files, configuring the build, or generating bindings.
//! The script uses `expect` to provide error messages for common failure points, but users should be prepared to troubleshoot any issues that arise during the build process.
//!
//! # Examples
//! ```
//! // This build script is typically run automatically by Cargo when building the kcapi-sys crate, so users do not need to run it manually.
//! // However, if you want to run it manually, you can use the following command in the terminal:
//! cargo build --package kcapi-sys
//! ```
//!
//! This command will trigger the build process for the kcapi-sys crate, which will execute this build script to build the libkcapi library and generate the necessary bindings for use in Rust applications.
//!
//! **Note**: Users should ensure that they have the necessary permissions and environment to build the library when running this command, as it may require access to certain system resources or tools for the build process to complete successfully.

use std::env;
use std::path::PathBuf;

use autotools::Config;
use fs_extra::dir::CopyOptions;

fn main() {
    // Get the OUT_DIR environment variable where Cargo expects build artifacts
    let outdir_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable is not set!"));

    // Create a temporary directory within OUT_DIR for building the library to avoid cluttering the main OUT_DIR with build artifacts
    let outdir_tmp_path = outdir_path.join("tmp");
    std::fs::remove_dir_all(&outdir_tmp_path).ok();
    std::fs::create_dir(&outdir_tmp_path).expect("Failed to create temporary build directory!");

    // Build the libkcapi library and get the path to the header file for generating bindings
    let libkcapi_header_path = build_libkcapi(&outdir_tmp_path);

    // After all build steps are complete, remove the temporary build directory to clean up
    std::fs::remove_dir(outdir_tmp_path)
        .expect("cannot remove temporary build directory after build!");

    // Tell Cargo to link against the built library and where to find it
    println!(
        "cargo::rustc-link-search={}",
        outdir_path
            .join("lib")
            .to_str()
            .expect("libdir path is not a valid string!")
    );
    println!("cargo::rustc-link-lib=kcapi");
    // Generate Rust bindings for the kcapi library using bindgen
    bindgen::Builder::default()
        .header(
            outdir_path
                .join(libkcapi_header_path)
                .to_str()
                .expect("header path is not a valid string!"),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings!")
        .write_to_file(outdir_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// Builds the libkcapi library using autotools and returns the path to the header file for generating bindings.
fn build_libkcapi(outdir_tmp_path: &PathBuf) -> &'static str {
    // Define the path to the libkcapi source code within the temporary directory
    let libkcapi_path = outdir_tmp_path.join("libkcapi");

    // Copy the libkcapi source code to the temporary directory for building
    fs_extra::dir::copy(
        PathBuf::from("libkcapi"),
        outdir_tmp_path,
        &CopyOptions::new().copy_inside(true),
    )
    .expect("Failed to copy libkcapi source code to OUT_DIR!");

    // Configure and build the library using autotools
    let mut config = Config::new(&libkcapi_path);
    // `-i` - Reconfigure the build system to ensure that it is properly set up for building the library, especially if there have been changes to the source code or build configuration
    // `-v` - Enable verbose output for the build process to help with debugging and understanding the build steps
    config.reconf("-iv");
    // Disable dependency tracking to speed up the build process, as we are building a library and do not need incremental builds
    config.disable("dependency-tracking", None);

    // Enable or disable features based on Cargo features
    #[cfg(feature = "year2038")]
    config.enable("year2038", None);
    #[cfg(not(feature = "kdf"))]
    config.disable("lib-kdf", None);
    #[cfg(not(feature = "sym"))]
    config.disable("lib-sym", None);
    #[cfg(not(feature = "md"))]
    config.disable("lib-md", None);
    #[cfg(not(feature = "aead"))]
    config.disable("lib-aead", None);
    #[cfg(not(feature = "rng"))]
    config.disable("lib-rng", None);
    #[cfg(feature = "asym")]
    config.enable("lib-asym", None);
    #[cfg(feature = "kpp")]
    config.enable("lib-kpp", None);
    #[cfg(not(feature = "largefile"))]
    config.disable("largefile", None);

    // Build the library and ensure that the build process completes successfully
    config.build();

    // Clean up the temporary build directory by removing the copied source code
    fs_extra::dir::remove(libkcapi_path)
        .expect("cannot remove libkcapi source directory after build!");

    "include/kcapi.h"
}
