#[macro_use]
extern crate glium;

#[cfg(feature = "image")]
extern crate image;

use glium::Surface;
use glium::glutin;
use glium::index::PrimitiveType;
use std::path::Path;

#[cfg(not(feature = "image"))]
fn main() {
    println!("You need to compile glium with the `image` feature in order to run this example");
}

#[cfg(feature = "image")]
fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_visibility(false)
        .build_glium()
        .unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display, 
            vec![
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
            ]
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               vec![0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 position;
                in vec3 color;

                out vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;

                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {  
            vertex: "
                #version 110

                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec3 color;

                varying vec3 vColor;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;

                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    ).unwrap();

    // drawing once

    // building the uniforms
    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    // drawing a frame
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    // reading the front buffer into an image
    let image: image::DynamicImage = display.read_front_buffer();
    let mut output = std::fs::File::create(&Path::new("glium-example-screenshot.png")).unwrap();
    image.save(&mut output, image::ImageFormat::PNG).unwrap();
}
