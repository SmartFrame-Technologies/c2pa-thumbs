use c2pa::ManifestStore;
use c2pa::Manifest;
use std::error::Error;
use base64;
use std::process;
use serde::{Serialize, Deserialize};
use serde_json;
use clap::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct ManifestThumbs {
    thumbnail: Option<String>,
    thumbnails: Vec<String>
}

#[derive(Parser)]
#[command(about = "I'm parsing cai data from images. Use `-h` to see more.", long_about = None)]
struct Args {
    #[arg(help = "The path to the file to read")]
    path: String,

    #[arg(short, long, help = "The manifest id - if not passed use active manifest")]
    manifest_label: Option<String>,

    #[arg(short, help = "List manifests labels")]
    list_manifests_labels: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let manifest_store = ManifestStore::from_file(args.path)?;

    if args.list_manifests_labels {
        println!("Possible manifests: ");
        for key in manifest_store.manifests().keys() {
            println!("{key}");
        }
        process::exit(0x0000);
    }

    let manifest: Option<&Manifest>;
    if args.manifest_label.is_none() {
        manifest = manifest_store.get_active();
    } else {
        manifest = manifest_store.get(args.manifest_label.unwrap().as_str());
    }

    let mut thumbs = ManifestThumbs { thumbnail: None, thumbnails: Vec::new()};
    if manifest.is_none() {
        println!("Error: Manifest unknown");
        process::exit(0x0100);
    }

    let ingredients = manifest.unwrap().ingredients();
    if let Some((format, data)) = manifest.unwrap().thumbnail() {
        thumbs.thumbnail = Some(format!("data:{};charset=utf-8;base64,{}", format, base64::encode(data.to_vec())));
    }

    for i in 0..ingredients.len() {
        if let Some((format, data)) = ingredients[i].thumbnail() {
            thumbs.thumbnails.push(format!("data:{};charset=utf-8;base64,{}", format, base64::encode(data.to_vec())));
        }
    }

    println!("{}", serde_json::to_string(&thumbs).unwrap());
    Ok(())
}
