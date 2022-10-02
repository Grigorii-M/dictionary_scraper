use dict_web_scraper::{WordInfo, WordType};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[clap(group(
        clap::ArgGroup::new("part of speech")
        .required(true)
        .args(&["verb", "noun", "adjective"])
        ))]
struct Args {
    /// The word to find in the dictionary
    #[arg(short, long)]
    word: String,

    #[clap(short, long)]
    verb: bool,
    #[clap(short, long)]
    noun: bool,
    #[clap(short, long)]
    adjective: bool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // match (args.verb, args.noun, args.adjective) {
    //     (true, _, _) => println!("verb"),
    //     (_, true, _) => println!("noun"),
    //     (_, _, true) => println!("adj"),
    //     _ => unreachable!(),
    // };

    let word_info = dict_web_scraper::find_word(&args.word, WordType::Verb)
        .await
        .unwrap();

    match word_info {
        WordInfo::VerbInfo(verb) => println!(
            "{}:\n{}\n{}\n{}\n{}\n{}\n{}",
            args.word,
            verb.meta_info,
            verb.verb_forms,
            verb.translation,
            verb.definition,
            verb.usage,
            verb.example
        ),
        _ => {}
    }

    Ok(())
}
