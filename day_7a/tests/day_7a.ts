import * as anchor from "@coral-xyz/anchor";
import { expect } from 'chai';
import { Program } from "@coral-xyz/anchor";
import { Day7a } from "../target/types/day_7a";

describe("day_7a", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Day7a as Program<Day7a>;

  const counter = anchor.web3.Keypair.generate()

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({ counter: counter.publicKey })
      .signers([counter])
      .rpc()

    const account = await program.account.counter.fetch(counter.publicKey)
    expect(account.count.toNumber() === 0)
  })

  it("Incremented the count", async () => {
    const tx = await program.methods
      .increment()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc()
    
    const account = await program.account.counter.fetch(counter.publicKey)
    expect(account.count.toNumber()).to.equal(1);
  })

  it("Decremented the count", async () => {
    // Set the count to 2
    await program.methods
      .increment()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();
    
    // Decrement the count
    await program.methods
      .decrement()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();

    // Fetch the updated counter account
    const account = await program.account.counter.fetch(counter.publicKey);

    // Assert that the count is 1
    expect(account.count.toNumber()).to.equal(1);
});


});
