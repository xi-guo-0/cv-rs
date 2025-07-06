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

pub fn sobel_edge_detection(img: &Image) -> Image {
    let (width, height, data) = match img {
        Image::Gray {
            width,
            height,
            data,
        } => (*width, *height, data),
        _ => panic!("Sobel only implemented for grayscale images"),
    };
    let gx = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let gy = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];
    let mut out = vec![0u8; width * height];

    for y in 0..height {
        for x in 0..width {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for ky in 0..3 {
                for kx in 0..3 {
                    let ix = x as isize + kx as isize - 1;
                    let iy = y as isize + ky as isize - 1;
                    if 0 <= ix && ix < width as isize && 0 <= iy && iy < height as isize {
                        let idx = iy as usize * width + ix as usize;
                        sx += data[idx] as f32 * gx[ky * 3 + kx];
                        sy += data[idx] as f32 * gy[ky * 3 + kx];
                    }
                }
            }
            let mag = sx.abs() + sy.abs();
            out[y * width + x] = mag.min(255.0) as u8;
        }
    }

    Image::gray(width, height, out)
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

    #[test]
    fn test_sobel_edge_detection_on_simple_image() {
        let input = Image::gray(3, 3, vec![0, 0, 0, 0, 255, 255, 0, 0, 0]);
        let output = sobel_edge_detection(&input);

        if let Image::Gray { data, .. } = output {
            assert!(data[4] > 200, "Center pixel expected to have strong edge");
        } else {
            assert!(false, "Output image is not grayscale");
        }
    }
}
