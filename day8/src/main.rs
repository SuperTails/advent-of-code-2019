use std::path::Path;

fn parse(path: &Path) -> Vec<u32> {
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn layers(data: &[u32]) -> Vec<Vec<u32>> {
    let chunks = data.chunks_exact(25 * 6);
    assert_eq!(chunks.remainder().len(), 0);

    chunks
        .map(|c| {
            c.iter().copied().collect()
        }).collect()
}

fn flatten(data: &[Vec<u32>]) -> Vec<u32> {
    let mut result = Vec::new();

    for pixel_idx in 0..(25 * 6) {
        let pixels = data.iter().rev().map(|l| l[pixel_idx]);
        let mut pixel = 0;

        for image_pixel in pixels {
            if image_pixel == 0 {
                pixel = 0;
            } else if image_pixel == 1 {
                pixel = 1;
            }
        }

        result.push(pixel);
    }

    result
}

fn main() {
    let image = parse(Path::new("./data.txt"));
    let layers = layers(&image);

    let min_zeros = layers.iter().min_by_key(|l| l.iter().filter(|c| **c == 0).count()).unwrap();

    let checksum = min_zeros.iter().filter(|c| **c == 1).count() * min_zeros.iter().filter(|c| **c == 2).count();

    println!("Checksum: {:?}", checksum);

    let flattened = flatten(&layers);

    for r in 0..6 {
        for c in 0..25 {
            if flattened[r * 25 + c] == 0 {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!("");
    }
}
