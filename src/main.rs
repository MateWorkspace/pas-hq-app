use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    pas_hq_app::composition::launch().await
}
