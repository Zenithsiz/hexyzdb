//! hexyzdb backend

use {
	app_error::{AppError, Context},
	axum::{Json, http::StatusCode, response::IntoResponse},
	std::{
		io,
		net::{IpAddr, Ipv4Addr, SocketAddr},
		path::{Path, PathBuf},
		sync::Arc,
	},
	tokio::fs,
};


#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
	/// Port
	port: u16,

	/// Resources directory
	resources: PathBuf,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			port:      8081,
			resources: PathBuf::from("resources/"),
		}
	}
}

struct State {
	config: Config,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	// Read the config file
	let config_file = Path::new("config.toml");
	let config = match fs::read_to_string(&config_file).await {
		Ok(config) => toml::from_str::<Config>(&config).context("Unable to parse config")?,
		Err(err) if err.kind() == io::ErrorKind::NotFound => {
			let config = Config::default();
			let config_str = toml::to_string_pretty(&config).context("Unable to serialize config")?;
			if let Err(err) = fs::write(config_file, &config_str).await {
				tracing::warn!(?config_file, ?err, "Unable to write default config file");
			}

			config
		},
		Err(err) => return Err(AppError::new(&err).context("Unable to read config file")),
	};
	tracing::debug!("Configuration: {config:?}");

	let state = State { config };
	let state = Arc::new(state);

	// Then build the app
	let app = { axum::Router::new().with_state(Arc::clone(&state)) };

	let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), state.config.port);
	let listener = tokio::net::TcpListener::bind(addr)
		.await
		.context("Unable to create tcp listener")?;
	axum::serve(listener, app)
		.await
		.context("Unable to start http server")?;

	Ok(())
}

/// Request error
struct _ReqError(AppError);

impl<E: Into<AppError>> From<E> for _ReqError {
	fn from(err: E) -> Self {
		Self(err.into())
	}
}

impl IntoResponse for _ReqError {
	fn into_response(self) -> axum::response::Response {
		let status = StatusCode::INTERNAL_SERVER_ERROR;
		let message = self.0.pretty().to_string();

		(status, Json(message)).into_response()
	}
}
