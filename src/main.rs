// use crate::ppi

use std::cell::RefCell;
use std::rc::Rc;

use arg::Args;
use log::{Level, LevelFilter, Metadata, Record};
use macroquad::prelude::*;
use rsmsx::libs::graphics::GraphicsType;
use rsmsx::libs::memory::Memory;
use rsmsx::libs::msx::MSX;
use rsmsx::libs::ports::Ports;
use rsmsx::libs::ppi::PPI;
use rsmsx::libs::psg::PSG;
use rsmsx::libs::sound::SoundType;
use rsmsx::libs::vdp::Vdp;
use rsmsx::libs::z80::z80_base::Z80;

const SYSTEM_ROM_FILE: &str = "cbios_main_msx1.rom";

#[derive(Args, Debug)]
///rsmsx 0.1.0
///MSX emulator written in rust
struct MyArgs {
    #[arg(long)]
    ///ROM in SLOT 1
    cart: String,

    ///System file
    #[arg(long = "sys")]
    system_rom: String,

    #[arg(long, default_value = "true")]
    ///Best quality rendering
    quality: bool,

    #[arg(long = "fint", default_value = "16")]
    ///Frame interval in milliseconds
    /// The `frame_interval` variable in the code is used to specify the interval in milliseconds
    /// between frames in the main game loop. This interval determines how often the game logic and
    /// rendering are updated. In this case, the `frame_interval` value is provided as a command-line
    /// argument when running the program, allowing the user to customize the frame rate of the game.
    frame_interval: u32,

    #[arg(long)]
    ///Mapper type (KONAMI4...)
    mtype: String,
}

static MY_LOGGER: MyLogger = MyLogger;

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
        // die on error
        if record.metadata().level() == Level::Error {
            panic!();
        }
    }
    fn flush(&self) {}
}

#[macroquad::main("rsmsx")]
async fn main() {
    log::set_logger(&MY_LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);

    let args: std::vec::Vec<_> = std::env::args().skip(1).collect();
    match MyArgs::from_args(args.iter().map(|x| x.as_str())) {
        Ok(mut args) => {
            if args.system_rom.is_empty() {
                args.system_rom = SYSTEM_ROM_FILE.to_string();
            }
            let ppi = Rc::new(RefCell::new(PPI::new()));
            let mut memory = Memory::new(ppi.clone());
            memory.load_bios_basic(&args.system_rom);
            if !args.cart.is_empty() {
                memory.load_rom(&args.cart, 1, &args.mtype);
            }
            let psg = PSG::new(SoundType::Normal);
            let vdp = Rc::new(RefCell::new(Vdp::new(GraphicsType::Normal, args.quality)));
            vdp.borrow_mut().init_graphics();
            let ports = Ports::new(vdp.clone(), ppi.clone(), psg);
            let mut cpu_z80 = Z80::new(memory, ports);
            cpu_z80.reset();
            cpu_z80.SetPC(0);
            let mut msx = MSX::new(
                cpu_z80,
                vdp.clone(),
                // memory.clone(),
                // ppi.clone(),
                // psg.clone(),
            );

            let avg_fps = msx.main_loop(args.frame_interval as isize).await;
            log::info!("Avg FPS: {:.2}", avg_fps);
        }
        Err(err) => println!("err={:?}", err),
    }
}
