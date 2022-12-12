import yfinance as yf

from analyzer.database import FinanceDatabase


def update_with_yfinance():
    db = FinanceDatabase()

    for tablename, ticker in db.INDEX_TICKER_TABLE.items():
        print(f"Updating {tablename}...", end=" ")

        last_update = db.get_last_update_date(tablename)

        if last_update is None:
            print("from all")
            df = yf.download(ticker, period="max")
        else:
            print("from", last_update.strftime("%Y-%m-%d"))
            df = yf.download(ticker, start=last_update)

        df = df.rename(columns={
            "Date": "date",
            "Open": "open",
            "Low": "low",
            "High": "high",
            "Close": "close",
        })

        db.update_table(tablename, df)
