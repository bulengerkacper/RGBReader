extern crate image;

fn main() {
    let im = image::open("C:\\Users\\Hyperbook\\Desktop\\nosa.jpg").unwrap();
    let photo = im.to_rgba8();
    let mut r_sum = 0;
    let mut g_sum = 0;
    let mut b_sum = 0;
    for px in photo.pixels() {
        r_sum = r_sum + px[0] as u32;
        g_sum = g_sum + px[1] as u32;
        b_sum = b_sum + px[2] as u32;
    }
    println!("{} {} {}", r_sum, g_sum, b_sum);
}

