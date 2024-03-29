import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day3 } from "../target/types/day_3";

const assert = require('assert');

describe("day_3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day3 as Program<Day3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.boatyMcBoatface(new anchor.BN(777)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Does addition!", async () => {
    // Add your test here.
    const tx = await program.methods.add(new anchor.BN(777), new anchor.BN(444)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Does subtraction!", async () => {
    // Add your test here.
    const tx = await program.methods.sub(new anchor.BN(777), new anchor.BN(444)).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Does subtraction without overflow!", async () => {
    try {
      // Add your test here.
      const tx = await program.methods.sub(new anchor.BN(777), new anchor.BN(888)).rpc();
      console.log("Your transaction signature", tx);
      assert.fail("The transaction should have failed but didn't.");
    } catch (err) {
      const errorString = err.toString();
      const containsOverflowError = errorString.includes("attempt to subtract with overflow") || (err.logs && err.logs.some(log => log.includes("attempt to subtract with overflow")));
      assert.ok(containsOverflowError, "Error does not contain the expected 'attempt to subtract with overflow' message");
    }
  });

  it("Does multiplication!", async () => {
    // Add your test here.
    const tx = await program.methods.mul(new anchor.BN(777), new anchor.BN(444)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Does division!", async () => {
    // Add your test here.
    const tx = await program.methods.div(new anchor.BN(777), new anchor.BN(444)).rpc();
    console.log("Your transaction signature", tx);
  })

  it("Does modulus!", async () => {
    // Add your test here.
    const tx = await program.methods.modulus(new anchor.BN(777), new anchor.BN(444)).rpc();
    console.log("Your transaction signature", tx);
  })
});
