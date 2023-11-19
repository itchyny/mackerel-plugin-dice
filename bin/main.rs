use mackerel_plugin::Plugin;
use mackerel_plugin_dice::DicePlugin;

fn main() {
    if let Err(err) = (DicePlugin {}).run() {
        eprintln!("mackerel-plugin-dice: {}", err);
        std::process::exit(1);
    }
}
