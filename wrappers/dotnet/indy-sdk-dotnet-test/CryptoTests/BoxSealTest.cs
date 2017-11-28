using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class BoxSealTest : IndyIntegrationTestBase
    {
        [TestMethod] 
        public async Task TestBoxSealWorks()
        {
            await Crypto.BoxSealAsync(VERKEY_MY1, MESSAGE);
        }

        [TestMethod]
        public async Task TestBoxSealWorksForInvalidKey()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                Crypto.BoxSealAsync(INVALID_VERKEY, MESSAGE)
           );
        }
    }
}
