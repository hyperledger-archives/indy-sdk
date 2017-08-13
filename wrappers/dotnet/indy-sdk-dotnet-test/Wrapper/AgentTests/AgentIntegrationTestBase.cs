using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.AgentTests
{
    public abstract class AgentIntegrationTestBase
    {
        protected static Wallet _wallet;
        protected static Pool _pool;
        protected string _poolName;
        private string _walletName = "agentWallet";

        [TestInitialize]
        public async Task SetUp()
        {
            await InitHelper.InitAsync();
            StorageUtils.CleanupStorage();

            _poolName = PoolUtils.CreatePoolLedgerConfig();

            var config2 = "{}";
            _pool = await Pool.OpenPoolLedgerAsync(_poolName, config2);

            await Wallet.CreateWalletAsync(_poolName, _walletName, "default", null, null);
            _wallet = await Wallet.OpenWalletAsync(_walletName, null, null);
        }

        [TestCleanup]
        public async Task TearDown()
        {
            if(_wallet != null)
                await _wallet.CloseAsync();

            await Wallet.DeleteWalletAsync(_walletName, null);

            if(_pool != null)
                await _pool.CloseAsync();

            StorageUtils.CleanupStorage();
        }        
    }
}
