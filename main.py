import scraper
import sheet_worker


def main():
    sheet_values = sheet_worker.get_sheet_values()
    valid, invalid = _parse_word_list(sheet_values)


def _parse_word_list(word_list):
    valid = []
    invalid = []

    for row in word_list:
        word = ""
        if row[0] == "German":
            word = row[2]
        elif row[1] == "German":
            word = row[3]
        else:
            invalid.append(row)
            continue
        if " " in word:
            invalid.append(row)
            continue
        valid.append(word)

    return (valid, invalid)


if __name__ == "__main__":
    main()
