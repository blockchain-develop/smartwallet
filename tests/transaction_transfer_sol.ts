import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "./../target/types/smartwallet";
import idl from "./../target/idl/smartwallet.json";
const { Keypair, ConfirmOptions, PublicKey } = require("@solana/web3.js");
const secp256k1 = require('secp256k1');
const { ethers, utils } = require("ethers");

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
  const programAddress = new PublicKey('C7iFDZry7Kdg7gc829qBCPEPKKdDkf1gRMxNxDEqeYUX');
  //const program = new Program(idl as Smartwallet, programAddress, provider);
  let program = anchor.workspace.Smartwallet as Program<Smartwallet>;
  //console.log("program", program)

  const owner = Buffer.from('024e1e9e0de1a665cbbcaaa4c3e9388e5900adc9ff5b9676f3b973dbe8b2b3aa', 'hex');
  const sysvarProgarmAddress = new PublicKey('Sysvar1nstructions1111111111111111111111111');

  let [smartWalletAddress, bump] = await PublicKey.findProgramAddress(
    ['wallet', 'config', owner],
    programAddress,
  );
  console.log('smart wallet address: ', smartWalletAddress.toBase58())
  console.log('bump: ', bump)

  let [ownerAddress, bump1] = await PublicKey.findProgramAddress(
    ['wallet', 'owner', owner],
    programAddress,
  );
  console.log('wallet owner address: ', ownerAddress.toBase58())
  console.log('bump: ', bump1)
  //
  const receipt = new PublicKey('2ZUhLpkBwxvZakU5gA7J3pMLnPhfyZZGQY9ArJtFPLqw');
  const transferInstuction = anchor.web3.SystemProgram.transfer({
    fromPubkey: ownerAddress,
    toPubkey: receipt,
    lamports: 1000000,
  })
  //
  const privKey = Buffer.from('6c35d4230c67d604a129d5b9de1cd2667096d93d0b0ec01512d21d1a30fe7676', 'hex');
  const pubKey = Buffer.from('03024e1e9e0de1a665cbbcaaa4c3e9388e5900adc9ff5b9676f3b973dbe8b2b3aa', 'hex');
  const messageHash = ethers.keccak256(transferInstuction.data).slice(2, 66);
  console.log('message hash: ', messageHash);
  const messageHashBytes = Buffer.from(messageHash, 'hex');
  //const hash = Uint8Array.from(messageHashBytes)
  const signObj = secp256k1.ecdsaSign(messageHashBytes, privKey);
  console.log(secp256k1.ecdsaVerify(signObj.signature, messageHashBytes, pubKey));
  //
  const tx = await program.methods
    .executeTransaction({
      owner: owner,
      signs: Buffer.from(signObj.signature),
      data: transferInstuction.data,
    })
    .accounts({
      wallet: smartWalletAddress,
      payer: wallet.publicKey,
      ixSysvar: sysvarProgarmAddress,
      program: anchor.web3.SystemProgram.programId,
    })
    .remainingAccounts(
      //transferInstuction.keys,
      [
        {
          isSigner: false,
          isWritable: true,
          pubkey: ownerAddress,
        },
        {
          isSigner: false,
          isWritable: true,
          pubkey: receipt,
        }
      ]
    )
    .signers([wallet.payer])
    .rpc();
  console.log("transaction signature", tx);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
