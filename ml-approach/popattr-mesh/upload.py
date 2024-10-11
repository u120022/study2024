import os
import pandas as pd

dfs = []
for f in os.listdir("./data"):
    dfs.append(pd.read_csv(f"./data/{f}"))
df = pd.concat(dfs)

print(df.iloc[:, 0])
print(df)
