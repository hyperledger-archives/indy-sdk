using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    public abstract class IndyIntegrationTestBase
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
