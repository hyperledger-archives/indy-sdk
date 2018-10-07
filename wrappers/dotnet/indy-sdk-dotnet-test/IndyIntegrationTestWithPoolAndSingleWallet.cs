using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test
{
    public abstract class IndyIntegrationTestWithPoolAndSingleWallet : IndyIntegrationTestBase
    {
        protected Pool pool;
        protected Wallet wallet;
        protected string poolName;

        [TestInitialize]
        public async Task CreateWallet()
        {
            poolName = PoolUtils.CreatePoolLedgerConfig();
            pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await Wallet.CreateWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
            wallet = await Wallet.OpenWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await pool.CloseAsync();
            await wallet.CloseAsync();
            await Wallet.DeleteWalletAsync(WALLET_CONFIG, WALLET_CREDENTIALS);
        }
    }
}
