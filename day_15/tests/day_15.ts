import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day15 } from "../target/types/day_15";

describe("compute_unit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day15 as Program<Day15>;
  const defaultKeyPair = new anchor.web3.PublicKey(
    "6wvvubC7LhyLtXcYmgHfhPtR2ZrfvuXyTaNStX96VQC2"
  );

  it("Is initialized!", async () => {
    // Add your test here.
    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before:", bal_before);

    const tx = await program.methods.initialize().rpc();

    let bal_after = await program.provider.connection.getBalance(defaultKeyPair);
    console.log("after:", bal_after);

    // Log the difference
    console.log(
      "diff:",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );
  });
});
