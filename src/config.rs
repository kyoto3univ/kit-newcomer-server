use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "TWITTER_CONSUMER_KEY")]
    pub twitter_consumer_key: String,
    #[envconfig(from = "TWITTER_CONSUMER_SECRET")]
    pub twitter_consumer_secret: String,
    #[envconfig(
        from = "TWITTER_CALLBACK",
        default = "http://localhost:8080/auth/twitter/callback"
    )]
    pub twitter_callback: String,
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
}
