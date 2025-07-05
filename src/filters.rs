use crate::image::Image;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResizeBackend {
    Cpu,
    Simd,
    Gpu,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResizeAlgorithm {
    Nearest,
    Bilinear,
    Bicubic,
}

pub fn resize(
    img: &Image,
    new_width: usize,
    new_height: usize,
    backend: ResizeBackend,
    algorithm: ResizeAlgorithm,
) -> Image {
    match backend {
        ResizeBackend::Cpu => match algorithm {
            ResizeAlgorithm::Nearest => resize_nearest_cpu(img, new_width, new_height),
            ResizeAlgorithm::Bilinear => unimplemented!("Bilinear resize not implemented yet"),
            ResizeAlgorithm::Bicubic => unimplemented!("Bicubic resize not implemented yet"),
        },
        ResizeBackend::Simd => unimplemented!("SIMD resize not implemented yet"),
        ResizeBackend::Gpu => unimplemented!("GPU resize not implemented yet"),
    }
}

fn resize_nearest_cpu(img: &Image, new_width: usize, new_height: usize) -> Image {
    match img {
        Image::Gray {
            width,
            height,
            data,
        } => {
            let mut new_data = vec![0u8; new_width * new_height];
            for y in 0..new_height {
                for x in 0..new_width {
                    let src_x = x * width / new_width;
                    let src_y = y * height / new_height;
                    new_data[y * new_width + x] = data[src_y * width + src_x];
                }
            }
            Image::gray(new_width, new_height, new_data)
        }
        Image::Rgb {
            width,
            height,
            data,
        } => {
            let mut new_data = vec![0u8; new_width * new_height * 3];
            for y in 0..new_height {
                for x in 0..new_width {
                    let src_x = x * width / new_width;
                    let src_y = y * height / new_height;
                    let src_idx = (src_y * width + src_x) * 3;
                    let dst_idx = (y * new_width + x) * 3;
                    new_data[dst_idx..dst_idx + 3].copy_from_slice(&data[src_idx..src_idx + 3]);
                }
            }
            Image::rgb(new_width, new_height, new_data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Image;

    #[test]
    fn test_resize_gray_nearest() {
        let img = Image::gray(2, 2, vec![10, 20, 30, 40]);
        let resized = resize(&img, 4, 4, ResizeBackend::Cpu, ResizeAlgorithm::Nearest);
        assert_eq!(resized.width(), 4);
        assert_eq!(resized.height(), 4);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_resize_gray_bilinear_unimplemented() {
        let img = Image::gray(2, 2, vec![10, 20, 30, 40]);
        let _ = resize(&img, 4, 4, ResizeBackend::Cpu, ResizeAlgorithm::Bilinear);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_resize_gray_bicubic_unimplemented() {
        let img = Image::gray(2, 2, vec![10, 20, 30, 40]);
        let _ = resize(&img, 4, 4, ResizeBackend::Cpu, ResizeAlgorithm::Bicubic);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_resize_simd_unimplemented() {
        let img = Image::gray(2, 2, vec![10, 20, 30, 40]);
        let _ = resize(&img, 4, 4, ResizeBackend::Simd, ResizeAlgorithm::Nearest);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_resize_gpu_unimplemented() {
        let img = Image::gray(2, 2, vec![10, 20, 30, 40]);
        let _ = resize(&img, 4, 4, ResizeBackend::Gpu, ResizeAlgorithm::Nearest);
    }
}
