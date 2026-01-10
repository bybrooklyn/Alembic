use std::env;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: IpAddr,
    pub port: u16,
    pub aggregation_interval: Duration,
    pub rate_limit_per_min: u32,
    pub rate_limit_burst: u32,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:alembic.db".to_string());
        
        let host = env::var("ALEMBIC_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string())
            .parse()
            .unwrap_or([0, 0, 0, 0].into());
            
        let port = env::var("ALEMBIC_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);
            
        let aggregation_interval_sec = env::var("ALEMBIC_AGGREGATION_INTERVAL")
            .unwrap_or_else(|_| "60".to_string())
            .parse()
            .unwrap_or(60);

        let rate_limit_per_min = env::var("ALEMBIC_RATE_LIMIT_PER_MIN")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);
            
        let rate_limit_burst = env::var("ALEMBIC_RATE_LIMIT_BURST")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        Self {
            database_url,
            host,
            port,
            aggregation_interval: Duration::from_secs(aggregation_interval_sec),
            rate_limit_per_min,
            rate_limit_burst,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn rate_limit_period(&self) -> Duration {
         // period = 60s / requests_per_min
         // e.g. 30 req/min => 60/30 = 2 seconds per request token
         if self.rate_limit_per_min == 0 {
             Duration::from_secs(60)
         } else {
             Duration::from_secs_f64(60.0 / self.rate_limit_per_min as f64)
         }
    }
}
