use std::time::{Duration, Instant};
use std::thread;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    GetAsyncKeyState,
    INPUT_0,
    INPUT_KEYBOARD,
    INPUT_MOUSE,
    INPUT,
    KEYBDINPUT,
    KEYEVENTF_KEYUP,
    MOUSE_EVENT_FLAGS,
    MOUSEEVENTF_ABSOLUTE,
    MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MOVE,
    MOUSEINPUT,
    VK_F6,
    VIRTUAL_KEY,
};
use windows::core::{Result, Error};

fn create_mouse_input(pos: Option<(i32,i32)>, click_type: MOUSE_EVENT_FLAGS) -> INPUT {
    let mut dw_flags = click_type;
    if pos.is_some() {
        dw_flags |= MOUSEEVENTF_ABSOLUTE;
    }
    let (dx,dy) = pos.unwrap_or((0,0));
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: 0,
                dwFlags: dw_flags,
                time: 0,
                dwExtraInfo: 0,
            }
        }
    }
}

fn create_keyboard_input(key: VIRTUAL_KEY, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: 0,
                dwFlags: if up {KEYEVENTF_KEYUP} else {Default::default()},
                time: 0,
                dwExtraInfo: 0,
            }
        }
    }
}

#[repr(transparent)]
pub struct Input(INPUT);

impl Input {
    pub fn move_mouse(pos: (i32,i32)) -> Self {
        Self(create_mouse_input(Some(pos), MOUSEEVENTF_MOVE))
    }

    pub fn click_mouse(pos: Option<(i32,i32)>, up: bool) -> Self {
        Self(create_mouse_input(pos, if up {MOUSEEVENTF_LEFTUP} else {MOUSEEVENTF_LEFTDOWN}))
    }

    pub fn press_key(key: VIRTUAL_KEY, up: bool) -> Self {
        Self(create_keyboard_input(key, up))
    }
}

pub fn send_input(inputs: &[Input]) -> Result<()> {
    if inputs.is_empty() {
        return Ok(());
    }
    // SAFETY: Always safe to transmute repr(transparent) structs
    let inputs = unsafe { std::mem::transmute(inputs) };
    // SAFETY: Only unsafe due to ffi call.
    let num_success = unsafe { SendInput(inputs, std::mem::size_of::<INPUT>() as _) };
    if num_success as usize != inputs.len() {
        return Err(Error::from_win32());
    }
    Ok(())
}

pub fn run_automation(actions: &[(Duration, Vec<Input>)]) -> Result<()> {
    let mut activated = vec![Instant::now(); actions.len()];
    let mut active = false;
    loop {
        let keystate = unsafe { GetAsyncKeyState(VK_F6.0 as _) } as u16;
        if keystate == 0x8001 {
            active = !active;
            println!("active={}", active);
            thread::sleep(Duration::from_millis(100));
            if active {
                let now = std::time::Instant::now();
                for t in &mut activated {
                    *t = now;
                }
            }
        }
        if !active {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        let now = Instant::now();
        for i in 0..actions.len() {
            if now < activated[i] + actions[i].0 {
                continue;
            }
            send_input(&actions[i].1)?;
            activated[i] = now;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
