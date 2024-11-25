import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "./../target/types/smartwallet";
import idl from "./../target/idl/smartwallet.json";
import { toBigInt } from "ethers";
const { Keypair, ConfirmOptions, PublicKey } = require("@solana/web3.js");
const secp256k1 = require('secp256k1');
const { ethers, utils } = require("ethers");

async function main() {
  // solana provider & wallet
  const url = anchor.web3.clusterApiUrl("devnet");
  console.log("solana url: ", url);
  const connection = new anchor.web3.Connection(url, "confirmed");
  let secretKey = Uint8Array.from([234, 212, 57, 137, 35, 17, 232, 92, 120, 194, 143, 102, 216, 107, 48, 229, 155, 18, 246, 139, 38, 132, 236, 183, 25, 228, 86, 109, 124, 165, 211, 146, 23, 45, 150, 153, 8, 189, 1, 79, 184, 11, 125, 231, 105, 235, 59, 26, 41, 206, 213, 116, 40, 183, 164, 59, 78, 149, 216, 16, 213, 29, 179, 178]);
  let keypair = Keypair.fromSecretKey(secretKey);
  const wallet = new anchor.Wallet(keypair);
  console.log("solana payer address: ", wallet.publicKey.toBase58());
  const privKeyHex = Buffer.from(wallet.payer.secretKey).toString('hex')
  const pubKeyHex = Buffer.from(wallet.publicKey.toBytes()).toString('hex')
  console.log('solana payer public key: ', pubKeyHex)
  console.log('solana payer private key: ', privKeyHex)
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);

  // program
  const programAddress = new PublicKey('C7iFDZry7Kdg7gc829qBCPEPKKdDkf1gRMxNxDEqeYUX');
  let program = anchor.workspace.Smartwallet as Program<Smartwallet>;

  const owner = Buffer.from('024e1e9e0de1a665cbbcaaa4c3e9388e5900adc9ff5b9676f3b973dbe8b2b3aa', 'hex');
  let [configAddress, configBump] = await PublicKey.findProgramAddress(
    ['wallet', 'config', owner],
    programAddress,
  );
  console.log('smart wallet config address: ', configAddress.toBase58())
  console.log('smart wallet config bump: ', configBump)

  let [ownerAddress, ownerBump] = await PublicKey.findProgramAddress(
    ['wallet', 'owner', owner],
    programAddress,
  );
  console.log('smart wallet owner address: ', ownerAddress.toBase58())
  console.log('smart wallet owner bump: ', ownerBump)

  // executed instruction
  //
  const receipt = new PublicKey('2ZUhLpkBwxvZakU5gA7J3pMLnPhfyZZGQY9ArJtFPLqw');
  const transferInstuction = anchor.web3.SystemProgram.transfer({
    fromPubkey: ownerAddress,
    toPubkey: receipt,
    lamports: 1000000,
  })
  transferInstuction.keys[1].isSigner = true;
  let raw = encodeInstruction(transferInstuction);

  // transaction index
  let indexBuffer = Buffer.alloc(8);
  indexBuffer.writeBigInt64LE(toBigInt(0));
  raw = Buffer.concat([raw, indexBuffer]);
  console.log('raw: ', raw.toString('hex'));

  // sign the instruction raw
  const privKey = Buffer.from('6c35d4230c67d604a129d5b9de1cd2667096d93d0b0ec01512d21d1a30fe7676', 'hex');
  const pubKey = Buffer.from('03024e1e9e0de1a665cbbcaaa4c3e9388e5900adc9ff5b9676f3b973dbe8b2b3aa', 'hex');
  const messageHash = ethers.keccak256(raw).slice(2, 66);
  console.log('message hash: ', messageHash);
  const messageHashBytes = Buffer.from(messageHash, 'hex');
  const signObj = secp256k1.ecdsaSign(messageHashBytes, privKey);
  console.log('verify the signature: ', secp256k1.ecdsaVerify(signObj.signature, messageHashBytes, pubKey));

  // transaction
  const tx = await program.methods
    .executeTransaction({
      owner: owner,
      signs: Buffer.from(signObj.signature),
      recoveryId: signObj.recoveryId,
      instructions: [
        {
          accountSize: 2,
          data: transferInstuction.data,
        }
      ],
    })
    .accounts({
      wallet: configAddress,
      payer: wallet.publicKey,
    })
    .remainingAccounts(
      [
        {
          isSigner: false,
          isWritable: false,
          pubkey: anchor.web3.SystemProgram.programId,
        },
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
    .preInstructions([
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 400000,
      })
    ])
    .rpc();
  console.log("transaction signature", tx);
}

function encodeInstruction(a: anchor.web3.TransactionInstruction) {
  console.log("program key: ", a.programId.toBuffer().toString('hex'));
  console.log("data: ", a.data.toString('hex'));
  var i: number;
  for (i = 0; i < a.keys.length; i++) {
    const key = a.keys[i];
    console.log("number: ", i)
    console.log("is signer: ", key.isSigner);
    console.log("is writable: ", key.isWritable);
    console.log("public key: ", key.pubkey);
  }
  //
  const programIdBuffer = Buffer.alloc(32).fill(a.programId.toBuffer());
  let keysBuffer = Buffer.alloc(8);
  keysBuffer.writeBigInt64LE(toBigInt(a.keys.length));
  for (i = 0; i < a.keys.length; i++) {
    const key = a.keys[i];
    let keyBuffer = Buffer.alloc(34).fill(key.pubkey.toBuffer());
    keyBuffer.writeInt8(key.isSigner ? 1 : 0, 32);
    keyBuffer.writeInt8(key.isWritable ? 1 : 0, 33);
    keysBuffer = Buffer.concat([keysBuffer, keyBuffer]);
  }
  let dataBuffer = Buffer.alloc(8 + a.data.length);
  dataBuffer.writeBigInt64LE(toBigInt(a.data.length));
  dataBuffer.fill(a.data, 8);

  return Buffer.concat([programIdBuffer, keysBuffer, dataBuffer]);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
