
use rand::prelude::*;

//use image::{ImageBuffer, Rgba};
use image::Rgba;

pub const PI: f32 = 3.14159265358979323846264338327950288_f32;

pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Clone for Vec2 {
    fn clone(&self) -> Self {
        Vec2 { x: self.x, y: self.y }
    }
}

pub struct Shape {
    pub width:u32,
    pub height:u32,
    pub corner_count:u32,
    pub vertices: Vec<Vec2>,
    pub origin: Vec2,
    pub radius:f32,
    pub random_rotation:bool,
    pub random_scale:bool,
    pub random_color:bool,
    pub foreground_color:Rgba<u8>,
    pub background_color:Rgba<u8>,
}

impl Shape {
    //pub fn new(&mut self,corner_count:u16,rotation:Option<f32>) -> Self { 
        pub fn new() -> Self { 
        Self {
            width:1001,
            height:1001,
            corner_count:0,
            vertices:Vec::new(),
            origin:Vec2::new(0.,0.),
            radius:1.,
            random_rotation:false,
            random_scale:false,
            random_color:false,
            foreground_color:Rgba([255,255,255,255]),
            background_color:Rgba([0,0,0,255]),
        }
        
    }
    
    fn rotate(&mut self,vertices:&mut Vec<Vec2>, rad: f32) {  

        for dot in vertices {

            let new_x = dot.x * f32::cos(rad) - dot.y * f32::sin(rad);
            let new_y = dot.y * f32::cos(rad) + dot.x * f32::sin(rad);
    
            dot.x = new_x;
            dot.y = new_y;
        }

   }
   
    pub fn set_shape(&mut self){
        self.vertices.clear();
        //if (random_scale.is_some() || radius.is_some()) && (random_rotation.is_some() || rotation.is_some()){
        
        //let mut vertices:Vec<Vec2>=Vec::new();
        let angle_offset = 2.*PI/self.corner_count as f32;
        
        
        if self.random_scale{
            let mut rng =rand::thread_rng();
            let y: f32 = rng.gen(); 
            self.radius= (y+1.) /2.;
        }
        for i in 1..self.corner_count+1{
           self.vertices.push(Vec2::new(f32::cos(i as f32 *angle_offset)*self.radius ,f32::sin(i as f32 *angle_offset)*self.radius));
        }
     
        if self.random_rotation{
            let mut rng = rand::thread_rng();
            let y: f32 = rng.gen(); 
            let mut clone = self.vertices.clone();
            self.rotate(&mut clone,y*PI*2.);
            self.vertices=clone;
        }
        }
    }


pub fn get_distance(point1: &Vec2, point2: &Vec2) -> f32 {
    f32::sqrt(f32::powf(point1.x - point2.x, 2.) + f32::powf(point1.y - point2.y, 2.))
}

pub fn normalize(x: &u32, y: &u32 ,shape :&Shape) -> Vec2 {
    let normalized_value = Vec2 {
        x: *x as f32 / shape.width as f32 * 2. - 1.,
        y: *y as f32 / shape.height as f32 * 2. - 1.,
    };
    normalized_value
}


pub fn get_area(vertices:&Vec<Vec2>,center:&Vec2) -> f32{
    let size =vertices.len();
    let p1= center;
    let mut val:f32 = 0.;
    
    for i in 0..size{
        let p2:&Vec2 = &vertices[i];
        let p3:&Vec2 = &vertices[(i+1) % size ];
        val +=f32::abs( (p1.x * (p2.y-p3.y) + p2.x * (p3.y-p1.y) + p3.x *(p1.y-p2.y) ) / 2.0 );
    }

    val
}
