fn main() {
    let TWIDTH: i32 = 120;
    let THIGHT: i32 = 30;

    let PIXRADIUS: i32 = 20;
    let FRADIUS: f64 = PIXRADIUS as f64 / THIGHT as f64;
    let gradient = " .:!/r(l1Z4H9W8$@";
    let gradient_size = gradient.len() - 2;

    let mut screen: [char; 3600] = [' '; 3600];
    let aspect: f64 = TWIDTH as f64 / THIGHT as f64;
    let pixel_aspect = 8.0 / 18.0;
    
    for t in 0..10000 {
        // let mut new_draw = 1;
        let shift = (TWIDTH as f64/3.0)*((t as f64)*0.002).sin();
        let shift = shift as i32;

        for w in 0..TWIDTH{
            for h in 0..THIGHT{
                let mut x:f64 = ((w+shift) as f64/TWIDTH as f64)*2.0 - 1.0;
                let y:f64 = (h as f64/THIGHT as f64)*2.0 - 1.0;
                x = x*pixel_aspect*aspect;
                
                let rad = (x*x + y*y).sqrt();
                if rad < FRADIUS {
                    let brightness = gradient_size as f64 - (gradient_size as f64* rad * (1.0/FRADIUS as f64));
                    let pixel_pos = brightness as usize;
                    let pixel = gradient.chars().nth(clamp(pixel_pos, 0, gradient_size)).unwrap();
                    screen[(w+h*TWIDTH) as usize] = pixel;//'@';
                    // if new_draw>0 {
                    //   print!("{},\t,{},{},{},{},{}\n", rad, shift, w, h, x, y);
                    //   new_draw = 0;
                    // }
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