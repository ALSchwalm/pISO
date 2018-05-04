use bitmap;
use displaymanager;
use error::Result;

pub trait Render {
    fn render(
        &self,
        &displaymanager::DisplayManager,
        &displaymanager::Window,
    ) -> Result<bitmap::Bitmap> {
        Ok(bitmap::Bitmap::new(0, 0))
    }
}
