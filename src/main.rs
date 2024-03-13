

use image::{ImageBuffer, Rgba};


use std::env;

mod stuff;
use stuff::*;
// fn draw_disc(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, center: &Vec2, radius: f32) {
//     for (x, y, pixel) in img.enumerate_pixels_mut() {
//         let normalized_position = normalize(&x, &y);
//         if get_distance(&normalized_position, &center) < radius {
//             *pixel = Rgba([255, 255, 255, 255]);
//         } else {
//             *pixel = Rgba([60, 60, 60, 255]);
//         }
//     }
// }

//add background color input
fn draw(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, shape:&mut Shape){
    let fg = shape.foreground_color;
    let bg= shape.background_color;
    
    let base_size = get_area(&shape.vertices, &shape.origin);
    //let range = &shape.radius;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let normalized_position = normalize(&x, &y);
        if get_distance(&normalized_position, &shape.origin) < shape.radius{
            if get_area(&shape.vertices, &normalized_position) <= base_size+0.000001{
                *pixel = fg;
            }
            else {
                *pixel = bg;
            }
        }
        else {
            *pixel = bg;
        }
    }

}
fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &str) {
    let _ = img.save(&filename).expect("Failed to save image");
}


pub fn main() {
    let help_text:&str=("usage: shapefactory.exe [-h] [-c <corner count>] [-W <canvas-width>] [-H <canvas-height>] [-scale <value>] 
                        a[-random-scale] [-random-rotation] [-fg <hex-32bit>] [-bg <hex-32bit>] [-n <filename>]");
    let args: Vec<String> = env::args().collect();
    if args.len()<=1{
        println!("{}",help_text);
        std::process::exit(0);
    }

    let mut shape =Shape::new();
    let mut img_count:u32 = 0;
    let mut base_filename =String::new();
    for i in 1..args.len(){
        if args.len()<i+1{
            println!("a possible flag has been entered but no value has been given");
            std::process::exit(0);
        }
        else{}
        match args[i].as_str(){
            "-h" => {println!("{}",help_text);std::process::exit(0);}
            "-c"=> {shape.corner_count=args[i+1].parse::<u32>().unwrap();}
            "-W" =>{shape.width=args[i+1].parse::<u32>().unwrap();}
            "-H"=>{shape.height=args[i+1].parse::<u32>().unwrap();}
            "-scale" => {shape.random_scale=false;  shape.radius=args[i+1].parse::<f32>().unwrap();}
            "-random-scale" => {shape.random_scale=true;}
            "-random-rotation" => {shape.random_rotation=true;}
            "-count"=>{img_count=args[i+1].parse::<u32>().unwrap();}
            "-fg"=>{
                let val =&args[i+1];
                let color:String;
                if val.starts_with("0x"){
                    if let Some(stripped_val) = val.strip_prefix("0x") {
                        color = stripped_val.to_string();
                }
                else {color=val.to_string()};
                if let Some(rgba_color) = to_rgb(color.as_str()){
                shape.foreground_color=rgba_color;}
                }}
            "-bg"=>{
                let val =&args[i+1];
                let color:String;
                if val.starts_with("0x"){
                    if let Some(stripped_val) = val.strip_prefix("0x") {
                        color = stripped_val.to_string();
                }
                else {color=val.to_string()};
                if let Some(rgba_color) = to_rgb(color.as_str()){
                shape.background_color=rgba_color;}
                }}
            "-n" => {base_filename=String::from(&args[i+1])}
            

            _ => {println!("{}",help_text);std::process::exit(0);}
        }
    }
    if shape.corner_count <=2{
        println!("Corner count must be higher than 2");
        std::process::exit(0);
    }
    if img_count <1{
        println!("Image count is not set or too low use [-count <number>] ");
        std::process::exit(0);
    }
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(shape.width, shape.height);
    for i in 0..img_count{
        shape.set_shape();
        draw(&mut img,&mut shape);
        let filename = format!("{}{}.png", base_filename, i);
        save_image(&img, &filename);
    }

}
