using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    public class IndyIntegrationTest
    {
        protected HashSet<Pool> _openedPools = new HashSet<Pool>();

        [TestInitialize]
        public void SetUp()
        {
            InitHelper.Init();
            StorageUtils.CleanupStorage();
        }

        [TestCleanup]
        public async Task TearDown()
        {
            foreach (var pool in _openedPools)
            {
                try
                {
                    await pool.CloseAsync();
                }
                catch (IndyException)
                { }
            }

            _openedPools.Clear();
            StorageUtils.CleanupStorage();
        }       
    }
}
