use std::process::Command;
use std::thread;
use std::time;
use rand::Rng;

fn main() {
    message("YOUR MESSAGE HERE");
}

fn message(Message: &str) { //If changed manually remove Message: &str
    let spin_all = false;
    let target_message = Message; //Or change manually
    let duration = 1.0; 
    let steps = 30; 
    let delay = duration / steps as f64;
    let qwerty = "!qwertyui@opasdfghjk#lzxcvbnm$%^&*()"; //Key range
    let mut rng = rand::thread_rng();
    let mut displayed_message: Vec<char> = vec![' '; target_message.len()]; 
    //Dont touch anything beneath here
    if spin_all {
        for _ in 0..steps {
            for i in 0..target_message.len() {
                let idx = rng.gen_range(0..qwerty.len()); 
                let spinning_letter = qwerty.chars().nth(idx).unwrap();
                displayed_message[i] = spinning_letter; 
            }
            Command::new("clear").status().expect("Failed to clear");
            println!("{}", displayed_message.iter().collect::<String>());
            thread::sleep(time::Duration::from_millis((delay * 1000.0) as u64));
        }
        displayed_message.iter_mut().enumerate().for_each(|(i, ch)| {
            *ch = target_message.chars().nth(i).unwrap(); 
        });
    } else {
        for (i, target_letter) in target_message.chars().enumerate() {
            for _ in 0..steps {
                let idx = rng.gen_range(0..qwerty.len());
                let spinning_letter = qwerty.chars().nth(idx).unwrap();
                displayed_message[i] = spinning_letter; 
                
                Command::new("clear").status().expect("Failed to clear");
                println!("{}", displayed_message.iter().collect::<String>());
                thread::sleep(time::Duration::from_millis((delay * 1000.0) as u64));
            }
            displayed_message[i] = target_letter; 
            Command::new("clear").status().expect("Failed to clear");
            println!("{}", displayed_message.iter().collect::<String>());
            thread::sleep(time::Duration::from_millis(500)); 
        }
    }
    Command::new("clear").status().expect("Failed to clear");
    println!("{}", displayed_message.iter().collect::<String>());
}
