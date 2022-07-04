//#[derive(Debug)]
// use std::io;
// use std::{thread, time};

fn main() {
    let TWIDTH: i32 = 120;
    let THIGHT: i32 = 30;
    // let TTOTAL: i32 = TWIDTH*THIGHT;

    let PIXRADIUS: i32 = 20;
    let FRADIUS: f64 = PIXRADIUS as f64 / THIGHT as f64;
    let gradient = " .:!/r(l1Z4H9W8$@";
    let gradient_size = gradient.len() - 1;

    let mut screen: [char; 3600] = [' '; 3600];
    let aspect: f64 = TWIDTH as f64 / THIGHT as f64;
    let pixel_aspect = 8.0 / 18.0;
    
    

    for t in 0..10000 {
        for w in 0..TWIDTH{
            for h in 0..THIGHT{
                let mut uv = vec2{
                    x: (w as f64/TWIDTH as f64)*2.0 - 1.0,
                    y: (h as f64/THIGHT as f64)*2.0 - 1.0,
                };

                uv.x *= pixel_aspect*aspect;
                uv.x += ((t as f64)*0.002).sin();

                let light = norm_vec3( vec3 {
                    x: -0.8,
                    y: 0.0 + ((t as f64)*0.002).sin(),
                    z: -0.5,
                });

                // Camera position
                let ro = vec3 {
                    x: -2.0,
                    y: 0.0,
                    z: 0.0,
                };
                // 
                let rd = norm_vec3(vec3 {
                    x: 1.0,
                    y: uv.x,
                    z: uv.y,
                });

                // let rad = (uv.x*uv.x + uv.y*uv.y).sqrt();
                //if rad < FRADIUS {
                let intersection = sphere(&ro, &rd, 1.0);
                let rad = intersection.x;
                if  rad > 0.0 {
                    let itPoint = vec3 {
                        x: ro.x + rd.x*rad,
                        y: ro.y + rd.y*rad,
                        z: ro.z + rd.z*rad,
                    };
                    let n = norm_vec3(itPoint);
                    let diff: f64 = dot(&n, &light);
                    let brightness = diff*20.0;
                    //let brightness = gradient_size as f64 - (gradient_size as f64* rad * (1.0/FRADIUS as f64));
                    let gradient_pos = brightness as usize;
                    let pixel = gradient.chars().nth(clamp(gradient_pos, 0, gradient_size)).unwrap();
                    screen[(w+h*TWIDTH) as usize] = pixel;//'@';
                }
                else { screen[(w+h*TWIDTH) as usize] = ' '; };
            };
            
        };
        let s: String = screen.iter().collect();
        print!("{}", s);
    };
}

fn clamp(val:usize, min:usize, max:usize) -> usize {
    if val < min { return min as usize; }
    else if val > max { return max as usize; }
    val as usize
}
fn vec2_len(v: &vec2) -> f64 {
    (v.x*v.x + v.y*v.y).sqrt()
}
fn vec3_len(v: &vec3) -> f64 {
    (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
}
fn norm_vec2(v: vec2) -> vec2 {
    let length = vec2_len(&v);
    vec2 {
        x: v.x / length,
        y: v.y / length,
    }
}
fn norm_vec3(v: vec3) -> vec3 {
    let length = vec3_len(&v);
    vec3 {
        x: v.x / length,
        y: v.y / length,
        z: v.z / length,
    }
}

fn dot(v1: &vec3, v2: &vec3) -> f64 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
} 

fn sphere(ro: &vec3, rd: &vec3, r: f64) -> vec2 {
    let b: f64 = dot(&rd, &ro);
    let c: f64 = dot(&ro, &ro) - r*r;
    let h: f64 = b*b - c;
    if h < 0.0 {
        vec2 {
            x: -1.0,
            y: -1.0,
        }
    }
    else {
        vec2 {
            x: -b-h,
            y: -b+h,
        }
    }
}

struct vec2 {
    x: f64,
    y: f64,
}
impl vec2 {
    fn sum(&self, other: &vec2) -> vec2 {
        vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn dif(&self, other: &vec2) -> vec2 {
        vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn mul(&self, other: &vec2) -> vec2 {
        vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
    fn div(&self, other: &vec2) -> vec2 {
        vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

struct vec3 {
    x: f64,
    y: f64,
    z: f64,
}
impl vec3 {
    fn sum(&self, other: &vec3) -> vec3 {
        vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    fn dif(&self, other: &vec3) -> vec3 {
        vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
    fn mul(&self, other: &vec3) -> vec3 {
        vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
    fn div(&self, other: &vec3) -> vec3 {
        vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}