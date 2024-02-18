use clap::Parser;
use librws::warp::account::WClientBuilder;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short='d', long, name = "DEVICE_ID")]
    device_id: String,

    #[clap(short='i', long, name = "INTERFACE_NAME")]
    interface_name: String,

    #[clap(short='p', long, name = "SOCKS5_PORT")]
    port_socks5: String,

    #[clap(short='U', long, name = "USER_PASSWD")]
    user_passwd: String,
}

fn main() {
    let args = Args::parse();
    let wclient = WClientBuilder::new()
    .w_id(&args.device_id)
    .build();

    wclient.process();
}
