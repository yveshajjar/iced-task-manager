pub mod app;
pub mod tasks;
pub mod storage;
pub mod pages;

pub mod widgets; 

use app::App;

fn main() -> iced::Result {
    App::run()
}
