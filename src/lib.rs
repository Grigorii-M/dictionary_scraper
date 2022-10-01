use scraper::{Html, Selector};

pub struct DictionaryScraper {
    word: String,
    html: Html,
}

impl DictionaryScraper {
    pub async fn new(word: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let resp = reqwest::get(format!("https://verbformen.de/konjugation/?w={}", word)).await?;

        let responce_body = resp.text().await?;

        let html = Html::parse_document(&responce_body);

        Ok(Self {
            word: word.to_string(),
            html,
        })
    }

    pub fn define(&self) -> String {
        let info_selector = Selector::parse(".rAbschnitt").unwrap();

        // verbformen.de webpage has only one element with .rAbschnitt class
        let word_info = self.html.select(&info_selector).next().unwrap();

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

        definition_str
    }

    pub fn word(&self) -> String {
        self.word.to_string()
    }
}
