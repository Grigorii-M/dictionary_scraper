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

    let word_info = dict_web_scraper::find_word(&args.word).await.unwrap();
    let result = word_info.definition();
    println!("{result}");
    
    Ok(())
}
