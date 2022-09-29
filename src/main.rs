use clap::Parser;
use scraper::{ Html, Selector };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// The word to find in the dictionary
    #[arg(short, long)]
    word: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args = Args::parse();

    let resp = reqwest::get(format!("https://verbformen.de/konjugation/?w={}", args.word))
        .await?;
    let responce_headers = resp.headers();
    println!("{:#?}", responce_headers);

    let responce_body = resp
        .text()
        .await?;

    let document = Html::parse_document(&responce_body);
    // println!("{:#?}", document);

    // let selector = Selector::parse("rInf, r1Zeile, ru3px, rO0px, rNt").unwrap();
    // // println!("{:?}", selector);
    //
    // for el in document.select(&selector) {
    //     println!("{:#?}", el);
    // }

    Ok(())
}
