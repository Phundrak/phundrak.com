use actix_web::{error, get, web, Responder, Result};
use derive_more::{Display, Error};
use dotenvy::dotenv;
use gql_client::Client;
use std::collections::HashMap;

mod data;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: String,
}

struct AppState {
    github_token: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            github_token: github_token(),
        }
    }
}

impl error::ResponseError for MyError {}

fn github_token() -> String {
    std::env::var("GH_TOKEN")
        .expect("Environment variable GH_TOKEN **MUST** be set!")
}

#[get("/phundrak-com/{user}")]
async fn user_info_github(
    user: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let gh_token = &state.github_token;
    let body = format!(
        r#"query {{
  user(login: "{}") {{
    newest: repositories(first: 10, orderBy: {{field: UPDATED_AT, direction: DESC}}) {{
      nodes {{
        name
        stargazerCount
        forkCount
      }}
    }}
    mostStarred: repositories(first: 10, orderBy: {{field: STARGAZERS, direction: DESC}}) {{
      nodes {{
        name
        stargazerCount
        forkCount
      }}
    }}
    pinned: pinnedItems(first: 10) {{
      nodes {{
        ... on Repository {{
          name
          stargazerCount
          forkCount
        }}
      }}
    }}
  }}
}}"#,
        user
    );
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", gh_token));
    headers.insert("User-Agent".to_string(), "PhunCache-App".to_string());
    let client =
        Client::new_with_headers("https://api.github.com/graphql", headers);
    client
        .query::<data::Data>(&body)
        .await
        .map(web::Json)
        .map_err(|e| {
            MyError {
                name: format!("Failed to retrieve data from GitHub: {e:?}"),
            }
            .into()
        })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use std::env::var;

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    human_panic::setup_panic!();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState::default()))
            .service(user_info_github)
    })
    .bind((
        var("ACTIX_ADDRESS")
            .expect("Environment variable ACTIX_ADDRESS must be set!"),
        var("ACTIX_PORT")
            .expect("Environment variable ACTIX_PORT must be set!")
            .parse()
            .expect("Failed to parse value of ACTIX_PORT into a u16"),
    ))?
    .run()
    .await
}
