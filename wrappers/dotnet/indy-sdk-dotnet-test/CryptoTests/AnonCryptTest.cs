using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class AnonCryptTest : IndyIntegrationTestBase
    {
        [TestMethod] 
        public async Task TestPrepAnonymousMsgWorks()
        {
            byte[] encryptedMsg = await Crypto.AnonCryptAsync(VERKEY_MY1, MESSAGE);
            Assert.IsNotNull(encryptedMsg);
        }

        [TestMethod]
        public async Task TestPrepAnonymousMsgWorksForInvalidRecipientVk()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                  Crypto.AnonCryptAsync(INVALID_VERKEY, MESSAGE)
           );
        }
    }
}
