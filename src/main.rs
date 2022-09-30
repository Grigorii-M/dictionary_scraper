use clap::Parser;
use scraper::{Html, Selector};

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

    let resp = reqwest::get(format!(
        "https://verbformen.de/konjugation/?w={}",
        args.word
    ))
    .await?;

    let responce_body = resp.text().await?;

    let document = Html::parse_document(&responce_body);

    let info_selector = Selector::parse(".rAbschnitt").unwrap();

    // verbformen.de webpage has only one element with .rAbschnitt class
    let word_info = document.select(&info_selector).next().unwrap();

    let selector1 = Selector::parse(".rInf").unwrap();
    let selector2 = Selector::parse(".r1Zeile").unwrap();
    let selector3 = Selector::parse(".rU3px").unwrap();
    let selector4 = Selector::parse(".rO0px").unwrap();
    let selector5 = Selector::parse(".rNt").unwrap();

    let definition = word_info
        .select(&selector1)
        .find(|el| {
            selector2.matches(el)
                && selector3.matches(el)
                && selector4.matches(el)
                && selector5.matches(el)
        })
        .unwrap();
    let mut definition_str = String::new();
    for el in definition.text() {
        definition_str.push_str(el);
    }

    println!("{}", definition_str.trim());
    Ok(())
}
