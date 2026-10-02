use clap::{
    Parser,
    Subcommand,
};
use std::{fs::File, time::Duration};
use rodio::{Source, source::SineWave};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    Play {
        file: String,
    }
}

fn main() {
    let args = Args::parse();

    match args.command { 
        Commands::Play { file } => 
        {      
            let handle = rodio::DeviceSinkBuilder::open_default_sink()
                .expect("Open default audio stream");
            
            let file = File::open(file).unwrap();

            let player = rodio::play(&handle.mixer(), file).unwrap();

            let source = SineWave::new(
                440.0).take_duration(Duration::from_secs_f32(0.25)
            ).amplify(0.20);
            player.append(source);

            player.sleep_until_end();
        }
    }
}