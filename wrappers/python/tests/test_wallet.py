from sovrin_client import SovrinError
from sovrin_client import SovrinWallet

a = SovrinWallet()
ret = a.create_wallet(0,
                      "pool", "wallet", "wallet-type",
                      "config", "creds",
                      lambda h,err: print("cb", h, err))

print("Hello")
