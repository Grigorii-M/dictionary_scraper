use scraper::{Html, Selector};

#[derive(Debug)]
pub enum WordType {
    Verb,
    Noun,
    Adjective,
}

pub async fn find_word(
    word: &str,
    word_type: WordType,
) -> Result<WordInfo, Box<dyn std::error::Error>> {
    // use `konjugation/<wrd>.htm` for verbs
    // use `deklination/substantive/<wrd>.htm` for nouns
    // use `deklination/adjektive/<wrd>.htm` for adjectives
    let resp = reqwest::get(format!("https://verbformen.de/konjugation/{}.htm", word)).await?;

    let responce_body = resp.text().await?;

    let html = Html::parse_document(&responce_body);

    let info_selector = Selector::parse(".rAbschnitt").unwrap();
    // verbformen.de webpage has only one element with .rAbschnitt class
    let full_info = Html::parse_document(&html.select(&info_selector).next().unwrap().html());

    match word_type {
        _ => Ok(WordInfo::VerbInfo(find_verb(&full_info))),
    }
}

pub enum WordInfo {
    VerbInfo(VerbInfo),
    NounInfo,
    AdjectiveInfo,
}

pub struct VerbInfo {
    pub meta_info: String,
    pub verb_forms: String,
    pub translation: String,
    pub definition: String,
    pub usage: String,
    pub example: String,
}

pub fn find_verb(html: &Html) -> VerbInfo {
    let brief_info_selector = Selector::parse("#vVdBxBox").unwrap();
    let brief_info = html.select(&brief_info_selector).next().unwrap();

    let gr_info_selector = Selector::parse(".rInf").unwrap();
    let gr_info = brief_info.select(&gr_info_selector).next().unwrap();

    let meta_info = gr_info.text().fold(String::new(), |mut acc, line| {
        acc = format!("{}{}", acc, line.replace("\n", " "));
        acc
    }).trim().to_string();

    let verb_forms_selector = Selector::parse("#stammformen").unwrap();
    let verb_forms = brief_info.select(&verb_forms_selector).next().unwrap();

    let verb_forms = verb_forms.text().fold(String::new(), |mut acc, line| {
        acc = format!("{}{}", acc, line.replace("\n", " "));
        acc
    }).trim().to_string();

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

    // info.for_each(|line| {
    //     data.push(line.trim().replace("\n", " "));
    // });

    let (translation, definition, usage, example) = {
        let info = info
            .map(|el| el.trim().replace("\n", " "))
            .collect::<Vec<_>>();
        (
            info[0].trim().to_string(),
            info[1].trim().to_string(),
            info[2].trim().to_string(),
            info[3].trim().to_string(),
        )
    };

    VerbInfo {
        meta_info,
        verb_forms,
        translation,
        definition,
        usage,
        example,
    }
}
