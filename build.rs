use embed_resource::CompilationResult;

fn main() {
    // This will be a noop if not windows.
    add_windows_icon();
}

/// Adds the windows icon during build.
fn add_windows_icon() {
    println!("cargo:rerun-if-changed=./private/icon/export/windows.ico");
    let result_icon =
        embed_resource::compile("./private/icon/windows.rc", embed_resource::NONE);

    match result_icon {
        CompilationResult::Ok => {
            println!("Added Windows icon.");
        }
        CompilationResult::NotWindows => {
            println!("Not on Windows, skipping Windows icon.");
        }
        _ => {
            panic!("Bundling the Windows icon failed.");
        }
    }
}
