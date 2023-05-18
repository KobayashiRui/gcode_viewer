use glam::{swizzles::*, Vec3, Vec3A};

pub fn calculate_print_time(gcode_data: &String) -> Result<f32, i32> {
    println!("Start calculating print time!");
    let mut counter = 0;
    let mut all_time: f32 = 0.0; //s
    let mut now_pos = Vec3::new(0.0, 0.0, 0.0); //mm
    let mut speed: f32 = 0.0; //speed: mm/s
    for line in gcode_data.lines() {
        //println!("Counter: {}", counter);
        counter += 1;
        if line.len() != 0 {
            //println!("c: {} = t:{}", counter, all_time);
            if all_time.is_nan() {
                println!("NAN!");
                println!("ERROR:{}", counter-1);
                return Err(counter-1)
            }
            let comment_pos = line.find(';').unwrap_or(line.len());
            let new_line = &line[..comment_pos];
            if new_line.len() == 0 {
                continue;
            }

            let g: Vec<&str> = new_line.split(' ').collect();
            //println!("Gcode: {}", g[0]);
            match g[0]{
                "G1" => {
                    let mut x = now_pos.x;
                    let mut y = now_pos.y;
                    let mut z = now_pos.z;
                    //let mut f = 0.0;
                    let mut e = 0.0;

                    let _x = g.iter().find(|s| s.contains("X"));
                    if _x != None {
                        x =  _x.unwrap().replace("X", "").parse::<f32>().unwrap();
                    }
                    let _y = g.iter().find(|s| s.contains("Y"));
                    if _y != None {
                        y =  _y.unwrap().replace("Y", "").parse::<f32>().unwrap();
                    }
                    let _z = g.iter().find(|s| s.contains("Z"));
                    if _z != None {
                        z =  _z.unwrap().replace("Z", "").parse::<f32>().unwrap();
                    }

                    let _e = g.iter().find(|s| s.contains("E"));
                    if _e != None {
                        e =  _e.unwrap().replace("E", "").parse::<f32>().unwrap();
                    }

                    let _f = g.iter().find(|s| s.contains("F"));
                    if _f != None {
                        let f =  _f.unwrap().replace("F", "").parse::<f32>().unwrap(); // [mm/min]
                        speed = f / 60.0; // [mm/s]
                    }

                    //println!("[Line{}] Move X{} Y{} Z{} Extrude E{} Speed F{}", counter, x, y, z, e, speed);

                    if x==now_pos.x && y==now_pos.y && z==now_pos.z && e == 0.0{
                        continue;
                    }

                    //移動量の計算
                    //xyzの移動がなく、eの量がある
                    let target_vec = Vec3::new(x, y, z);
                    let move_vec = target_vec - now_pos;
                    let move_length = move_vec.length();

                    if x==now_pos.x && y==now_pos.y && z==now_pos.z && e != 0.0{
                        let time = e / speed; // time sec
                        all_time += time;
                    }else{
                        let time = move_length / speed; // time sec
                        all_time += time;
                        now_pos = target_vec;
                    }
                },
                "G2" => {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut i = 0.0;
                    let mut j = 0.0;
                    let mut f = 0.0;
                    let mut e = 0.0;

                    let _x = g.iter().find(|s| s.contains("X"));
                    if _x != None {
                        x =  _x.unwrap().replace("X", "").parse::<f32>().unwrap();
                    }
                    let _y = g.iter().find(|s| s.contains("Y"));
                    if _y != None {
                        y =  _y.unwrap().replace("Y", "").parse::<f32>().unwrap();
                    }
                    let _i = g.iter().find(|s| s.contains("I"));
                    if _i != None {
                        i =  _i.unwrap().replace("I", "").parse::<f32>().unwrap();
                    }
                    let _j = g.iter().find(|s| s.contains("J"));
                    if _j != None {
                        j =  _j.unwrap().replace("J", "").parse::<f32>().unwrap();
                    }

                    let _f = g.iter().find(|s| s.contains("F"));
                    if _f != None {
                        f =  _f.unwrap().replace("F", "").parse::<f32>().unwrap();
                    }

                    let _e = g.iter().find(|s| s.contains("E"));
                    if _e != None {
                        e =  _e.unwrap().replace("E", "").parse::<f32>().unwrap();
                    }
                    //println!("[Line{}] Rotate X{} Y{} I{} J{} Extrude E{} Speed F{}", counter, x, y, i, j, e, f);
                    //all_time += 1.0;
                },
                "G4" => {
                    let mut p = 0.0;
                    let mut s = 0.0;

                    let _p = g.iter().find(|s| s.contains("P"));
                    if _p != None {
                        p =  _p.unwrap().replace("P", "").parse::<f32>().unwrap();
                    }
                    let _s = g.iter().find(|s| s.contains("S"));
                    if _s != None {
                        s =  _s.unwrap().replace("S", "").parse::<f32>().unwrap();
                    }
                    //println!("[Line{}] Wait P{} S{}", counter, p, s);

                    let time = (p*0.001) + s; // time sec
                    all_time += time;
                },
                _ => {
                    //println!("No Process {}", g[0]);
                }
            }

        }
    }

    println!("ALL Time {}[s]", all_time);
    return Ok(all_time)
}