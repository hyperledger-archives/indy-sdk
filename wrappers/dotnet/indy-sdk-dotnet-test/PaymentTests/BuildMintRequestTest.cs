using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildMintRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildMintRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildMintRequestAsync(wallet, DID_TRUSTEE, outputs, null)
             );
        }

        [TestMethod]
        public async Task TestBuildMintRequestWorksForEmptyOutputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.BuildMintRequestAsync(wallet, DID_TRUSTEE, emptyArray, null)
             );
        }

        [TestMethod]
        public async Task TestBuildMintRequestWorksForIncompatiblePaymentMethods()
        {
            var ex = await Assert.ThrowsExceptionAsync<IncompatiblePaymentMethodsException>(() =>
                 Payments.BuildMintRequestAsync(wallet, DID_TRUSTEE, incompatibleOutputs, null)
             );
        }
    }
}
