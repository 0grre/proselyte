mod info;
mod convert;

use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Info{
        path: String,
    },
    #[structopt(
    about = "get image's dimensions"
    )]
    Dimensions {
        path: String,
    },
    #[structopt(
    about = "Plays specified room",
    help = "USAGE: play MyRoomName"
    )]
    Play {
        name: String,
    },
    #[structopt(
    about = "Test a specified room",
    help = "USAGE: volume MyRoomName"
    )]
    Convert {
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    return match args {
        Cli::Info { path }  => info::info(path).await,
        Cli::Dimensions { path } => info::dimensions(path),
        Cli::Play { name } => play(name).await,
        Cli::Convert { path } => convert::to_webp(path).await
    }
}
// c'est bien le nom de la fonction qu'on appelle en argument
async fn play(name: String){
    println!("ça marche {}", name);
}
