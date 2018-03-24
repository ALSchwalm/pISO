use bitmap;
use displaymanager;
use error::Result;

pub trait Render {
    fn render(&self, &displaymanager::Window) -> Result<bitmap::Bitmap>;
}
