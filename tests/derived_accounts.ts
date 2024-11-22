import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "./../target/types/smartwallet";
import idl from "./../target/idl/smartwallet.json";
const { Keypair, ConfirmOptions, PublicKey } = require("@solana/web3.js");

async function main() {
  const programAddress = new PublicKey('C7iFDZry7Kdg7gc829qBCPEPKKdDkf1gRMxNxDEqeYUX');
  const owner = Buffer.from('024e1e9e0de1a665cbbcaaa4c3e9388e5900adc9ff5b9676f3b973dbe8b2b3aa', 'hex');
  const [walletConfigAddress, bump1] = await PublicKey.findProgramAddress(["wallet", "config", owner], programAddress);
  const [walletOwnerAddress, bump2] = await PublicKey.findProgramAddress(["wallet", "owner", owner], programAddress);

  console.log("wallet config address: ", walletConfigAddress.toBase58(), "bump", bump1);
  console.log("wallet owner address: ", walletOwnerAddress.toBase58(), "bump", bump2);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
