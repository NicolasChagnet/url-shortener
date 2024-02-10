use crate::model::Url;
use std::env;
use mongodb::{IndexModel,bson::doc,
    options::{ClientOptions, Compressor, IndexOptions}, Client, Collection
};
use std::time::Duration;
use rand::{distributions::Alphanumeric, Rng};

pub const DB_NAME: &str = "url_shorten";
pub const COLL_NAME: &str = "urls";
const NSTR: usize = 4;

fn gen_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(NSTR)
        .map(char::from)
        .collect()
}

pub async fn connect_db() -> Client {
    // Loading database config
    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri)
        .await
        .expect("Error parsing database URI!");
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    // the server will select the algorithm it supports from the list provided by the driver
    client_options.compressors = database_config.compressors;
    let client = Client::with_options(client_options)
        .expect("Error connecting to database!");
    return client;
}

pub async fn create_index_url(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"from": 1})
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<Url>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("Error creating a unique index!");
}

pub fn get_url_collection(client: &Client) -> Collection<Url> {
    client
        .database(DB_NAME)
        .collection::<Url>(COLL_NAME)
}

pub async fn get_url_origin_db(client: &Client, from: &str) -> Option<Url> {
    let collection: Collection<Url> = get_url_collection(client);
    let url_rq = collection.find_one(
        doc! {
            "from": from
        }, None).await;
    match url_rq {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error fetching in DB.");
            None
        }
    }
}

pub async fn create_unique_url_obj(client: &Client) -> String {
    let mut rnd_str = gen_random_string();
    let mut req_from = get_url_origin_db(client, &rnd_str).await;
    loop {
        match req_from {
            None => return rnd_str,
            Some(_) => {
                rnd_str = gen_random_string();
                req_from = get_url_origin_db(client, &rnd_str).await;
            }
        }
    }
}

pub async fn set_url_db(client: &Client, to: &str) -> Option<String>  {
    let collection = get_url_collection(client);
    let from = create_unique_url_obj(client).await;
    let url_obj = Url::new(&from, to);
    let result = collection.insert_one(url_obj, None).await;
    match result {
        Ok(_) => Some(from),
        Err(err) => {
            eprintln!("{}", err.to_string());
            None
        }
    }
}

// Database configure structure
pub struct DatabaseConfig {
    pub uri: String,
    pub connection_timeout: Option<Duration>,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
    pub compressors: Option<Vec<Compressor>>
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let mongo_uri: String = env::var("MONGO_URI")
            .expect("Failed to load `MONGO_URI` environment variable.");

        let mongo_connection_timeout: u64 = env::var("MONGO_CONNECTION_TIMEOUT")
            .expect("Failed to load `MONGO_CONNECTION_TIMEOUT` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_CONNECTION_TIMEOUT` environment variable.");

        let mongo_min_pool_size: u32 = env::var("MONGO_MIN_POOL_SIZE")
            .expect("Failed to load `MONGO_MIN_POOL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_MIN_POOL_SIZE` environment variable.");

        let mongo_max_pool_size: u32 = env::var("MONGO_MAX_POOL_SIZE")
            .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_MAX_POOL_SIZE` environment variable.");

        Self {
            uri: mongo_uri,
            connection_timeout: Some(Duration::from_secs(mongo_connection_timeout)),
            min_pool_size: Some(mongo_min_pool_size),
            max_pool_size: Some(mongo_max_pool_size),
            compressors: Some(vec![
               Compressor::Snappy,
               Compressor::Zlib {
                   level: Default::default(),
               },
               Compressor::Zstd {
                   level: Default::default(),
               },
           ])
        }
    }
}