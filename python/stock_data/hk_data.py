import create_data
engine = create_data.engine

import tushare as ts
ts.set_token('5625a39d681260388dba159adfc437019ab0b45cd2efc94204406915887f6b95')
hk = ts.HKequity()
df = hk.HKEqu(listStatusCD='L', field='secShortName,listDate,tradingUnit,partyID')
#df = df.

print(df.to_json())





