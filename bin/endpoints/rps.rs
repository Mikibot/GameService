use games::rps as rps_game;
use rocket_contrib::Json;

use self::rps_game::Response;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/<weapon>/<bet>")]
fn rps(weapon: String, bet: u64) -> Json<Response> {
    Json(rps_game::rps(bet, &weapon))
}
