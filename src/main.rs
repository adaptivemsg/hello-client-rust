use adaptivemsg as am;
use clap::Parser;
use hello_server::api::hello::{HelloReply, HelloRequest};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "hello-client-rust", about = "Hello client for hello-server-go")]
struct Args {
    /// Server address (examples: tcp://127.0.0.1:5555, uds://@adaptivemsg-hello, uds:///tmp/adaptivemsg-hello.sock)
    #[arg(
        short,
        long,
        default_value = "tcp://127.0.0.1:5555",
        help = "Use tcp://HOST:PORT for TCP, uds://@adaptivemsg-* for abstract UDS, or uds:///tmp/adaptivemsg-*.sock for a filesystem socket"
    )]
    addr: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let client = am::Client::new();
    let conn = client.connect(&args.addr).await?;

    let reply: HelloReply = conn
        .send_recv(HelloRequest {
            who: "John".into(),
            question: "who are you".into(),
        })
        .await?;
    info!("reply: {}", reply.answer);

    let err_reply: Result<HelloReply, am::Error> = conn
        .send_recv(HelloRequest {
            who: "Bob".into(),
            question: "error please".into(),
        })
        .await;
    match err_reply {
        Ok(reply) => info!("unexpected reply: {}", reply.answer),
        Err(am::Error::Remote { code, message }) => {
            warn!("expected error: {code}: {message}");
        }
        Err(err) => return Err(err.into()),
    }

    Ok(())
}
