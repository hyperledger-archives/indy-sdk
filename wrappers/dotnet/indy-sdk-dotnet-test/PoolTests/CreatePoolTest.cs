using Hyperledger.Indy.PoolApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.IO;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PoolTests
{
    [TestClass]
    public class CreatePoolTest : IndyIntegrationTestBase
    {
        [TestMethod]
        public async Task TestCreatePoolWorksForNullConfig()
        {
            var txnFile = "testCreatePoolWorks.txn";

            try
            {
                File.Create(txnFile).Dispose();
                await Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", null);
            }
            finally
            {
                File.Delete(txnFile);
            }
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForConfigJSON()
        {
            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            await Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", configJson);
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForEmptyName()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            var ex = await Assert.ThrowsExceptionAsync<InvalidParameterException>(() =>
                Pool.CreatePoolLedgerConfigAsync("", configJson)
            );

            Assert.AreEqual(2, ex.ParameterIndex);
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForTwice()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");

            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            await Pool.CreatePoolLedgerConfigAsync("pool1", configJson);

            var ex = await Assert.ThrowsExceptionAsync<PoolLedgerConfigExistsException>(() =>
                Pool.CreatePoolLedgerConfigAsync("pool1", configJson)
            );;
        }
    }
}
