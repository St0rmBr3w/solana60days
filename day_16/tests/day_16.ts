import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day16 } from "../target/types/day_16";

describe("day_16", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day16 as Program<Day16>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seeds = [];

    // Solana requires us to specify in advance the accounts a transaction will interact with. 
    // Since we are interacting with the account that stores MyStruct, we need to compute its 
    // “address” in advance and pass it to the initialize() function.
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("the storage account address is", myStorage.toBase58());
    await program.methods.initialize().accounts({ myStorage: myStorage }).rpc();
  });
});
