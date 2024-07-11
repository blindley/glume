
use gl::types::GLenum;

type Error = Box<dyn std::error::Error>;

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

pub fn compile_shader(src: &str, ty: u32) -> Result<u32, Error> {
    let ty_str = shader_type_as_str(ty);
    if ty_str.is_none() {
        return Err("Invalid shader type".into());
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
            Err(msg.into())
        } else {
            Ok(shader)
        }
    }
}

pub fn link_shader_program(shaders: &[u32]) -> Result<u32, Error> {
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
            Err(msg.into())
        } else {
            Ok(program)
        }
    }
}

pub fn create_buffer_f32(data: &[f32], usage: GLenum) -> Result<u32, Error> {
    let mut buffer = 0;
    let data_size = (data.len() * std::mem::size_of::<f32>()) as _;
    let data_ptr = data.as_ptr() as _;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            data_size,
            data_ptr,
            usage,
        );
    }

    Ok(buffer)
}

pub fn create_texture(format: GLenum, size: (u32, u32), data: &[u8])
    -> Result<u32, Error>
{
    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            format as i32,
            size.0 as i32,
            size.1 as i32,
            0,
            format,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as _,
        );
    }

    Ok(texture)
}

pub fn create_texture_rgb(size: (u32, u32), data: &[u8]) -> Result<u32, Error> {
    create_texture(gl::RGB, size, data)
}

pub fn create_texture_rgba(size: (u32, u32), data: &[u8]) -> Result<u32, Error> {
    create_texture(gl::RGBA, size, data)
}

pub extern "system"
fn standard_debug_callback(
    source: u32,
    gltype: u32,
    id: u32,
    severity: u32,
    _length: i32,
    message: *const i8,
    _user_param: *mut std::ffi::c_void,
) {
    if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
        return;
    }

    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        gl::DEBUG_SOURCE_OTHER => "Other",
        _ => "Unknown",
    };

    let gltype = match gltype {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
        gl::DEBUG_TYPE_OTHER => "Other",
        _ => "Unknown",
    };

    let severity = match severity {
        gl::DEBUG_SEVERITY_HIGH => "High",
        gl::DEBUG_SEVERITY_MEDIUM => "Medium",
        gl::DEBUG_SEVERITY_LOW => "Low",
        gl::DEBUG_SEVERITY_NOTIFICATION => "Notification",
        _ => "Unknown",
    };

    unsafe {
        let message = std::ffi::CStr::from_ptr(message).to_str().unwrap();
        println!(
            "OpenGL Debug Message: source: {}, type: {}, id: {}, severity: {}, message: {}",
            source, gltype, id, severity, message
        );
    }
}
