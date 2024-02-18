use clap::Parser;
use librws::account::WClientBuilder;

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
    let _args = Args::parse();
    let wclient = WClientBuilder::new()
    // .w_id(&args.device_id)
    .random_id()
    // .random_key()
    .wg_key()
    .random_token()
    .random_dev()
    .random_tz()
    .build();

    println!("{:#?}", wclient);
}
