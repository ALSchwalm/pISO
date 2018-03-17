use std::ops::{Deref, DerefMut, Index};

pub struct Bitmap{
    contents: Vec<Vec<u8>>
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        Bitmap {
            contents: vec![vec![0; width]; height]
        }
    }

    pub fn from_slice(slice: &[&[u8]]) -> Bitmap {
        let mut contents = Vec::with_capacity(slice.len());

        contents.extend(slice.iter().map(|s| s.to_vec()));

        Bitmap {
            contents: contents
        }
    }

    pub fn width(&self) -> usize {
        self.contents.get(0).map(|v| v.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.contents.len()
    }

    pub fn set_width(&mut self, width: usize) {
        if width > self.width() {
            let self_width = self.width();
            for row in self.contents.iter_mut() {
                row.extend((0..).take(width - self_width));
            }
        } else {
            for row in self.contents.iter_mut() {
                row.truncate(width);
            }
        }
    }

    pub fn set_height(&mut self, height: usize) {
        if height > self.height() {
            let self_width = self.width();
            let self_height = self.height();
            self.contents.append(&mut vec![
                vec![0;  self_width];
                height - self_height
            ]);
        } else {
            self.contents.truncate(height);
        }
    }

    pub fn blit(&mut self, other: Bitmap, position: (usize, usize)) {
        // If the current contents are empty, then set_width/height don't do anything,
        // (as expected), so create a 1x1 bitmap and expand that.
        if self.width() == 0 {
            self.contents = vec![vec![0]];
        }
        if other.width() + position.0 > self.width() {
            self.set_width(other.width() + position.0);
        }
        if other.height() + position.1 > self.height() {
            self.set_height(other.height() + position.1);
        }

        for i in 0..other.height() {
            for j in 0..other.width() {
                self.contents[i+position.1][j+position.0] = other[i][j];
            }
        }
    }
}

impl<Idx> Index<Idx> for Bitmap where Idx: Into<usize> {
    type Output = Vec<u8>;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[index.into()]
    }
}

impl Deref for Bitmap{
    type Target = Vec<Vec<u8>>;
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl DerefMut for Bitmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.contents
    }
}
