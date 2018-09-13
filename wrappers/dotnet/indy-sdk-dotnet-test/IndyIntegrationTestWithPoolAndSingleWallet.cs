using Hyperledger.Indy.PoolApi;
using Hyperledger.Indy.Test.Util;
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
            await Pool.SetProtocolVersionAsync(PoolUtils.PROTOCOL_VERSION);

            poolName = PoolUtils.CreatePoolLedgerConfig();
            pool = await Pool.OpenPoolLedgerAsync(poolName, null);

            await WalletUtils.CreateWallet(WALLET, WALLET_KEY);
            wallet = await WalletUtils.OpenWallet(WALLET, WALLET_KEY);
        }

        [TestCleanup]
        public async Task DeleteWallet()
        {
            await pool.CloseAsync();
            await wallet.CloseAsync();
            await WalletUtils.DeleteWallet(WALLET, WALLET_KEY);
        }
    }
}
