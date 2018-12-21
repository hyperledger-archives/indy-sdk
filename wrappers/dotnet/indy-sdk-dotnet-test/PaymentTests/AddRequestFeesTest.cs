using Hyperledger.Indy.PaymentsApi;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System.Threading.Tasks;

namespace Hyperledger.Indy.Test.PairwiseTests
{
    [TestClass]
    public class AddRequestFeesTest : PaymentIntegrationTestBase
    {
        [TestMethod]
        public async Task TestAddRequestFeesWorksForUnknownPaymentMethod()
        {
            var ex = await Assert.ThrowsExceptionAsync<UnknownPaymentMethodException>(() =>
                 Payments.AddRequestFeesAsync(wallet, DID_TRUSTEE, emptyObject, inputs, outputs, null)
             );
        }

        [TestMethod]
        public async Task TestAddRequestFeesWorksForEmptyInputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.AddRequestFeesAsync(wallet, DID_TRUSTEE, emptyObject, emptyArray, outputs, null)
             );
        }

        [TestMethod]
        public async Task TestAddRequestFeesWorksForSeveralMethods()
        {
            var ex = await Assert.ThrowsExceptionAsync<IncompatiblePaymentMethodsException>(() =>
                 Payments.AddRequestFeesAsync(wallet, DID_TRUSTEE, emptyObject, incompatibleInputs, emptyObject, null)
             );
        }

        [TestMethod]
        public async Task testAddRequestFeesWorksForInvalidInputs()
        {
            var ex = await Assert.ThrowsExceptionAsync<InvalidStructureException>(() =>
                 Payments.AddRequestFeesAsync(wallet, DID_TRUSTEE, emptyObject, invalidInputs, emptyObject, null)
             );
        }

    }
}
