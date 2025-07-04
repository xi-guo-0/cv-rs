#[derive(Clone)]
pub enum Image {
    Gray {
        width: usize,
        height: usize,
        data: Vec<u8>,
    },
    Rgb {
        width: usize,
        height: usize,
        data: Vec<u8>,
    },
}

impl Image {
    pub fn gray(width: usize, height: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), width * height);
        Image::Gray {
            width,
            height,
            data,
        }
    }
    pub fn rgb(width: usize, height: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), width * height * 3);
        Image::Rgb {
            width,
            height,
            data,
        }
    }
    pub fn width(&self) -> usize {
        match self {
            Image::Gray { width, .. } | Image::Rgb { width, .. } => *width,
        }
    }
    pub fn height(&self) -> usize {
        match self {
            Image::Gray { height, .. } | Image::Rgb { height, .. } => *height,
        }
    }
    pub fn data(&self) -> &Vec<u8> {
        match self {
            Image::Gray { data, .. } | Image::Rgb { data, .. } => data,
        }
    }
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        match self {
            Image::Gray { data, .. } | Image::Rgb { data, .. } => data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_image() {
        let w = 4;
        let h = 3;
        let data = vec![1u8; w * h];
        let img = Image::gray(w, h, data.clone());
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);
        assert_eq!(img.data(), &data);
    }

    #[test]
    fn test_rgb_image() {
        let w = 2;
        let h = 2;
        let data = vec![255u8, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 0];
        let img = Image::rgb(w, h, data.clone());
        assert_eq!(img.width(), w);
        assert_eq!(img.height(), h);
        assert_eq!(img.data(), &data);
    }
}
