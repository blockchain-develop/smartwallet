import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "../target/types/smartwallet";
const secp256k1 = require('secp256k1')
import { randomBytes } from "crypto"

async function main() {
  let privKey
  do {
    privKey = randomBytes(32)
  } while (!secp256k1.privateKeyVerify(privKey))
  const pubKey = secp256k1.publicKeyCreate(privKey)

  const privKeyHex = Buffer.from(privKey).toString('hex')
  const pubKeyHex = Buffer.from(pubKey).toString('hex')

  console.log('public key: ', pubKeyHex)
  console.log('private key: ', privKeyHex)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
