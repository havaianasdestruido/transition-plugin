use std::path::Path;
use std::ffi::CString;
use image::{RgbaImage, ImageBuffer};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <name> <frames> <out_dir>", args[0]);
        std::process::exit(1);
    }
    let name = &args[1];
    let frames: usize = args[2].parse().expect("frames integer");
    let out_dir = Path::new(&args[3]);
    std::fs::create_dir_all(out_dir).expect("create out dir");
    let width = 1920u32;
    let height = 1080u32;
    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let c_name = CString::new(name.as_str()).unwrap();
    for i in 0..frames {
        let progress = i as f32 / (frames - 1) as f32;
        unsafe { transition_core::RenderTransition(c_name.as_ptr(), progress, pixels.as_mut_ptr()); }
        let img: RgbaImage = ImageBuffer::from_raw(width, height, pixels.clone()).unwrap();
        let out_path = out_dir.join(format!("{:04}.png", i));
        img.save(out_path).expect("save png");
    }
}
