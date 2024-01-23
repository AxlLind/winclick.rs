use std::time::Duration;

fn main() {
    winclick::run_automation(&[(Duration::from_millis(5), "")]).expect("winclick loop failed");
}
