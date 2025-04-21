fn main() {
    let nx = 100;
    let ny = 100;
    //let first_line = "P3\n" + nx.to_string();
    //println!(first_line);
    let header = format!("P3\n{} {}\n255\n", nx.to_string(), ny.to_string());
    println!("{}", header);
    for j in (0..ny-1).rev()
    {
        for i in 0..nx
        {
            let r = i as f64/nx as f64;
            let g = j as f64/ny as f64;
            let b = 0.2;
            //let color = Vec3{ e0: r, e1: g, e2: b };
            let ir = (255.99*r) as u8;
            let ig = (255.99*g) as u8;
            let ib = (255.99*b) as u8;
            println!("{}", format!("{} {} {}\n", ir, ig, ib));
        }
    }
    
}

// struct Vec3 {
//     e0: f64,
//     e1: f64,
//     e2: f64
// }

// impl Vec3 {
//     fn x(&self) -> f64 {
//         self.e0
//     }
//     fn y(&self) -> f64 {
//         self.e1
//     }
//     fn z(&self) -> f64 {
//         self.e2
//     }
//     fn r(&self) -> f64 {
//         self.e0
//     }
//     fn g(&self) -> f64 {
//         self.e1
//     }
//     fn b(&self) -> f64 {
//         self.e2
//     }
// }
