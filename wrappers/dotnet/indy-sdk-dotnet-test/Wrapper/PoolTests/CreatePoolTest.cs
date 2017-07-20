using Indy.Sdk.Dotnet.Wrapper;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.IO;
using System.Threading.Tasks;

namespace Indy.Sdk.Dotnet.Test.Wrapper.PoolTests
{
    [TestClass]
    public class CreatePoolTest : IndyIntegrationTest
    {
        [TestMethod]
        public void TestCreatePoolWorksForNullConfig()
        {
            var txnFile = "testCreatePoolWorks.txn";

            try
            {
                File.Create(txnFile).Dispose();
                Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", null).Wait();
            }
            finally
            {
                File.Delete(txnFile);
            }
        }

        [TestMethod]
        public void TestCreatePoolWorksForConfigJSON()
        {
            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            Pool.CreatePoolLedgerConfigAsync("testCreatePoolWorks", configJson).Wait();
        }

        [TestMethod]
        public async Task TestCreatePoolWorksForEmptyName()
        {

            var genesisTxnFile = PoolUtils.CreateGenesisTxnFile("genesis.txn");
            var path = Path.GetFullPath(genesisTxnFile).Replace('\\', '/');

            var configJson = string.Format("{{\"genesis_txn\":\"{0}\"}}", path);

            var ex = await Assert.ThrowsExceptionAsync<IndyException>(() =>
                Pool.CreatePoolLedgerConfigAsync("", configJson)
            );

            Assert.AreEqual(ErrorCode.CommonInvalidParam2, ex.ErrorCode);
        }
    }
}
