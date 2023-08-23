use imageproc::{filter::box_filter, window};
use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{GrayImage, ImageBuffer};

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<String> = std::env::args().collect();
    let radiusxy = args[2].parse::<u32>()?;
    let img = ImageReader::open(&args[1])?.decode()?;

    let gray_img = img.to_luma8();

    let filtered = box_filter(&gray_img, radiusxy,radiusxy);
    filtered.save("filtered.png")?;


    let raw = from_raw()?;
    let raw_filt = box_filter(&raw, radiusxy,radiusxy);
    let v = raw_filt.to_vec();    
    println!("the box filtered vector: {:?}",v);

    Ok(())
}

fn from_raw() -> Result<GrayImage, Box<dyn std::error::Error>>{
    let width = 10;
    let height = 10;

    // let raw_data: Vec<u8> = vec![/* RGB values */; width * height * 3];
    let raw_data: Vec<u8> =    vec![1,2,3,4,5,6,7,8,9,10,
                                    11,12,13,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,255,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10,
                                    1,2,3,4,5,6,7,8,9,10];
    //; (width * height) as usize];
    println!("the len of vec is: {}", raw_data.len());

    Ok(ImageBuffer::from_vec(width as u32, height as u32, raw_data).unwrap())
    //Ok(result)

}