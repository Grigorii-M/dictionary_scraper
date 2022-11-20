import requests
from bs4 import BeautifulSoup
import json


def get_word_data(word):
    response = requests.get(f"https://www.dwds.de/api/wb/snippet/?q={word}")
    data = json.loads(response.text)
    print(word)

    for i in range(len(data)):
        print(f"\t{i})")

        lemma = data[i]
        article_url = lemma["url"]
        word_type = lemma["wortart"]

        response = requests.get(article_url)

        soup = BeautifulSoup(response.text, "html.parser")
        res = soup.find_all(class_="dwdswb-definition")

        if len(res) > 0:
            for tag in res:
                print(f"\t{tag.get_text()} -- (dwds)")
        else:
            url_part = ""
            if word_type in {"Adjektiv", "Komparativ", "Superlativ"}:
                url_part = "deklination/adjektive"
            elif word_type == "Substantiv":
                url_part = "deklination/substantive"
            elif word_type == "Verb":
                url_part = "konjugation"
            else:
                ...

            response = requests.get(
                f"https://www.verbformen.de/{url_part}/steckbrief/info/{word}.htm"
            )
            soup = BeautifulSoup(response.text, "html.parser")
            res = soup.find("i")
            assert res is not None
            print(f"\t{res.get_text()} -- (verbformen)")
