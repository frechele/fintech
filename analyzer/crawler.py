import yfinance as yf

from analyzer.database import FinanceDatabase


def update_with_yfinance():
    db = FinanceDatabase()

    for tablename, ticker in db.INDEX_TICKER_TABLE.items():
        print(f"Updating {tablename}...")
        df = yf.download(ticker, period="max")

        df = df.rename(columns={
            "Date": "date",
            "Open": "open",
            "Low": "low",
            "High": "high",
            "Close": "close",
        })

        db.update_table(tablename, df)


if __name__ == "__main__":
    update_with_yfinance()
