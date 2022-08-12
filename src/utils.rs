use std::fmt::Display;

pub fn vec_to_image<T>(vector: &mut Vec<T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut image: Vec<Vec<T>> = Vec::new();
    let dimension = (vector.len() as f32).sqrt().abs();
    assert_eq!(dimension, dimension.abs());
    for i in 0..dimension as usize {
        image.push(Vec::new());
        for j in 0..dimension as usize {
            image[i].push(vector[i * dimension as usize + j]);
        }
    }
    return image;
}
pub fn rotate_image<T>(image: &mut Vec<Vec<T>>)
where
    T: Copy,
    T: Display,
{
    assert_eq!(image.len(), image[0].len());

    let image_len = image.len();
    for i in 0..image_len {
        for j in i..image_len {
            (image[i][j], image[j][i]) = (image[j][i], image[i][j]);
        }
    }

    for i in 0..image_len {
        for j in 0..image_len / 2 {
            (image[i][j], image[i][image_len - 1 - j]) = (image[i][image_len - 1 - j], image[i][j])
        }
    }
}
