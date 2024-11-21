import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "./../target/types/smartwallet";
import idl from "./../target/idl/smartwallet.json";
const { Keypair, ConfirmOptions, PublicKey, Secp256k1Program, Transaction } = require("@solana/web3.js");
const secp256k1 = require('secp256k1');
const { keccak_256 } = require("js-sha3");

async function main() {
  const url = anchor.web3.clusterApiUrl("devnet");
  console.log("url: ", url);
  const connection = new anchor.web3.Connection(url, "confirmed");
  let secretKey = Uint8Array.from([234, 212, 57, 137, 35, 17, 232, 92, 120, 194, 143, 102, 216, 107, 48, 229, 155, 18, 246, 139, 38, 132, 236, 183, 25, 228, 86, 109, 124, 165, 211, 146, 23, 45, 150, 153, 8, 189, 1, 79, 184, 11, 125, 231, 105, 235, 59, 26, 41, 206, 213, 116, 40, 183, 164, 59, 78, 149, 216, 16, 213, 29, 179, 178]);
  let keypair = Keypair.fromSecretKey(secretKey);
  const wallet = new anchor.Wallet(keypair);
  console.log("wallet address: ", wallet.publicKey.toBase58());
  const privKeyHex = Buffer.from(wallet.payer.secretKey).toString('hex')
  const pubKeyHex = Buffer.from(wallet.publicKey.toBytes()).toString('hex')
  console.log('public key: ', pubKeyHex)
  console.log('private key: ', privKeyHex)
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  //
  let secp256k1PrivateKey;
  do {
    secp256k1PrivateKey = Keypair.generate().secretKey.slice(0, 32);
  } while (!secp256k1.privateKeyVerify(secp256k1PrivateKey));

  let secp256k1PublicKey = secp256k1.publicKeyCreate(secp256k1PrivateKey, false).slice(1);
  let ethAddress = Secp256k1Program.publicKeyToEthAddress(secp256k1PublicKey);

  console.log('ethereum address: ', ethAddress.toString('hex'));

  let msg = Buffer.from("hello");
  let msgHash = Buffer.from(keccak_256.update(msg).digest());
  let { signature, recid: recoverId } = secp256k1.ecdsaSign(msgHash, secp256k1PrivateKey);

  console.log('msg: ', msg);
  console.log('signature: ', signature);
  console.log('recover id: ', recoverId);

  let transaction = new Transaction().add(
    Secp256k1Program.createInstructionWithEthAddress({
      ethAddress: ethAddress.toString('hex'),
      message: msg,
      signature: signature,
      recoveryId: recoverId,
    }),
  );

  const tx = await provider.sendAndConfirm(transaction);
  console.log("transaction signature", tx);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
