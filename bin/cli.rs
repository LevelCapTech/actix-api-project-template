//! CLI tool driving the API client
use anyhow::{anyhow, Context, Result};
use dialoguer::Confirm;
use log::{debug, info};
// models may be unused if all inputs are primitive types
#[allow(unused_imports)]
use openapi_client::{
    models, ApiNoContext, Client, ContextWrapperExt,
    GetIndexResponse,
    ItemsGetResponse,
    ItemsPostResponse,
    ItemsIdDeleteResponse,
    ItemsIdGetResponse,
    ItemsIdPutResponse,
};
use simple_logger::SimpleLogger;
use structopt::StructOpt;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Actix Web API",
    version = "1.0.0",
    about = "CLI access to Actix Web API"
)]
struct Cli {
    #[structopt(subcommand)]
    operation: Operation,

    /// Address or hostname of the server hosting this API, including optional port
    #[structopt(short = "a", long, default_value = "http://localhost")]
    server_address: String,

    /// Path to the client private key if using client-side TLS authentication
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long, requires_all(&["client-certificate", "server-certificate"]))]
    client_key: Option<String>,

    /// Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long, requires_all(&["client-key", "server-certificate"]))]
    client_certificate: Option<String>,

    /// Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long)]
    server_certificate: Option<String>,

    /// If set, write output to file instead of stdout
    #[structopt(short, long)]
    output_file: Option<String>,

    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Don't ask for any confirmation prompts
    #[allow(dead_code)]
    #[structopt(short, long)]
    force: bool,
}

#[derive(StructOpt, Debug)]
enum Operation {
    /// メインエンドポイント
    GetIndex {
    },
    /// アイテム一覧取得
    ItemsGet {
    },
    /// 新規アイテム作成
    ItemsPost {
        #[structopt(parse(try_from_str = parse_json))]
        items_post_request: models::ItemsPostRequest,
    },
    /// アイテム削除
    ItemsIdDelete {
        id: i32,
    },
    /// アイテム取得
    ItemsIdGet {
        id: i32,
    },
    /// アイテム更新
    ItemsIdPut {
        id: i32,
        #[structopt(parse(try_from_str = parse_json))]
        items_id_put_request: models::ItemsIdPutRequest,
    },
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    if args.client_certificate.is_some() {
        debug!("Using mutual TLS");
        let client = Client::try_new_https_mutual(
            &args.server_address,
            args.server_certificate.clone().unwrap(),
            args.client_key.clone().unwrap(),
            args.client_certificate.clone().unwrap(),
        )
        .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else if args.server_certificate.is_some() {
        debug!("Using TLS with pinned server certificate");
        let client =
            Client::try_new_https_pinned(&args.server_address, args.server_certificate.clone().unwrap())
                .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else {
        debug!("Using client without certificates");
        let client =
            Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
        Ok(Box::new(client.with_context(context)))
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    let client =
        Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
    Ok(Box::new(client.with_context(context)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    if let Some(log_level) = args.verbosity.log_level() {
        SimpleLogger::new().with_level(log_level.to_level_filter()).init()?;
    }

    debug!("Arguments: {:?}", &args);

    let auth_data: Option<AuthData> = None;

    #[allow(trivial_casts)]
    let context = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        auth_data,
        XSpanIdString::default()
    );

    let client = create_client(&args, context)?;

    let result = match args.operation {
        Operation::GetIndex {
        } => {
            info!("Performing a GetIndex request");

            let result = client.get_index(
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetIndexResponse::Status200
                (body)
                => "Status200\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::ItemsGet {
        } => {
            info!("Performing a ItemsGet request");

            let result = client.items_get(
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                ItemsGetResponse::Status200
                (body)
                => "Status200\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::ItemsPost {
            items_post_request,
        } => {
            info!("Performing a ItemsPost request");

            let result = client.items_post(
                items_post_request,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                ItemsPostResponse::Status201
                (body)
                => "Status201\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::ItemsIdDelete {
            id,
        } => {
            prompt(args.force, "This will delete the given entry, are you sure?")?;
            info!("Performing a ItemsIdDelete request on {:?}", (
                &id
            ));

            let result = client.items_id_delete(
                id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                ItemsIdDeleteResponse::Status204
                => "Status204\n".to_string()
                    ,
            }
        }
        Operation::ItemsIdGet {
            id,
        } => {
            info!("Performing a ItemsIdGet request on {:?}", (
                &id
            ));

            let result = client.items_id_get(
                id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                ItemsIdGetResponse::Status200
                (body)
                => "Status200\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
        Operation::ItemsIdPut {
            id,
            items_id_put_request,
        } => {
            info!("Performing a ItemsIdPut request on {:?}", (
                &id
            ));

            let result = client.items_id_put(
                id,
                items_id_put_request,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                ItemsIdPutResponse::Status200
                (body)
                => "Status200\n".to_string()
                   +
                    &serde_json::to_string_pretty(&body)?,
            }
        }
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(output_file, result)?
    } else {
        println!("{}", result);
    }
    Ok(())
}

fn prompt(force: bool, text: &str) -> Result<()> {
    if force || Confirm::new().with_prompt(text).interact()? {
        Ok(())
    } else {
        Err(anyhow!("Aborting"))
    }
}

// May be unused if all inputs are primitive types
#[allow(dead_code)]
fn parse_json<'a, T: serde::de::Deserialize<'a>>(json_string: &'a str) -> Result<T> {
    serde_json::from_str(json_string).map_err(|err| anyhow!("Error parsing input: {}", err))
}
