#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate solana_sdk;

use rocket::response::content;
use rocket::http::ContentType;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::solana_client::SolanaClient;
use solana_sdk::token_client::TokenClient;

#[derive(Serialize, Deserialize, Debug)]
struct Selfie {
    data: String,
}

#[post("/selfie", data = "<selfie>")]
fn mint_selfie(selfie: rocket_contrib::json::Json<Selfie>, signer: Keypair) -> content::Json<String> {
    let selfie = selfie.into_inner();

    let client = SolanaClient::new("https://devnet.solana.com");
    let token_client = TokenClient::new(&client);

    let result = match token_client
        .create_token(
            &signer,
            "SELFIE".to_string(),
            0,
            vec![(1, selfie.data.as_bytes().to_vec())],
        )
    {
        Ok(result) => result,
        Err(e) => return content::Json(format!("Token creation failed: {:?}", e)),
    };

    content::Json(format!("Token minted: {:?}", result))
}

fn main() {
    rocket::ignite().mount("/", routes![mint_selfie]).launch();
}
