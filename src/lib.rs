use scraper::{Html, Selector};
use scraper::element_ref::Text;

pub async fn find_word(word: &str) -> Result<WordInfo, Box<dyn std::error::Error>> {
    // use `konjugation/<wrd>.htm` for verbs
    // use `deklination/substantive/<wrd>.htm` for nouns
    // use `deklination/adjektive/<wrd>.htm` for adjectives
    let resp = reqwest::get(format!("https://verbformen.de/konjugation/{}.htm", word)).await?;

    let responce_body = resp.text().await?;

    let html = Html::parse_document(&responce_body);

    let info_selector = Selector::parse(".rAbschnitt").unwrap();
    // verbformen.de webpage has only one element with .rAbschnitt class
    let full_info = Html::parse_document(&html.select(&info_selector).next().unwrap().html());

    Ok(WordInfo {
        html: full_info,
    })
}

pub struct WordInfo {
    html: Html,
}

impl WordInfo {
    pub fn definition(&self) -> String {
        let brief_info_selector = Selector::parse("#vVdBxBox").unwrap();
        let brief_info = self.html.select(&brief_info_selector).next().unwrap();

        let gr_info_selector = Selector::parse(".rInf").unwrap();
        let gr_info = brief_info.select(&gr_info_selector).next().unwrap();

        let mut data = Vec::new();

        data.push(gr_info.text().fold(String::new(), |mut acc, line| {acc = format!("{}{}", acc, line.replace("\n", " ")); acc }));

        let verb_forms_selector = Selector::parse("#stammformen").unwrap();
        let verb_forms = brief_info.select(&verb_forms_selector).next().unwrap();

        data.push(verb_forms.text().fold(String::new(), |mut acc, line| {acc = format!("{}{}", acc, line.replace("\n", " ")); acc }));

        // These classes are being used with translation, definition, case and example
        let selector1 = Selector::parse(".r1Zeile").unwrap();
        let selector2 = Selector::parse(".rU3px").unwrap();
        let selector3 = Selector::parse(".rO0px").unwrap();

        let info = brief_info.select(&selector1).filter_map(|el| {
            if selector2.matches(&el) && selector3.matches(&el) {
                let mut data = String::new();
                el.text().for_each(|line| data.push_str(line));
                Some(data)
            } else {
                None
            }
        });

        info.for_each(|line| {
            data.push(line.trim().replace("\n", " "));
        });

        let mut result = String::new();
        data.into_iter().for_each(|line| {
            result.push_str(&line.trim());
            result.push_str("\n");
        });
        result
    }
}
