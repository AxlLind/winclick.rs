use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_MOUSE, INPUT_0, MOUSEINPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSE_EVENT_FLAGS, GetAsyncKeyState, VK_F6};

fn create_mouse_input(dx: i32, dy: i32, click_type: MOUSE_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: 0,
                dwFlags: click_type,
                time: 0,
                dwExtraInfo: 0,
            }
        }
    }
}

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
                create_mouse_input(0, 0, MOUSEEVENTF_LEFTDOWN),
                create_mouse_input(0, 0, MOUSEEVENTF_LEFTUP),
            ];
            let num_success = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as _) };
            assert!(num_success == 2);
        }
    }
}
