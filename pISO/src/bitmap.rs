use std::ops::{Deref, DerefMut, Index, IndexMut};

#[allow(unused)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Bitmap {
    contents: Vec<Vec<u8>>,
}

impl Bitmap {
    pub fn new(width: usize, height: usize) -> Bitmap {
        Bitmap {
            contents: vec![vec![0; width]; height],
        }
    }

    pub fn from_slice(slice: &[&[u8]]) -> Bitmap {
        let mut contents = Vec::with_capacity(slice.len());

        contents.extend(slice.iter().map(|s| s.to_vec()));

        Bitmap { contents: contents }
    }

    pub fn rotate(&self, dir: Direction) -> Bitmap {
        let mut out = Bitmap::new(self.height(), self.width());
        for (y, row) in self.contents.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                match dir {
                    Direction::Left => {
                        out[self.width() - x - 1][y] = *pixel;
                    }
                    Direction::Right => {
                        out[x][self.height() - y - 1] = *pixel;
                    }
                }
            }
        }
        out
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
                row.extend(vec![0; width - self_width]);
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
            self.contents
                .append(&mut vec![vec![0; self_width]; height - self_height]);
        } else {
            self.contents.truncate(height);
        }
    }

    pub fn blit(&mut self, other: &Bitmap, position: (usize, usize)) {
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
                self.contents[i + position.1][j + position.0] = other[i][j];
            }
        }
    }

    pub fn clip(&mut self, position: (usize, usize), size: (usize, usize)) {
        let mut new_bitmap = Bitmap::new(size.0, size.1);

        for i in position.0..position.0 + size.0 {
            for j in position.1..position.1 + size.1 {
                new_bitmap[j - position.1][i - position.0] = self.contents[j][i];
            }
        }

        *self = new_bitmap;
    }

    // Blit the fraction of 'other' that is visible (x>=0 and y>=0)
    pub fn blit_clip(&mut self, other: &Bitmap, position: (i32, i32)) {
        let mut other = other.clone();
        let (x, y) = position;
        let (current_width, current_height) = (self.width(), self.height());
        let (clip_x, new_width, new_x) = if x < 0 {
            let new_width = other.width() as i32 + x;
            if new_width < 0 {
                (-x, 0, 0)
            } else if new_width > self.width() as i32 {
                (-x, self.width(), 0)
            } else {
                (-x, new_width as usize, 0)
            }
        } else {
            (0, other.width(), x)
        };

        let (clip_y, new_height, new_y) = if y < 0 {
            let new_height = other.height() as i32 + y;
            if new_height < 0 {
                (-y, 0, 0)
            } else if new_height > self.height() as i32 {
                (-x, self.height(), 0)
            } else {
                (-y, new_height as usize, 0)
            }
        } else {
            (0, other.height(), y)
        };
        other.clip(
            (clip_x as usize, clip_y as usize),
            (new_width as usize, new_height as usize),
        );
        self.blit(&other, (new_x as usize, new_y as usize));
        self.set_width(current_width);
        self.set_height(current_height);
    }
}

impl<Idx> Index<Idx> for Bitmap
where
    Idx: Into<usize>,
{
    type Output = Vec<u8>;
    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[index.into()]
    }
}

impl<Idx> IndexMut<Idx> for Bitmap
where
    Idx: Into<usize>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Vec<u8> {
        &mut self.contents[index.into()]
    }
}

impl Deref for Bitmap {
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

#[allow(unused)]
pub enum BorderStyle {
    Top,
    Bottom,
    Left,
    Right,
    All,
}

pub fn with_border(bitmap: Bitmap, style: BorderStyle, mut padding: usize) -> Bitmap {
    padding += 1;
    match style {
        BorderStyle::Top => {
            let mut top_added = Bitmap::new(bitmap.width(), bitmap.height() + padding);
            top_added.blit(&bitmap, (0, padding));
            for pixel in top_added[0 as usize].iter_mut() {
                *pixel = 1;
            }
            top_added
        }
        BorderStyle::Bottom => {
            let mut bottom_added = Bitmap::new(bitmap.width(), bitmap.height() + padding);
            bottom_added.blit(&bitmap, (0, 0));
            for pixel in bottom_added.iter_mut().last().unwrap().iter_mut() {
                *pixel = 1;
            }
            bottom_added
        }
        BorderStyle::Left => {
            let mut left_added = Bitmap::new(bitmap.width() + padding, bitmap.height());
            left_added.blit(&bitmap, (padding, 0));
            for row in left_added.iter_mut() {
                row[0] = 1;
            }
            left_added
        }
        BorderStyle::Right => {
            let mut right_added = Bitmap::new(bitmap.width() + padding, bitmap.height());
            right_added.blit(&bitmap, (0, 0));
            for row in right_added.iter_mut() {
                *row.last_mut().unwrap() = 1;
            }
            right_added
        }
        BorderStyle::All => {
            let top_added = with_border(bitmap, BorderStyle::Top, padding);
            let bottom_added = with_border(top_added, BorderStyle::Bottom, padding);
            let left_added = with_border(bottom_added, BorderStyle::Left, padding);
            with_border(left_added, BorderStyle::Right, padding)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clip() {
        let bitmap = Bitmap::from_slice(&[
            &[1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 0, 1],
            &[1, 1, 1, 1, 1],
        ]);

        let zeros = Bitmap::from_slice(&[&[0, 0, 0], &[0, 0, 0], &[0, 0, 0]]);

        let mut clipped = bitmap.clone();
        clipped.clip((1, 1), (3, 3));
        assert_eq!(clipped, zeros);

        let mut clipped = bitmap.clone();
        clipped.clip((0, 0), (3, 3));
        assert_eq!(
            clipped,
            Bitmap::from_slice(&[&[1, 1, 1], &[1, 0, 0], &[1, 0, 0]])
        );
    }

    #[test]
    fn test_blit_clip() {
        let ones = Bitmap::from_slice(&[
            &[1, 1, 1, 1, 1],
            &[1, 1, 1, 1, 1],
            &[1, 1, 1, 1, 1],
            &[1, 1, 1, 1, 1],
            &[1, 1, 1, 1, 1],
        ]);

        let zeros =
            Bitmap::from_slice(&[&[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0]]);

        let mut bitmap = zeros.clone();
        bitmap.blit_clip(&ones, (0, -4));
        assert_eq!(
            bitmap,
            Bitmap::from_slice(&[&[1, 1, 1, 1], &[0, 0, 0, 0], &[0, 0, 0, 0], &[0, 0, 0, 0],])
        );

        let mut bitmap = zeros.clone();
        bitmap.blit_clip(&ones, (0, 10));
        assert_eq!(bitmap, zeros);
    }
}
