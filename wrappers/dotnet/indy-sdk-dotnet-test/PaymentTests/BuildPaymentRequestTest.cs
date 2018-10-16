using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class BuildPaymentRequestTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestBuildPaymentRequestWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.BuildPaymentRequestAsync(wallet, DID_TRUSTEE, inputs, outputs, null)
             );
        }

        [TestMethod]
        public async Task TestBuildPaymentRequestWorksForEmptyInputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.BuildPaymentRequestAsync(wallet, DID_TRUSTEE, emptyArray, outputs, null)
             );
        }

        [TestMethod]
        public async Task TestBuildPaymentRequestWorksForEmptyOutputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.BuildPaymentRequestAsync(wallet, DID_TRUSTEE, inputs, emptyObject, null)
             );
        }

        [TestMethod]
        public async Task TestBuildPaymentRequestWorksForIncompatiblePaymentMethods()
        {
            var ex = await Assert.ThrowsExceptionAsync<IncompatiblePaymentMethodsException>(() =>
                 Payments.BuildPaymentRequestAsync(wallet, DID_TRUSTEE, incompatibleInputs, emptyObject, null)
             );
        }

        [TestMethod]
        public async Task TestBuildPaymentRequestWorksForInvalidInputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.BuildPaymentRequestAsync(wallet, DID_TRUSTEE, invalidInputs, outputs, null)
             );
        }
    }
}
