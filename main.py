import scraper

word = input("Enter the word: ").lower()
data = scraper.get_word_data(word)

for d in data:
    print(d)
