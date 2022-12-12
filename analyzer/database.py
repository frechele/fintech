import psycopg2 as pg
import pandas as pd
import datetime
from typing import Union
from enum import Enum

import warnings

warnings.filterwarnings('ignore')

from analyzer.config import DB_CONFIG


class OHLCDatabase:
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

    def get_last_update_date(self, tablename: str):
        with self.conn.cursor() as curs:
            sql = f"""
            SELECT MAX(date) FROM "{tablename}"
            """
            curs.execute(sql)
            return curs.fetchone()[0]

    def get_values(self, tablename: str, start_date: Union[None, datetime.date] = None, end_date: Union[None, datetime.date] = None, limits: Union[None, int] = None) -> pd.DataFrame:
        if start_date is None and end_date is None:
            sql = f"""
            SELECT * FROM "{tablename}"
            """
        elif start_date is None:
            end_date = end_date.strftime("%Y-%m-%d")
            sql = f"""
            SELECT * FROM "{tablename}"
            WHERE date <= '{end_date}'
            """
        elif end_date is None:
            start_date = start_date.strftime("%Y-%m-%d")
            sql = f"""
            SELECT * FROM "{tablename}"
            WHERE date >= '{start_date}'
            """
        else:
            start_date = start_date.strftime("%Y-%m-%d")
            end_date = end_date.strftime("%Y-%m-%d")
            sql = f"""
            SELECT * FROM "{tablename}"
            WHERE date BETWEEN '{start_date}' AND '{end_date}'
            """

        if limits is not None:
            sql += f"""
            ORDER BY date DESC
            LIMIT {limits}
            """

        df = pd.read_sql(sql, self.conn)

        if limits is not None:
            df = df.sort_values("date")

        return df.set_index("date")

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


class TimeTerm(Enum):
    SHORT = 0
    MID = 1
    LONG = 2


class AnalysisDatabase:
    def __init__(self):
        self.conn = pg.connect(**DB_CONFIG)

        self._create_corr_table()
        self.conn.commit()

    def __del__(self):
        self.conn.close()

    def update_correlation(self, df: pd.DataFrame):
        with self.conn.cursor() as curs:
            sql = """
            DELETE FROM "correlation"
            """
            curs.execute(sql)

            for row in df.iterrows():
                code = row[0]
                for term, corr in row[1].items():
                    term = TimeTerm[term.upper()].value

                    sql = f"""
                    INSERT INTO "correlation" (code, term, corr)
                    VALUES ('{code}', {term}, {corr:.5f})
                    """
                    curs.execute(sql)

        self.conn.commit()

    def get_correlation(self, code: str, term: TimeTerm) -> float:
        with self.conn.cursor() as curs:
            sql = f"""
            SELECT corr FROM "correlation"
            WHERE code = '{code}' AND term = {term.value}
            """
            curs.execute(sql)
            return curs.fetchone()[0]

    def _create_corr_table(self):
        with self.conn.cursor() as curs:
            sql = """
            CREATE TABLE IF NOT EXISTS "correlation" (
                code VARCHAR(30),
                term INT,
                corr FLOAT,
                PRIMARY KEY (code, term)
            )
            """
            curs.execute(sql)
