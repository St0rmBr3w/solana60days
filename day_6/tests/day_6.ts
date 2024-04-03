import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day6 } from "../target/types/day_6";

describe("day_6", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day6 as Program<Day6>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("name", "Bob").rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is constant!", async () => {
    // Add your test here.
    const tx = await program.methods.constants().rpc();
    console.log("Your transaction signature", tx);
  })

  it("Age checker if else!", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerIfElse(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Age checker ternary!", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerTernary(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Age checker match!", async () => {
    // Add your test here.
    const tx = await program.methods.ageCheckerMatch(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("For loops!", async () => {
    // Add your test here.
    const tx = await program.methods.forLoops().rpc();
    console.log("Your transaction signature", tx);
  })

  it("Fixed array!", async () => {
    // Add your test here.
    const tx = await program.methods.fixedArray(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Dynamic array!", async () => {
    // Add your test here.
    const tx = await program.methods.dynamicArray(new anchor.BN(35)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Structs!", async () => {
    // Add your test here.
    const tx = await program.methods.structs("Alice", new anchor.BN(20)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Usizes!", async () => {
    // Add your test here.
    const tx = await program.methods.uSizing().rpc();
    console.log("Your transaction signature", tx);
  })

  it("Even vectors!", async () => {
    // Add your test here.
    const tx = await program.methods.evenVector([new anchor.BN(20), new anchor.BN(23), new anchor.BN(32)]).rpc();
    console.log("Your transaction signature", tx);
  })
});
