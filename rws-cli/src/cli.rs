use clap::Parser;
use librws::account::WClientBuilder;
use librws::api::CloudflareBuilder;
use librws::cf::Shot;

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
    let wclient = WClientBuilder::random();
    println!("{:#?}", wclient);
    let cfapi = CloudflareBuilder::from_dev(wclient.model()).get_api();
    println!("{:#?}", cfapi);

    let s = Shot::new();
    println!("{:#?}", s);
}
