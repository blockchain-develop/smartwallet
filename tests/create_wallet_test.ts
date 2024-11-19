import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartwallet } from "./../target/types/smartwallet";
const { Keypair, ConfirmOptions, PublicKey } = require("@solana/web3.js");

async function main() {
  const url = anchor.web3.clusterApiUrl("devnet");
  console.log("url: ", url);
  const connection = new anchor.web3.Connection(url, "confirmed");
  let seed = Uint8Array.from([234, 212, 57, 137, 35, 17, 232, 92, 120, 194, 143, 102, 216, 107, 48, 229, 155, 18, 246, 139, 38, 132, 236, 183, 25, 228, 86, 109, 124, 165, 211, 146, 23, 45, 150, 153, 8, 189, 1, 79, 184, 11, 125, 231, 105, 235, 59, 26, 41, 206, 213, 116, 40, 183, 164, 59, 78, 149, 216, 16, 213, 29, 179, 178]);
  let account = Keypair.fromSeed(seed.subarray(0, 32));
  const wallet = new anchor.Wallet(account);
  console.log("wallet address: ", wallet.publicKey.toBase58());
  const provider = new anchor.AnchorProvider(connection, wallet, {});
  anchor.setProvider(provider);
  const program = anchor.workspace.Smartwallet as Program<Smartwallet>;

  const owner = Buffer.from('1699ede87439081722ba0d5d210d8c4e351cbf98a97ff6c2a770cc9c71abd219', 'hex');
  const systemProgarmAddress = anchor.web3.SystemProgram.programId;
  const programAddress = new PublicKey('ye56RjR2rNTtj9Fw1TmVpefmV3YCup3sWtCzKi3v22R');

  let smartWalletAddress, _ = await PublicKey.findProgramAddress(
    ['wallet', 'wallet', owner],
    programAddress,
  );
  const tx = await program.methods
    .walletCreate({
      owner: owner,
    })
    .accounts({
      wallet: smartWalletAddress,
      creator: wallet.publicKey,
      systemProgram: systemProgarmAddress,
    })
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
