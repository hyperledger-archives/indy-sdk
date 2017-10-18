class PoolInfo:
    def __init__(self, handle=None, name=None):
        self.handle = handle
        self.name = name


class WalletInfo:
    def __init__(self, handle=None, name=None, list_did=None):
        self.handle = handle
        self.name = name
        self.list_did = list_did or []


class State:
    def __init__(self, wallet=None, pool=None, did=None, wallets=None, pools=None):
        self.wallet = wallet or WalletInfo()
        self.pool = pool or PoolInfo()
        self.did = did
        self.wallets = wallets or []
        self.pools = pools or []

    def to_json(self):
        return {
            'wallet': self.wallet.__dict__,
            'pool': self.pool.__dict__,
            'did': self.did,
            'wallets': self.wallets,
            'pools': self.pools
        }

    @staticmethod
    def from_json(data):
        return State(WalletInfo(data['wallet']['handle'], data['wallet']['name'], data['wallet']['list_did']),
                     PoolInfo(data['pool']['handle'], data['pool']['name']),
                     data['did'],
                     data['wallets'],
                     data['pools'])
