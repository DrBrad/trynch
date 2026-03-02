use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use image::{ImageBuffer, Rgb};
use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use crate::bus::event_bus::send_event;
use crate::bus::events::log_event::LogEvent;
use crate::utils::detections::Detections;
use crate::utils::severities::Severities;

pub fn run() {
    thread::spawn(|| {
        let index = CameraIndex::Index(0);

        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::None);

        let mut cam = Camera::new(index, requested).unwrap();
        cam.open_stream().unwrap();

        let mut prev_gray: Vec<u8> = Vec::new();
        let per_pixel_thresh: u8 = 18;
        let motion_pixels_threshold: u32 = 1_000;
        let cooldown = Duration::from_secs(2);
        let mut last_save = Instant::now() - cooldown;

        loop {
            let frame = cam.frame().unwrap();
            let rgb = frame.decode_image::<RgbFormat>().unwrap();
            let w = frame.resolution().width();
            let h = frame.resolution().height();

            let gray = to_gray(&rgb);

            if prev_gray.is_empty() {
                prev_gray = gray;
                continue;
            }

            let changed = motion_score(&prev_gray, &gray, per_pixel_thresh);

            if changed > motion_pixels_threshold && last_save.elapsed() >= cooldown {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                let filename = format!("motion_{}.png", now.as_secs());

                let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(w, h, rgb.clone()).unwrap();
                img.save(&filename).unwrap();

                println!("Motion! changed_pixels={} saved={}", changed, filename);
                send_event(Box::new(LogEvent::new(filename, Detections::Motion, Severities::Warning, now)));

                last_save = Instant::now();
            }

            prev_gray = gray;
        }
    });
}

fn to_gray(rgb: &[u8]) -> Vec<u8> {
    let mut g = Vec::with_capacity(rgb.len() / 3);
    for px in rgb.chunks_exact(3) {
        let y = (px[0] as u16 * 77 + px[1] as u16 * 150 + px[2] as u16 * 29) >> 8;
        g.push(y as u8);
    }
    g
}

fn motion_score(prev: &[u8], cur: &[u8], per_pixel_thresh: u8) -> u32 {
    prev.iter()
        .zip(cur.iter())
        .map(|(&a, &b)| (a.abs_diff(b) > per_pixel_thresh) as u32)
        .sum()
}
