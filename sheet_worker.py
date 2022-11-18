import os.path

from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

# If modifying these scopes, delete the file token.json.
SCOPES = [
    "https://www.googleapis.com/auth/drive.readonly",
    "https://www.googleapis.com/auth/spreadsheets.readonly",
]


def _auth():
    creds = None
    # The file token.json stores the user's access and refresh tokens, and is
    # created automatically when the authorization flow completes for the first
    # time.
    if os.path.exists("token.json"):
        creds = Credentials.from_authorized_user_file("token.json", SCOPES)
    # If there are no (valid) credentials available, let the user log in.
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file("credentials.json", SCOPES)
            creds = flow.run_local_server(port=0)
        # Save the credentials for the next run
        with open("token.json", "w") as token:
            token.write(creds.to_json())

    return creds


def get_words():
    creds = _auth()
    translation_sheet = _get_translations_sheet(creds)
    sheet_values = _get_sheet_values(translation_sheet, creds)
    print(f"{sheet_values}")


def _get_translations_sheet(creds):
    try:
        service = build("drive", "v3", credentials=creds)
        files = list()
        page_token = None
        while True:
            response = (
                service.files()
                .list(
                    q="name contains 'Saved translations' and trashed = false",
                    # if at least one is specified, the rest should be too
                    fields="files(name, id, createdTime, mimeType)",
                    pageToken=page_token,
                )
                .execute()
            )
            for file in response.get("files", []):
                if file.get("mimeType") == "application/vnd.google-apps.spreadsheet":
                    files.append(file)

            page_token = response.get("nextPageToken", None)
            if page_token is None:
                break

        if len(files) == 1:
            return files[0]
        else:
            # Get the latest file
            files.sort(key=lambda el: el.get("createdTime"))

        return files[-1]

    except HttpError as error:
        print(f"An error occurred during file retrieval:\n {error}")


def _get_sheet_values(spreadsheet, creds):
    try:
        service = build("sheets", "v4", credentials=creds)
        sheet = service.spreadsheets()
        result = (
            sheet.values()
            .get(
                spreadsheetId=spreadsheet.get("id"),
                range=f"A:D",
            )
            .execute()
        )
        values = result.get("values", [])
        return values

    except HttpError as error:
        print(f"An error occured during sheet parsing:\n {error}")
