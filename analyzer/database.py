import psycopg2 as pg
import pandas as pd
import time

from analyzer.config import DB_CONFIG


class FinanceDatabase:
    def __init__(self):
        self.conn = pg.connect(**DB_CONFIG)

        self.INDEX_TICKER_TABLE = {
            "SNP500": "^GSPC",
            "NASDAQ": "^IXIC",
            "DowJones": "^DJI",
            "VIX": "^VIX",

            "KOSPI": "^KS11",

            "NATURAL_GAS": "NG=F",
            "WTI": "CL=F",
            "BITCOIN": "BTC-USD",

            "LONG_TERM_US_TREASURY": "TLT",
            "MID_TERM_US_TREASURY": "IEF",
            "SHORT_TERM_US_TREASURY": "SHY",
        }

        for tablename in self.INDEX_TICKER_TABLE.keys():
            self._check_or_create_index_table(tablename)

        self.conn.commit()

    def __del__(self):
        self.conn.close()

    def get_tables(self):
        return list(self.INDEX_TICKER_TABLE.keys())

    def get_ticker(self, index_name: str):
        return self.INDEX_TICKER_TABLE[index_name]

    def update_table(self, tablename: str, df: pd.DataFrame):
        with self.conn.cursor() as curs:
            df = df[["open", "high", "low", "close"]]

            for row in df.itertuples():
                date = row[0].strftime("%Y-%m-%d")
                ohlc = list(map(float, row[1:]))

                sql = f"""
                INSERT INTO "{tablename}" (date, open, high, low, close)
                VALUES ('{date}', {ohlc[0]:.2f}, {ohlc[1]:.2f}, {ohlc[2]:.2f}, {ohlc[3]:.2f})
                ON CONFLICT (date) DO NOTHING
                """
                curs.execute(sql)

        self.conn.commit()

    def _check_or_create_index_table(self, tablename: str):
        with self.conn.cursor() as curs:
            sql = f"""
            CREATE TABLE IF NOT EXISTS "{tablename}" (
                date DATE,
                open FLOAT,
                high FLOAT,
                low FLOAT,
                close FLOAT,
                PRIMARY KEY (date)
            ) 
            """
            curs.execute(sql)
