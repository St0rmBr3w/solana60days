import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day21 } from "../target/types/day_21";

describe("balance", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day21 as Program<Day21>;

  let pubkey = new anchor.web3.PublicKey("6wvvubC7LhyLtXcYmgHfhPtR2ZrfvuXyTaNStX96VQC2");

  it("It tests balance!", async () => {
    // Add your test here.
    const tx = await program.methods.readBalance().accounts({ acct: pubkey }).rpc();
    console.log("Your transaction signature", tx);
  });
});
