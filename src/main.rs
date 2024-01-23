use std::time::Duration;
use winclick::Input;

fn main() {
    let actions = [
        (Duration::from_millis(5), vec![
            Input::click_mouse(None, false),
            Input::click_mouse(None, true),
        ])
    ];
    winclick::run_automation(&actions).expect("winclick loop failed");
}
