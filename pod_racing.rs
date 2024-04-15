use std::{io, ops::{Add, Sub, Mul}};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}
#[derive(Copy, Clone)]
struct Pos {
    x:i32,
    y:i32,
}

impl Add for Pos{
    type Output = Pos;
    fn add(self, other:Pos) -> Pos{
        Pos{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

}
impl Sub for Pos{
    type Output = Pos;
    fn sub(self, other:Pos) -> Pos{
        Pos{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

}
impl Mul<i32> for Pos{
    type Output = Pos;
    fn mul(self, f:i32) -> Pos{
        Pos{
            x: self.x * f,
            y: self.y * f,
        }
    }

}


/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/


fn calculate_thrust(angle:i32,distance:i32) -> i32 {
    let thrust = 100;
    let ratio_angle =  angle.abs() as f32 / 90.0;
    let factor_angle = (1.0 - ratio_angle).min(1.0).max(0.0) ;
    let factor_distance = (distance as f32 / 1200.0).min(1.0).max(0.0);
    (thrust as f32 * factor_angle * factor_distance) as i32

}

fn main() {
    let mut boost_available = true;
    let mut prev_pos = Pos{
        x: -1,
        y: -1,
    };
    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let next_checkpoint_x = parse_input!(inputs[2], i32); // x position of the next check point
        let next_checkpoint_y = parse_input!(inputs[3], i32); // y position of the next check point
        let next_checkpoint_dist = parse_input!(inputs[4], i32); // distance to the next checkpoint
        let next_checkpoint_angle = parse_input!(inputs[5], i32); // angle between your pod orientation and the direction of the next checkpoint
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let opponent_x = parse_input!(inputs[0], i32);
        let opponent_y = parse_input!(inputs[1], i32);

        let my_pos = Pos{
            x: x,
            y: y,
        };

        let next_cp = Pos{
            x: next_checkpoint_x,
            y: next_checkpoint_y,
        };

        if prev_pos.x < 0 {
            prev_pos = my_pos;
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        eprintln!("Distance to next checkpoint: {}", next_checkpoint_dist);

        let thrust = calculate_thrust(next_checkpoint_angle,next_checkpoint_dist);

        let diff = my_pos - prev_pos;
        
        let target = next_cp - (diff * 3); 

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"

        prev_pos = my_pos;
        if boost_available && next_checkpoint_angle == 0 && next_checkpoint_dist > 5000{
            println!("{} {} {}", target.x , target.y , "BOOST");
            boost_available = false;
        }else{
            println!("{} {} {}", target.x , target.y, thrust);
        }
    }
}


