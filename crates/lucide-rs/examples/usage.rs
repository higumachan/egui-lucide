use lucide_rs::Color;

fn main() {
    let activity = lucide_rs::activity().color(Color::red());

    println!("{}", activity.svg());
}
