import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
    .initialize(new anchor.BN(888), new anchor.BN(777), "hello").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Protects overflow", async () => {
    try {
      const tx = await program.rpc.initialize(new anchor.BN(0), new anchor.BN(1), "hello");
      console.log("Your transaction signature", tx);
      // Add an assertion here if necessary to verify specific behavior
    } catch (error) {
      console.error("Error occurred:", error);
      // Handle the error or check the error message to ensure it's the expected overflow error
    }
  });

  it("Array and power test", async () => {
    const tx = await program.methods.array([new anchor.BN(777),
    new anchor.BN(888)]).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Cube root test", async () => {
    const tx = await program.methods.cube([new anchor.BN(125)]).rpc();
    console.log("Your transaction", tx);
  })
});
