import org.hyperledger.indy.sdk.crypto.CryptoResults;
import org.hyperledger.indy.sdk.did.Did;
import org.hyperledger.indy.sdk.did.DidResults;
import org.hyperledger.indy.sdk.wallet.Wallet;
import org.hyperledger.indy.sdk.crypto.Crypto;

import java.io.*;
import java.util.Arrays;
import java.util.Scanner;
import java.nio.file.Files;


public class MessageEncryption {
	Wallet walletHandle;
	String myDID;
	String myVerkey;
	String theirVerkey;
	String theirDID;

	void init() throws Exception {
		Scanner input = new Scanner(System.in);
		System.out.println("Who are you? ");
		String line = input.nextLine();
		String walletName = line + "-wallet";
		Wallet.createWallet("pool", walletName, null, null, null).get();
		this.walletHandle = Wallet.openWallet(walletName, null, null).get();

		DidResults.CreateAndStoreMyDidResult res = Did.createAndStoreMyDid(this.walletHandle, "{}").get();
		this.myVerkey = res.getVerkey();
		this.myDID = res.getDid();
		System.out.println("My DID and Verkey: " + this.myDID + " " + this.myVerkey);

		System.out.println("Other party's DID and Verkey? ");
		line = input.nextLine().trim();
		String []parts = line.split(" ");
		this.theirDID = parts[0];
		this.theirVerkey = parts[1];
	}

	private void prep(String message) throws Exception {
		byte[] binaryMessage = message.getBytes();
		byte[] encrypted = Crypto.authCrypt(this.walletHandle, this.myVerkey, this.theirVerkey, binaryMessage).get();
		File file = new File("encrypted");
		FileOutputStream out = new FileOutputStream(file);
		out.write(encrypted);
		System.out.println("Encrypted length: " + encrypted.length);
		out.close();
	}

	private void read() throws Exception {
		File file = new File("encrypted");
		byte[] encrypted = Files.readAllBytes(file.toPath());
		System.out.println("Encrypted length: " + encrypted.length);
		CryptoResults.AuthDecryptResult res = Crypto.authDecrypt(this.walletHandle, this.myVerkey, encrypted).get();
		System.out.println("Their Verkey: " + res.getVerkey());
		System.out.println("Decrypted message: " + new String(res.getDecryptedMessage()));
	}

	void demo() throws Exception {
		this.init();

		Scanner input = new Scanner(System.in);

		while (true) {
			System.out.print("> ");
			String []args = input.nextLine().trim().split(" ");
			String command = args[0];
			String rest;
			if (args.length > 1) {
				rest = String.join(" ", Arrays.copyOfRange(args, 1, args.length));
			} else {
				rest = "";
			}

			if (command.equals("prep")) {
				this.prep(rest);
			} else if (command.equals("read")) {
				this.read();
			} else if (command.equals("quit")) {
				break;
			} else {
				System.out.println("Huh?");
			}
		}
	}
}
