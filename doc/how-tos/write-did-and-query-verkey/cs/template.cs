

/*
Example demonstrating how to add DID with the role of Trust Anchor as Steward.

Uses seed to obtain Steward's DID which already exists on the ledger.
Then it generates new DID/Verkey pair for Trust Anchor.
Using Steward's DID, NYM transaction request is built to add Trust Anchor's DID and Verkey
on the ledger with the role of Trust Anchor.
Once the NYM is successfully written on the ledger, it generates new DID/Verkey pair that represents
a client, which are used to create GET_NYM request to query the ledger and confirm Trust Anchor's Verkey.

For the sake of simplicity, a single wallet is used. In the real world scenario, three different wallets
would be used and DIDs would be exchanged using some channel of communication
*/


public class WriteDIDAndQueryVerkey
{
    public static void Main(string[] args)
    {

    }
}
