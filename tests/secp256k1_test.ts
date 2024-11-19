import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "../target/types/smartwallet";
const secp256k1 = require('secp256k1')
import { randomBytes } from "crypto"

async function main() {
  let privKey
  do {
    privKey = randomBytes(32)
  } while (!secp256k1.priveteKeyVerify(privKey))

  const pubKey = secp256k1.pubKeyCreate(privKey)

  console.log(privKey, pubKey)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
