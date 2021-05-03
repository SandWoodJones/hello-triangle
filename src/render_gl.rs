use std::ffi::{CString, CStr};

use gl::types::{GLuint, GLchar, GLenum};

// wrapper for gl program
pub struct Program {
	id: GLuint
}

impl Program {
	// takes a slice of Shader structs
	pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
		let program_id = unsafe { gl::CreateProgram() };

		for shader in shaders {
			unsafe { gl::AttachShader(program_id, shader.id()) };
		}

		unsafe { gl::LinkProgram(program_id) };

		let mut result = 1;
		unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut result) };
		
		if result == 0 { // if there's an error
			let mut len = 0;
			unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len) };
			
			let error_msg = create_whitespace_cstring_with_len(len as usize);

			unsafe { gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), error_msg.as_ptr() as *mut GLchar) };

			Err(error_msg.to_string_lossy().into_owned())
		} else {
			for shader in shaders {
				unsafe { gl::DetachShader(program_id, shader.id()) };
			}

			Ok(Program {id: program_id})
		}

	}

	pub fn id(&self) -> GLuint {
		self.id
	}

	pub fn set_used(&self) {
		unsafe { gl::UseProgram(self.id) };
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.id) }
	}
}

// wrapper for gl shader
pub struct Shader {
	id: GLuint
}

impl Shader {
	pub fn from_source(source: &CStr, kind: GLenum) -> Result<Shader, String> {
		Ok(Shader {
			id: shader_from_source(source, kind)?
		})
	}

	pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
		Shader::from_source(source, gl::VERTEX_SHADER)
	}

	pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
		Shader::from_source(source, gl::FRAGMENT_SHADER)
	}

	pub fn id(&self) -> GLuint {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id); }
	}
}

// compiles a shader from a string
fn shader_from_source(
source: &CStr, // needs to be a zero terminated string to pass to glShaderSource()
kind: GLuint) -> Result<GLuint, String> {
	let id = unsafe { gl::CreateShader(kind) }; // gets the shader object's id

	unsafe {
		gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(id)
	}

	let mut result = 1;
	unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut result) }; // get shader's compilation status
	if result == 0 { // if compilation fails
		let mut len = 0;
		unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) } // get the length of the error

		let error_msg = create_whitespace_cstring_with_len(len as usize);

		// finally, write the shader info log into error_msg
		unsafe { gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error_msg.as_ptr() as *mut GLchar) };

		Err(error_msg
				.to_string_lossy() // converts CString to String returns value that can either be String or &str
				.into_owned()) // obtains definitive String
	} else {
		Ok(id)
	}
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
	let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1); // allocate buffer of correct size

	// fill buffer with spaces
	buffer.extend( // the extend() method accepts an iterator and appends new items from it
			[b' '] // a stack-allocated array with a single ASCII space byte
			.iter() // get iterator over array
			.cycle() // repeats iterator forever
			.take(len as usize)); // limits returned items to len

	unsafe { CString::from_vec_unchecked(buffer) }
}
