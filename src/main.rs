use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Dora — Your Daily Quest Generator ===");
    println!();

    loop {
        let name = ask("Name: ");
        let mood = ask("Mood (happy, tired, bored, stressed...): ");
        let time_input = ask("Free time in minutes: ");

        let free_time: u32 = match time_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("(Couldn't read that as a number, using 15)");
                15
            }
        };

        let quest = pick_quest(&mood, free_time);
        let reward = pick_reward(free_time);

        println!();
        println!("Name: {}", name);
        println!("Mood: {}", mood);
        println!("Free time: {} minutes", free_time);
        println!();
        println!("Your quest:");
        println!("{}", quest);
        println!();
        println!("Reward:");
        println!("{}", reward);
        println!();

        let start = ask("Start the quest timer? (y/n): ");
        if start.to_lowercase().trim() == "y" {
            run_timer(free_time);
        }

        let again = ask("Generate another quest? (y/n): ");
        if again.to_lowercase().trim() != "y" {
            println!("Goodbye, brave adventurer!");
            break;
        }
        println!();
    }
}

fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn pick_quest(mood: &str, free_time: u32) -> String {
    let quests = vec![
        "Drink a glass of water and stretch for 2 minutes.",
        "Clean one small thing near you, then rest for 5 minutes.",
        "Send a kind message to a friend you haven't talked to in a while.",
        "Look out the window for 60 seconds and notice 3 things.",
        "Take a 15-minute walk without your phone.",
        "Tidy your desk and write down 3 things you want to do this week.",
        "Cook or prepare a small snack from scratch.",
        "Read 5 pages of any book.",
        "Work on a personal project for 45 minutes with NO distractions.",
        "Write a one-page reflection on what you want to learn next.",
        "Learn one new thing in Rust and write a tiny example.",
    ];

    let index = (mood.len() as u32 + free_time) as usize % quests.len();
    let base = quests[index];

    let mood_line = match mood.to_lowercase().as_str() {
        "tired" => " (Go easy on yourself, hero. The couch is also a dungeon.)",
        "happy" => " (You're glowing today! Spread the vibes.)",
        "bored" => " (Boredom is just a quest waiting to start.)",
        "stressed" => " (Breathe. The dragon can wait 5 minutes.)",
        _ => " (Whatever you're feeling, the quest awaits.)",
    };

    format!("{}{}", base, mood_line)
}

fn run_timer(minutes: u32) {
    let mut remaining = minutes * 60;
    while remaining > 0 {
        let mm = remaining / 60;
        let ss = remaining % 60;
        print!("\rTime left: {:02}:{:02} ", mm, ss);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        remaining -= 1;
    }
    println!("\rTime's up! Quest complete.       ");
}

fn pick_reward(free_time: u32) -> String {
    if free_time >= 45 {
        String::from("+50 XP, +3 Willpower, and a rare loot drop")
    } else if free_time >= 15 {
        String::from("+25 XP and +2 Focus")
    } else {
        String::from("+10 XP and +1 Discipline")
    }
}
