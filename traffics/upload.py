import os
import subprocess
import concurrent.futures
import pandas as pd

os.makedirs("./layer1", exist_ok=True)
os.makedirs("./layer2", exist_ok=True)
os.makedirs("./layer3", exist_ok=True)

env = os.environ.copy()
env["PGPASSWORD"] = "0"

subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", "DROP TABLE IF EXISTS traffics;"], env=env)
subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", "CREATE TABLE IF NOT EXISTS traffics (datetime Timestamp, src_code Text, loc_code Int, link_type Int, link_code Int, traffics Int);"], env=env)

def csv_convert(f1):
    csv = pd.read_csv(f"./layer2/{f1}", encoding="sjis")
    csv = csv[["時刻", "情報源コード", "計測地点番号", "リンク区分", "リンク番号", "断面交通量"]]
    csv = csv.rename(columns={
        "時刻": "datetime",
        "情報源コード": "src_code",
        "計測地点番号": "loc_code",
        "リンク区分": "link_type",
        "リンク番号": "link_code",
        "断面交通量": "traffics",
    })
    csv.to_csv(f"./layer3/{f1}", index=False)
    print(f1)

for f0 in os.listdir("./data"):
    for f1 in os.listdir("./layer1"):
        os.remove(f"./layer1/{f1}")
    for f1 in os.listdir("./layer2"):
        os.remove(f"./layer2/{f1}")
    for f1 in os.listdir("./layer3"):
        os.remove(f"./layer3/{f1}")

    subprocess.call(["sh", "-c", f"zipinfo -1 ./data/{f0} | grep tokyo | parallel \"unzip -j -d ./layer1 ./data/{f0} {{}}\""])

    subprocess.call(["sh", "-c", "ls ./layer1 | parallel \"unzip -j -d ./layer2 -O sjis ./layer1/{}\""])

    with concurrent.futures.ProcessPoolExecutor(max_workers=10) as executor:
        executor.map(csv_convert, os.listdir("./layer2"))

    for f1 in os.listdir("./layer3"):
        subprocess.call(["psql", "-U", "postgis", "-h", "localhost", "-c", f"\\COPY traffics FROM ./layer3/{f1} WITH CSV HEADER"], env=env)
