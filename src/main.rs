pub mod app;
pub mod todo;
pub mod storage;
pub mod pages;
pub mod theme;

pub mod widgets; 

use app::App;

fn main() -> iced::Result {
    App::run()
}
