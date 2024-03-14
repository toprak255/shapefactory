

use image::{ImageBuffer, Rgba};
use rand::prelude::*;
use std::env;

mod stuff;
use stuff::*;

fn draw(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, shape:&mut Shape){
    let bg= shape.background_color;
    let fg;
    if shape.random_color{
        let mut rng =rand::thread_rng();
        let r: u8 = rng.gen(); 
        let g: u8 = rng.gen(); 
        let b: u8 = rng.gen(); 
        fg=Rgba([r,g,b,255]);
    }
    else{
        fg = shape.foreground_color;
    }
    
    let base_size = get_area(&shape.vertices, &shape.origin);
    //let range = &shape.radius;
    for (x, y, pixel) in img.enumerate_pixels_mut() {

        let normalized_position = normalize(&x, &y,&shape);
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
fn save_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, folder_path: Option<&String>, filename: &str) {
    let path=format!("{}/{}", folder_path.unwrap(), filename);
    let _ = img.save(&path).expect("Failed to save image");
}

pub fn main() {
    let help_text:&str="usage: shapefactory.exe [-corner <corner count>] [-Width <canvas-width>] [-Height <canvas-height>] [-scale <value>] 
                        [-random-scale] [-random-rotation] [-f <folderPath>] [-n <filename>] 
                        [-count <number of images>] [-random-color]
                        
                        providing filename and corner count is necassary, other values have defaults that can optionally be changed
                        ";
    let args: Vec<String> = env::args().collect();
    if args.len()<=1{
        println!("{}",help_text);

        std::process::exit(0);
    }

    let mut shape =Shape::new();
    let mut img_count:u32 = 0;
    let mut base_filename =String::new();
    let mut folder_path =String::from(".");
    for i in 1..args.len(){
        match args[i].as_str(){
            //"-h" => {println!("{}",help_text);std::process::exit(0);}
            "-corner"=> {shape.corner_count=args[i+1].parse::<u32>().unwrap();}
            "-Width" =>{shape.width=args[i+1].parse::<u32>().unwrap();}
            "-Height"=>{shape.height=args[i+1].parse::<u32>().unwrap();}
            "-scale" => {shape.random_scale=false;  shape.radius=args[i+1].parse::<f32>().unwrap();}
            "-random-scale" => {shape.random_scale=true;}
            "-random-rotation" => {shape.random_rotation=true;}
            "-random-color" => {shape.random_color=true}
            "-count"=>{img_count=args[i+1].parse::<u32>().unwrap();}
            "-fg"=>{}
            "-bg"=>{}
            "-f" => {folder_path=String::from(&args[i+1])}
            "-n" => {base_filename=String::from(&args[i+1])}
            
            _ => {}
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
        save_image(&img,Some(&folder_path), &filename);
    }

}
