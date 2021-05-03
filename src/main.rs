use glutin::event::{Event, WindowEvent, VirtualKeyCode};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest, Api};
use glutin::dpi::PhysicalSize;

use gl;

use std::ffi::CString;

pub mod render_gl;

use render_gl::{Shader, Program};

static WINDOW_W: u16 = 512;
static WINDOW_H: u16 = 512;

fn main() {
	let event_loop = EventLoop::new();

	let window_builder = WindowBuilder::new()
							.with_title("triangle")
							.with_inner_size(PhysicalSize::new(WINDOW_W, WINDOW_H))
							.with_resizable(false);

	let windowed_context = ContextBuilder::new()
							.with_gl_profile(GlProfile::Core)
							.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1))) // set the opengl version
							.build_windowed(window_builder, &event_loop) // builds the window and associates it to a gl context
							.unwrap();
	let windowed_context = unsafe { windowed_context.make_current().unwrap() };  // sets this context as the current context

	#[allow(unused_variables)]
	let gl = gl::load_with(|symbol| windowed_context.context().get_proc_address(symbol));
	
	// create shaders
	let vertex_shader = Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
	let frag_shader = Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();
	// link shaders
	let shader_program = Program::from_shaders(&[vertex_shader, frag_shader]).unwrap();
	// use shaders
	shader_program.set_used();

	// triangle vertices
	let vertices: Vec<f32> = vec![
		// positions		// colors
		 0.6, -0.5, 0.0,	1.0, 0.0, 0.0, // bottom right
		-0.6, -0.5, 0.0,	0.0, 1.0, 0.0, // bottom left
		 0.0,  0.5, 0.0,	0.0, 0.0, 1.0  // top
	];

	let mut vbo = 0; // Vertex Buffer Object
	unsafe { gl::GenBuffers(1, &mut vbo) };

	// binds buffer and sends data to it
	unsafe {
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER, // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer do data
			gl::STATIC_DRAW // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
	}

	let mut vao = 0; // Vertex Array Object
	unsafe { gl::GenVertexArrays(1, &mut vao) };

	// makes it current by binding it
	unsafe {
		gl::BindVertexArray(vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

		gl::EnableVertexAttribArray(0); // layout (location = 0)
		gl::VertexAttribPointer(
			0, // index for generic vertex attribute 
			3, // number of components per generic vertex attribute
			gl::FLOAT, // type of data
			gl::FALSE, // normalized?
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint, // offset between consecutive attributes (each vertex is made of 6 floats)
			std::ptr::null() // offset of first component
		);

		gl::EnableVertexAttribArray(1); // layout (location = 1)
		gl::VertexAttribPointer(
			1,
			3,
			gl::FLOAT,
			gl::FALSE,
			(6 * std::mem::size_of::<f32>()) as gl::types::GLint,
			(3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
		);
		
		// unbinds vbo and vao
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}

	unsafe {
		gl::Viewport(0, 0, WINDOW_W as i32, WINDOW_H as i32); // tells opengl the size of the window
		gl::ClearColor(0.4, 0.4, 0.5, 1.0);
	}

	// main loop
	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait; // pauses the loop if there are no events

		match event {
			Event::LoopDestroyed => return,
			Event::WindowEvent {event: w_event, ..} => match w_event {
				WindowEvent::Resized(new_size) => windowed_context.resize(new_size), // updates the window if its resized
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit, // exits the loop if the window is closed
				WindowEvent::KeyboardInput {input, ..} => match input.virtual_keycode {
					Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit, // quit when pressing esc
					_ => ()
				},
				_ => ()
			},

			Event::RedrawRequested(_) => {
				unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

				shader_program.set_used();
				unsafe {
					gl::BindVertexArray(vao);
					gl::DrawArrays(
						gl::TRIANGLES, // mode
						0, // start of index
						3 // number of indices
					)
				}


				windowed_context.swap_buffers().unwrap();
			},

			_ => ()
		}
	})
}
