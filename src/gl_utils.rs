
fn shader_source(shader: u32, src: &str) {
    let src = std::ffi::CString::new(src).unwrap();
    let src_ptr = src.as_ptr();

    unsafe {
        gl::ShaderSource(shader, 1, &src_ptr, std::ptr::null());
    }
}

fn shader_type_as_str(ty: u32) -> Option<&'static str> {
    match ty {
        gl::VERTEX_SHADER => Some("vertex"),
        gl::TESS_CONTROL_SHADER => Some("tess control"),
        gl::TESS_EVALUATION_SHADER => Some("tess evaluation"),
        gl::GEOMETRY_SHADER => Some("geometry"),
        gl::FRAGMENT_SHADER => Some("fragment"),
        gl::COMPUTE_SHADER => Some("compute"),
        _ => None,
    }
}

pub fn compile_shader(src: &str, ty: u32) -> Result<u32, String> {
    let ty_str = shader_type_as_str(ty);
    if ty_str.is_none() {
        return Err("Invalid shader type".to_string());
    }

    let ty_str = ty_str.unwrap();

    unsafe {
        let shader = gl::CreateShader(ty);
        shader_source(shader, src);

        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0; len as usize];
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as _);
            let log = std::str::from_utf8(&buffer).unwrap();
            let msg = format!("Failed to compile {} shader: {}", ty_str, log);

            gl::DeleteShader(shader);
            Err(msg)
        } else {
            Ok(shader)
        }
    }
}

pub fn link_shader_program(shaders: &[u32]) -> Result<u32, String> {
    unsafe {
        let program = gl::CreateProgram();
        for &shader in shaders {
            gl::AttachShader(program, shader);
        }

        gl::LinkProgram(program);

        for &shader in shaders {
            gl::DetachShader(program, shader);
        }

        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0; len as usize];
            gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), buffer.as_mut_ptr() as _);
            let log = std::str::from_utf8(&buffer).unwrap();
            let msg = format!("Failed to link shader program: {}", log);

            gl::DeleteProgram(program);
            Err(msg)
        } else {
            Ok(program)
        }
    }
}
