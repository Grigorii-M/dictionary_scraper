use dict_web_scraper::DictionaryScraper;
use clap::Parser;

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

    if let Ok(scraper) = DictionaryScraper::new(&args.word).await {
        let definition_str = scraper.define();

        println!("{}: {}", scraper.word(), definition_str.trim());
    }
    
    Ok(())
}
