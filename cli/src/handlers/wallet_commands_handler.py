from indy import wallet

from src.constants import WALLET, POOL
from src.utils.environment import wallet_dir, get_files
from src.utils.errors import CliError


class WalletCommandHandler:
    @staticmethod
    async def init_wallet(state):
        wallets = get_files(wallet_dir())
        wallet_name = state.wallet.name or WALLET
        pool_name = state.pool.name or POOL

        if wallet_name not in wallets:
            await wallet.create_wallet(pool_name, wallet_name, None, None, None)

        handle = await wallet.open_wallet(wallet_name, None, None)
        state.wallet.name = wallet_name
        state.wallet.handle = handle
        state.wallets = wallets

        print("Wallet {} set as active".format(WALLET))

    @staticmethod
    async def create_wallet_command_handler(state, matched_vars):
        pool_name = matched_vars.get('pool_name')
        if not pool_name and not state.pool.name:
            raise CliError("Connect to any pool or pass pool name")

        pool_name = matched_vars.get('pool_name') or state.pool.name
        wallet_name = matched_vars.get('wallet_name')
        await wallet.create_wallet(pool_name,
                                   wallet_name,
                                   matched_vars.get('type'),
                                   matched_vars.get('config'),
                                   matched_vars.get('credentials'))
        print("Wallet created: {} for pool {}".format(wallet_name, pool_name))

    @staticmethod
    async def use_wallet_command_handler(state, matched_vars):
        wallet_name = matched_vars.get('wallet_name')

        if wallet_name not in state.wallets:
            raise CliError("Invalid wallet")

        if wallet_name == state.wallet.name:
            raise CliError("Wallet {} already active".format(wallet_name))

        if state.wallet:
            await wallet.close_wallet(state.wallet.handle)

        handle = await wallet.open_wallet(wallet_name,
                                          matched_vars.get('config'),
                                          matched_vars.get('credentials'))
        state.wallet.name = wallet_name
        state.wallet.handle = handle
        print("Wallet {} set as active".format(wallet_name))

    @staticmethod
    async def close_wallet(state, matched_vars):
        if state.wallet:
            await wallet.close_wallet(state.wallet.handle)

    @staticmethod
    async def list_wallets_command_handler(state, matched_vars):
        if state.wallets:
            print("Available wallets:")
            print("    " + "\n    ".join(state.wallets))
        else:
            print("You haven't any wallet")
