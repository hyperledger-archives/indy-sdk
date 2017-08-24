using Indy.Sdk.Dotnet.Test.Wrapper.WalletTests;
using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    public abstract class IndyIntegrationTestBase
    {
        protected HashSet<Pool> _openedPools = new HashSet<Pool>();
        
        [TestInitialize]
        public async Task SetUp()
        {
            await InitHelper.InitAsync();
            StorageUtils.CleanupStorage();
        }

        [TestCleanup]
        public async Task TearDown()
        {
            foreach (var pool in _openedPools)
            {
                if(pool != null)
                    await pool.CloseAsync();
            }

            _openedPools.Clear();
            StorageUtils.CleanupStorage();
        }       
    }
}
