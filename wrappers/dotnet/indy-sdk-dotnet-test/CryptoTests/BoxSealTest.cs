using Hyperledger.Indy.CryptoApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Linq;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.CryptoTests
{
    [TestClass]
    public class BoxSealTest : CryptoIntegrationTestBase
    {
        [TestMethod] //Not sure if this is a good test, but since the encrypted content is not static...
        public async Task TestBoxSealWorks()
        {
            var encryptedMessage = await Crypto.BoxSealAsync(recipientVerKey, MESSAGE);

            var decryptedMessage = await Crypto.BoxSealOpenAsync(wallet, recipientVerKey, encryptedMessage);
            Assert.IsTrue(MESSAGE.SequenceEqual(decryptedMessage));
        }
    }
}
