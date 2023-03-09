pub fn calculate_print_time(gcode_data: &String) {
    println!("Start calculating print time");
    let mut counter = 0;
    for line in gcode_data.lines() {
        if line.len() != 0 {
            let top_char = line.chars().nth(0).unwrap();
            //Commnet OUt
            if top_char == ';' {
                //println!("[Line{}] CommentOut", counter);
                continue;
            }

            let g: Vec<&str> = line.split(' ').collect();
            //println!("{}", g[0]);
            match g[0]{
                "G1" => {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut z = 0.0;
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
                        f =  _f.unwrap().replace("F", "").parse::<f32>().unwrap();
                    }

                    println!("[Line{}] Move X{} Y{} Z{} Extrude E{} Speed F{}", counter, x, y, z, e, f);
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

                    println!("[Line{}] Rotate X{} Y{} I{} J{} Extrude E{} Speed F{}", counter, x, y, i, j, e, f);
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
                    println!("[Line{}] Wait P{} S{}", counter, p, s);

                },
                _ => {
                }
            }

            counter += 1;
        }
    }

}