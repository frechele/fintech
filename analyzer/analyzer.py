import pandas as pd
import datetime

from analyzer.config import load_configuration
from analyzer.database import OHLCDatabase


def get_min_latest_date(db: OHLCDatabase) -> datetime.date:
    min_latest_date = datetime.date.today()
    for tablename in db.get_tables():
        min_latest_date = min(min_latest_date, db.get_last_update_date(tablename))

    return min_latest_date


def merge_all_tables(db: OHLCDatabase, latest_time: datetime.date, traiding_days: int) -> pd.DataFrame:
    tables = db.get_tables()
    df = db.get_values(tables[0], end_date=latest_time, limits=traiding_days)["close"].to_frame()
    df = df.rename(columns={"close": tables[0]})

    for tablename in tables[1:]:
        df_tmp = db.get_values(tablename, end_date=latest_time, limits=traiding_days)["close"].to_frame()
        df_tmp = df_tmp.rename(columns={"close": tablename})

        df = pd.merge(df, df_tmp, left_index=True, right_index=True, how="left")

    return df


def calc_correlation(db: OHLCDatabase) -> pd.DataFrame:
    latest_time = get_min_latest_date(db)

    df_short = merge_all_tables(db, latest_time, 5)
    df_short = df_short.drop("VIX", axis=1)
    corr_short = df_short.corr()["SNP500"]

    df_mid = merge_all_tables(db, latest_time, 20)
    df_mid = df_mid.drop("VIX", axis=1)
    corr_mid = df_mid.corr()["SNP500"]

    df_long = merge_all_tables(db, latest_time, 120)
    df_long = df_long.drop("VIX", axis=1)
    corr_long = df_long.corr()["SNP500"]

    df = pd.concat([corr_short, corr_mid, corr_long], axis=1, keys=["short", "mid", "long"])
    return df
