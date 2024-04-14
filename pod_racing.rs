use std::{f32::consts::PI, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let under = (x1 - x2).pow(2) + (y1 - y2).pow(2);
    f32::sqrt(under as f32) as i32
}

fn calculate_thrust(angle: i32, distance: i32) -> i32 {
    let mut thrust = 100;
    if angle < -90 || angle > 90 {
        thrust = 0;
    };
    if (distance < 1000) {
        thrust = 20;
    };
    thrust
}

fn main() {
    let mut boost_available = true;
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

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        eprintln!("Distance to next checkpoint: {}", next_checkpoint_dist);

        let thrust = calculate_thrust(next_checkpoint_angle, next_checkpoint_dist);

        let dist_enemy_to_checkpoint =
            distance(opponent_x, opponent_y, next_checkpoint_x, next_checkpoint_y);

        // You have to output the target position
        // followed by the power (0 <= thrust <= 100)
        // i.e.: "x y thrust"
        if next_checkpoint_dist > dist_enemy_to_checkpoint && boost_available && thrust == 100 {
            println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, "BOOST");
            boost_available = false;
        } else {
            println!("{} {} {}", next_checkpoint_x, next_checkpoint_y, thrust);
        }
    }
}
