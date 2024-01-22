use std::time::Duration;
use winclick::{Input, send_input};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_F6};

fn main() {
    let mut clicking = false;
    loop {
        let state = unsafe { GetAsyncKeyState(VK_F6.0 as _) } as u16;
        if state == 0x8001 {
            clicking = !clicking;
            println!("clicking={}", clicking);
        }
        if clicking {
            let inputs = [
                Input::click_mouse(None, false),
                Input::click_mouse(None, true),
            ];
            send_input(&inputs).expect("Failed to send inputs");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
