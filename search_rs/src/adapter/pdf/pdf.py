import pdfplumber


def extract_pdf(path: str):
    with pdfplumber.open(path) as pdf:
        total_pages = len(pdf.pages)

        content = ""
        for i in range(0, total_pages):
            content += pdf.pages[i].extract_text()

    return content

# if __name__ == '__main__':
#     print(extract_pdf())
#     print(extract_pdf(r'D:\Workspace\SearchInFiles\search_rs\examples\ReferenceCard.pdf'))
