import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";

describe("Calculator", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const program = anchor.workspace.Calculator as Program<Calculator>;

    // Initialize test
    it("is initialized!", async () => {
        // ...
    });

    it("Protects underflow", async () => {
        // ...
    });

    // Tests for addition
    it("Addition test", async () => {
        const a = new anchor.BN(10);
        const b = new anchor.BN(5);
        const tx = await program.methods.add(a, b).rpc();
        console.log("Your transaction signature for addition", tx);
    });

    // Tests for multiplication
    it("Multiplication test", async () => {
        const a = new anchor.BN(10);
        const b = new anchor.BN(5);
        const tx = await program.methods.multiply(a, b).rpc();
        console.log("Your transaction signature for multiplication", tx);
    });

    // Tests for multiplication
    it("Subtraction test", async () => {
        const a = new anchor.BN(10);
        const b = new anchor.BN(5);
        const tx = await program.methods.subtract(a, b).rpc();
        console.log("Your transaction signature for subtraction", tx);
    });

    // Tests for division
    it("Division test", async () => {
        const a = new anchor.BN(10);
        const b = new anchor.BN(5);
        const tx = await program.methods.divide(a, b).rpc();
        console.log("Your transaction signature for division", tx);
    });

    // Tests for square root
    it("Sqrt test", async () => {
        const a = new anchor.BN(10);
        const tx = await program.methods.sqrt(a).rpc();
        console.log("Your transaction signature for square root", tx);
    });

    // Tests for log10
    it("Log base 10 test", async () => {
        const a = new anchor.BN(5);
        const tx = await program.methods.log10(a).rpc();
        console.log("Your transaction signature for log 10", tx);
    })


});