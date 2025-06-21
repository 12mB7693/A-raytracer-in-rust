fn main() {
    let nx = 200;
    let ny = 100;
    //let first_line = "P3\n" + nx.to_string();
    //println!(first_line);
    let header = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    println!("{}", header);
    for j in (0..ny-1).rev()
    {
        for i in 0..nx
        {
            let color = Vec3(i as f64/nx as f64, j as f64/ny as f64, 0.2);
            let ir = (255.99*color.r()) as u8;
            let ig = (255.99*color.g()) as u8;
            let ib = (255.99*color.b()) as u8;
            println!("{}", format!("{} {} {}\n", ir, ig, ib));
        }
    }
    
}

struct Vec3(f64, f64, f64);

impl Vec3 {
     fn x(&self) -> f64 {
         self.0
     }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn r(&self) -> f64 {
        self.0
    }
    fn g(&self) -> f64 {
        self.1
    }
    fn b(&self) -> f64 {
        self.2
    }
}
