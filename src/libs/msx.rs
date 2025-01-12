use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use super::{vdp::Vdp, z80::z80_base::Z80};

use macroquad::ui::{hash, root_ui};

const CYCLES_PER_FRAME: u64 = 60000; // The z80 runs at 3.58 Mhz. Every 16msec 57280 cycles pass.
const NANO_SEC_PER_SEC: u32 = 1_000_000_000;
const MILLIS_PER_NANO_SEC: u32 = 1_000_000;

fn nanoseconds() -> i64 {
    (get_time() * NANO_SEC_PER_SEC as f64) as i64
}

pub struct MSX {
    cpu_z80: Z80,
    vdp: Rc<RefCell<Vdp>>,
}

impl MSX {
    pub fn new(cpu_z80: Z80, vdp: Rc<RefCell<Vdp>>) -> Self {
        Self { cpu_z80, vdp }
    }
    pub async fn main_loop(&mut self, frame_interval: isize) -> f64 {
        log::info!("Beginning simulation...");
        // state_init();
        let mut controls = Controls::new();

        let mut current_time: i64;
        let mut elapsed_time: i64;
        let mut lag: i64 = 0;
        let mut n_frames: i64 = 0;
        let update_interval = (MILLIS_PER_NANO_SEC as i64) * (frame_interval as i64);

        let start_time = nanoseconds();
        let mut previous_time = start_time;
        let mut paused = false;

        // self.cpu_z80.debug = true;

        loop {
            current_time = nanoseconds();
            elapsed_time = current_time - previous_time;
            previous_time = current_time;
            lag += elapsed_time;
            while lag >= update_interval {
                if !paused {
                    self.cpu_frame();
                }
                lag -= update_interval;
            }

            clear_background(RED);

            // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
            // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
            // draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);

            self.vdp.borrow_mut().update_buffer();
            self.vdp.borrow_mut().graphics_render();

            // if !paused {
            // 	if n_frames%(60*5) == 0 {
            // 		state_save(msx);
            // 	}
            // }

            controls.update();
            if controls.f12 == 1 {
                break;
            }
            if controls.pause == 1 {
                paused = !paused;
            }
            // if controls.space == 1 {
            //     break;
            // }
            // if controls.f12 == 1 {
            // 	state_revert(msx);
            // 	paused = true;
            // }

            // if controls.space == 1 {
            // 	paused = false;
            // }

            root_ui().window(hash!(), vec2(512., 0.), vec2(256., 192. * 2.), |ui| {
                if ui.button(None, "Save") {
                    self.save();
                }
                if ui.button(None, "Load") {
                    self.load();
                }

                ui.separator();
            });

            n_frames += 1;
            next_frame().await;
        }
        let delta = (nanoseconds() - start_time) as f64 / (NANO_SEC_PER_SEC as f64);
        (n_frames as f64) / (delta as f64)
    }

    pub fn cpu_frame(&mut self) {
        self.cpu_z80.cycles %= CYCLES_PER_FRAME;
        while self.cpu_z80.cycles < CYCLES_PER_FRAME {
            if self.cpu_z80.halted {
                break;
            }
            self.cpu_z80.do_opcode();
        }

        if self.vdp.borrow().enabled_interrupts {
            self.vdp.borrow_mut().set_frame_flag();
            self.cpu_z80.interrupt();
        }
    }
    fn save(&mut self) {
        println!("save");
    }
    fn load(&mut self) {
        println!("load");
    }
}

pub struct Controls {
    pause: usize,
    f12: usize,
    space: usize,
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}

impl Controls {
    pub fn new() -> Self {
        Self {
            pause: 0,
            f12: 0,
            space: 0,
        }
    }
    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Pause) {
            if self.pause < 2 {
                self.pause += 1;
            }
        } else {
            self.pause = 0;
        }

        if is_key_pressed(KeyCode::F12) {
            if self.f12 < 2 {
                self.f12 += 1;
            }
        } else {
            self.f12 = 0;
        }

        if is_key_pressed(KeyCode::Space) {
            if self.space < 2 {
                self.space += 1;
            }
        } else {
            self.space = 0;
        }
    }
}
