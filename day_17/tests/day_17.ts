import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day17 } from "../target/types/day_17";
import { assert } from "chai";

describe('Day17', () => {
  // Configure the client to use the local cluster and setup the provider and program.
  anchor.setProvider(anchor.AnchorProvider.local());

  const program = anchor.workspace.Day17 as Program<Day17>;

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
    await program.methods.printX().accounts({myStorage: myStorage}).rpc();

    // Fetch the account to verify initialization
    const storageAccount = await program.account.myStorage.fetch(myStorage.toBase58());
    console.log("Account initialized with x value:", storageAccount.x.toString());
    // Add an assertion here if there's a default or expected state after initialization
});

  it("Sets a value!", async () => {
    // Add your tests here.
    const seeds = [];

    // Solana requires us to specify in advance the accounts a transaction will interact with. 
    // Since we are interacting with the account that stores MyStruct, we need to compute its 
    // “address” in advance and pass it to the initialize() function.
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    await program.methods.set(new anchor.BN("170")).accounts({ myStorage: myStorage }).rpc();
    // Fetch the updated myStorage account
    const storageAccount = await program.account.myStorage.fetch(myStorage.toBase58());
    assert.equal(storageAccount.x.toString(), '170');
    console.log("Account set with x value:", storageAccount.x.toString());
  });

  it("Increments a value!", async () => {
    // Add your tests here.
    const seeds = [];

    // Solana requires us to specify in advance the accounts a transaction will interact with. 
    // Since we are interacting with the account that stores MyStruct, we need to compute its 
    // “address” in advance and pass it to the initialize() function.
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    await program.methods.set(new anchor.BN("170")).accounts({ myStorage: myStorage }).rpc();
    await program.methods.increment().accounts({ myStorage: myStorage }).rpc();
    // Fetch the updated myStorage account
    const storageAccount = await program.account.myStorage.fetch(myStorage.toBase58());
    assert.equal(storageAccount.x.toString(), '171');
    console.log("Account with x value incremented:", storageAccount.x.toString());
  });

});
