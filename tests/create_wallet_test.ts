import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "../target/types/smartwallet";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Smartwallet as Program<Smartwallet>;

  const owner = Buffer.from('1699ede87439081722ba0d5d210d8c4e351cbf98a97ff6c2a770cc9c71abd219', 'hex');
  const tx = await program.methods.walletCreate(
    {
      owner: owner,

    },
  ).rpc();
  console.log("transaction signature", tx);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
