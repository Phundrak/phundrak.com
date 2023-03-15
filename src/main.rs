use actix_web::{error, get, web, Responder, Result};
use data::Data;
use derive_more::{Display, Error};
use dotenvy::dotenv;
use gql_client::{Client, GraphQLError};
use std::collections::HashMap;

mod data;

macro_rules! GITHUB_GRAPHQL_QUERY {
    () => {
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
}}"#
    };
}

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

fn prepare_github_query_headers(gh_token: &String) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", gh_token));
    headers.insert("User-Agent".to_string(), "PhunCache-App".to_string());
    headers
}

fn make_gh_graphql_client(gh_token: &String) -> Client {
    let headers = prepare_github_query_headers(gh_token);
    Client::new_with_headers("https://api.github.com/graphql", headers)
}

async fn make_gh_graphql_call(
    user: String,
    gh_token: &String,
) -> Result<Option<Data>, GraphQLError> {
    let body = format!(GITHUB_GRAPHQL_QUERY!(), user);
    let client = make_gh_graphql_client(gh_token);
    let data = client.query::<Data>(&body).await;
    println!("{:?}", data);
    data
}

#[get("/phundrak-com/{user}")]
async fn user_info_github(
    user: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let gh_token = &state.github_token;
    match make_gh_graphql_call(user.to_string(), gh_token).await {
        Ok(val) => Ok(web::Json(val)),
        Err(e) => Err(MyError {
            name: format!("Failed to retrieve data from GitHub: {e:?}"),
        }
        .into()),
    }
}

fn setup_application() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    human_panic::setup_panic!();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    use std::env::var;

    setup_application();

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
