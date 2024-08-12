mod consts;

mod methods;
mod uptime;

use crate::consts::InfoPanel;

fn main() {
    let info = InfoPanel::new();
    println!("{:#?}", info);
}
