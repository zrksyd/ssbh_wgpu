use std::fmt::Write;
use std::path::Path;

fn write_shader_module<P: AsRef<Path>>(wgsl_path: P, output_path: P, include_path: &str) {
    let wgsl_source = std::fs::read_to_string(wgsl_path).unwrap();

    // Generate the Rust bindings and write to a file.
    std::fs::create_dir_all(output_path.as_ref().parent().unwrap()).unwrap();

    let mut text = String::new();
    writeln!(&mut text, "// File automatically generated by build.rs.").unwrap();
    writeln!(&mut text, "// Changes made to this file will not be saved.").unwrap();
    text += &wgsl_to_wgpu::create_shader_module(&wgsl_source, include_path).unwrap();

    std::fs::write(output_path, text.as_bytes()).unwrap();
}

fn main() {
    // TODO: Only rerun if the shaders change?
    let mut shader_paths: Vec<_> = std::fs::read_dir("src/shader")
        .unwrap()
        .into_iter()
        .filter_map(|p| Some(p.ok()?.path()))
        .filter(|p| p.extension().unwrap().to_string_lossy() == "wgsl")
        .collect();

    // Use alphabetical order for consistency.
    shader_paths.sort();

    let mut f = String::new();
    writeln!(&mut f, "// File automatically generated by build.rs.").unwrap();
    writeln!(&mut f, "// Changes made to this file will not be saved.").unwrap();

    let shader_folder = Path::new("src/shader");

    // Create each shader module and add it to shader.rs.
    for shader_path in shader_paths {
        let file_name = shader_path.with_extension("");
        let shader_name = file_name.file_name().unwrap().to_string_lossy().to_string();

        let output_path = shader_folder.join(Path::new(&(shader_name.clone() + ".rs")));

        let include_path = shader_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        writeln!(&mut f, "pub mod {};", shader_name).unwrap();
        write_shader_module(&shader_path, &output_path, &include_path);
    }

    std::fs::write("src/shader.rs", f.as_bytes()).unwrap();
}
