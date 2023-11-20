mod utils;

use worker::*;

#[event(start)]
pub fn start() {
    utils::set_panic_hook();
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok("Hello, World!")
}
