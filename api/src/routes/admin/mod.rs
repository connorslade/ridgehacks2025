use afire::Server;

use crate::app::App;

mod publish;

pub fn attach(server: &mut Server<App>) {
    publish::attach(server);
}
