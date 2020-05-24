#![deny(clippy::all)]
#![forbid(unsafe_code)]
mod cpu;
mod flags_register;
mod gpu;
mod instruction_builder;
mod instruction_targets;
mod instructions;
mod ld_tests;
mod memory_map;
mod registers;
mod util;
mod window;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

use cpu::CPU;
use gpu::GPU;
use instruction_builder::InstructionBuilder;
use memory_map::MemoryMap;
use util::Util;
use window::create_window;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 300;

fn main() {
    main_cpu();
    //main_render().map_err(|err| println!("{:?}", err)).ok();
}

fn main_cpu() {
    println!("START");

    let mut memory_map = MemoryMap::new();
    let file_as_byte_array = Util::get_file_as_byte_vec(&String::from("resources/DMG_ROM.bin"));

    memory_map.load_vec(0, file_as_byte_array);
    memory_map.dump();

    let mut _cpu = CPU::new(memory_map);

    let mut _instruction_builder = InstructionBuilder::new();

    // parse next instruction
    let _instruction = _instruction_builder.create(&mut _cpu);
    _instruction.execute(&mut _cpu);

    // parse next instruction
    let _instruction = _instruction_builder.create(&mut _cpu);
    _instruction.execute(&mut _cpu);

    println!("END");
}

#[allow(dead_code)]
fn main_render() -> Result<(), Error> {
    println!("Hello, world!");

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (_window, surface, p_width, p_height, _hidpi_factor) = create_window(
        "I wish I was a little big taller, I wish I was a baller, I wish I had a rabbit in a hat.",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &event_loop,
    );

    // the surface texture
    let surface_texture = SurfaceTexture::new(p_width, p_height, surface);

    // the pixel buffer which is later displayed on the surface texture
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;

    let mut gpu = GPU::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);

    event_loop.run(move |event, _, control_flow| {
        // The one and only event that winit_input_helper doesn't have for us...
        if let Event::RedrawRequested(_) = event {
            // clear buffer by filling with solid color
            for pixel in pixels.get_frame().chunks_exact_mut(4) {
                pixel[0] = 0x00; // R
                pixel[1] = 0xff; // G
                pixel[2] = 0x00; // B
                pixel[3] = 0xff; // A
            }

            // DEBUG: test graphics output
            //gpu.set_pixel(100, 100);
            gpu.set_line(0, 0, 100, 50);

            gpu.draw(pixels.get_frame());

            // Draw the framebuffer to the `SurfaceTexture`
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // For everything else, for let winit_input_helper collect events to build its state.
        // It returns `true` when it is time to update our game state and request a redraw.
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}
