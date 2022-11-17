import requests
from bs4 import BeautifulSoup
from enum import Enum


class Word(Enum):
    Undefined = 0
    Substantive = "deklination/substantive"
    Verb = "konjugation"


def get_word_data(word):
    responce = requests.get(f"https://www.verbformen.de/?w={word}")
    soup = BeautifulSoup(responce.text, "html.parser")

    title = soup.title
    assert title is not None
    title = title.string
    assert title is not None

    word_type = Word.Undefined
    if "des Verbs" in title:
        word_type = Word.Verb
    elif "des Substantivs" in title:
        word_type = Word.Substantive
    else:
        raise ValueError("Unexpected word type")

    responce1 = requests.get(
        f"https://www.verbformen.de/{word_type.value}/steckbrief/info/{word}.htm"
    )
    soup1 = BeautifulSoup(responce1.text, "html.parser")
    paras = soup1.find_all("p")

    data = list()
    for p in paras:
        data.append(p.get_text().strip().replace("\n", " "))

    return data
