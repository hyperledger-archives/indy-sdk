using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test
{
    public class IndyIntegrationTest
    {
        protected HashSet<Pool> openedPools = new HashSet<Pool>();

        [TestInitialize]
        public void SetUp()
        {
            InitHelper.Init();
            StorageUtils.CleanupStorage();
        }

        [TestCleanup]
        public async Task TearDown()
        {
            foreach (var pool in openedPools)
            {
                try
                {
                    await pool.CloseAsync();
                }
                catch (IndyException)
                { }
            }

            openedPools.Clear();
            StorageUtils.CleanupStorage();
        }       
    }
}
