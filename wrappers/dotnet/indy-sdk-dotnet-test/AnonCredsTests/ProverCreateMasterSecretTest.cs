using Hyperledger.Indy.AnonCredsApi;
using Hyperledger.Indy.Test.Util;
using Hyperledger.Indy.WalletApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.AnonCredsTests
{
    [TestClass]
    public class ProverCreateMasterSecretTest : AnonCredsIntegrationTestBase
    {
        [TestMethod]
        public async Task TestProverCreateMasterSecretWorks()
        {
        }

        [TestMethod] 
        public async Task TestProverCreateMasterSecretWorksForDuplicate()
        {
            var ex = await Assert.ThrowsExceptionAsync<DuplicateMasterSecretNameException>(() =>
               AnonCreds.ProverCreateMasterSecretAsync(wallet, masterSecretId)
           );
        }

        [TestMethod]
        public async Task TestProverCreateMasterSecretWorksForEmptyName()
        {
            await AnonCreds.ProverCreateMasterSecretAsync(wallet, null);
        }
    }
}
