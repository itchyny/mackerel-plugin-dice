extern crate mackerel_plugin;
extern crate mackerel_plugin_dice;

use mackerel_plugin::Plugin;
use mackerel_plugin_dice::DicePlugin;

fn main() {
    let plugin = DicePlugin {};
    match plugin.run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("mackerel-plugin-dice: {}", err);
            std::process::exit(1);
        }
    }
}
