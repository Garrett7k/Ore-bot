extern crate autopilot;
extern crate image;
use autopilot::{geometry::{Point, Rect, Size}, key::toggle};
use rand::Rng;

fn main() {
    let osrs_window_rect = Rect::new(Point::new(400.0, 25.0), Size::new(762.0, 500.0));
    let afk: [u8;4]= [237,85,125,255];
    let afk = image::Rgba(afk);
    let iron_ore_deposit_1: [u8;4] = [110,84,236,255];
    let iron_ore_deposit_2: [u8;4] = [255,173,0,255];
    let iron_ore_deposit_3: [u8;4] = [255,0,221,255];
    let iron_in_inv: [u8;4] = [0,255,135,255];
    let iron_in_inv_pixel = image::Rgba(iron_in_inv);
    let mut rng_sleeper = rand::thread_rng();

    loop {

        let a = autopilot::bitmap::capture_screen().expect("Cant take screenshot");
        if let Some(_) = a.find_color(afk, Some(0.), Some(osrs_window_rect), None) {
            detect_move_click(iron_ore_deposit_1, osrs_window_rect);
            half_sleep(rng_sleeper.gen_range(72..90));
            detect_move_click(iron_ore_deposit_2, osrs_window_rect);
            half_sleep(rng_sleeper.gen_range(70..107));
            detect_move_click(iron_ore_deposit_3, osrs_window_rect);
        }
        
        let c = autopilot::bitmap::capture_screen().expect("Cant take screenshot");
        let pixel_of_28th_slot = c.get_pixel(autopilot::geometry::Point::new(1105.0, 470.0));
        if pixel_of_28th_slot == iron_in_inv_pixel {
            toggle(&autopilot::key::Code(autopilot::key::KeyCode::Shift), true, &[], 0);
            for _ in 0..=27 {
                detect_move_click(iron_in_inv, osrs_window_rect);
                half_sleep(rng_sleeper.gen_range(67..144));
            }
            toggle(&autopilot::key::Code(autopilot::key::KeyCode::Shift), false, &[], 0);
        }
    }
}

fn detect_move_click(pixel: [u8; 4], nrect: Rect) {
    let mut rng1 = rand::thread_rng();
    let p = image::Rgba(pixel);
    let p_scan = autopilot::bitmap::capture_screen().expect("Cant take screenshot");
    let all_points_in_p_scan = p_scan.find_every_color(p, Some(0.), Some(nrect), None);
    let random_testing =  rng1.gen_range(0..all_points_in_p_scan.len());
    autopilot::mouse::move_to(all_points_in_p_scan[random_testing]).unwrap();
    half_sleep(rng1.gen_range(800..840));
    autopilot::mouse::click(autopilot::mouse::Button::Left, Some(0));
    half_sleep(rng1.gen_range(700..740));

    
}

fn half_sleep(n: u64) {
    std::thread::sleep(std::time::Duration::from_millis(1000 - n));
}