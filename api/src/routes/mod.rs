use afire::Server;

use crate::app::App;

mod subscribe;

pub fn attach(server: &mut Server<App>) {
    subscribe::attach(server);
}
