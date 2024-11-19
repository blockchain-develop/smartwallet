import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "../target/types/smartwallet";

async function main() {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Smartwallet as Program<Smartwallet>;

  const tx = await program.methods.walletCreate(
    {
      owner: bytes(0),

    },
  ).rpc();
  console.log("transaction signature", tx);
}
