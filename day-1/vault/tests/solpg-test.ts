describe("vault", () => {
  
  it("Full Test: Initialize, Deposit, Withdraw", async () => {
    // ---------------------------------------------------------
    // 1. SIMPLE ASSERT HELPER (Fixes 'require' error)
    // ---------------------------------------------------------
    const assert = {
      ok: (condition, message) => {
        if (!condition) throw new Error("ASSERT FAILED: " + (message || "Unknown error"));
      },
      equal: (actual, expected, message) => {
        if (actual !== expected) {
          throw new Error(`ASSERT FAILED: ${message || ""} (Expected ${expected}, got ${actual})`);
        }
      }
    };

    // ---------------------------------------------------------
    // 2. SETUP
    // ---------------------------------------------------------
    const program = pg.program;
    const user = pg.wallet.keypair;
    
    // Random seed to ensure unique PDA every time
    const uniqueSeed = new BN(Math.floor(Math.random() * 1000000));

    // Derive PDA
    const [vaultPda, vaultBump] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"), 
        user.publicKey.toBuffer(),
        uniqueSeed.toArrayLike(Buffer, "le", 8)
      ],
      program.programId
    );

    console.log("Testing with Seed:", uniqueSeed.toString());

    // ---------------------------------------------------------
    // 3. INITIALIZE
    // ---------------------------------------------------------
    const txInit = await program.methods
      .initialize(uniqueSeed) 
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    console.log("Initialized! Tx:", txInit);

    // Verify Initialized State
    const vaultAccount = await program.account.vault.fetch(vaultPda);
    assert.ok(vaultAccount.owner.equals(user.publicKey), "Owner mismatch!");
    
    // ---------------------------------------------------------
    // 4. DEPOSIT
    // ---------------------------------------------------------
    const depositAmount = new BN(0.1 * web3.LAMPORTS_PER_SOL);
const preBalanceDep = await pg.connection.getBalance(vaultPda);

    await program.methods
      .deposit(uniqueSeed, depositAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalanceDep = await pg.connection.getBalance(vaultPda);
    
    // Assert Balance
    assert.equal(
        postBalanceDep, 
        preBalanceDep + depositAmount.toNumber(), 
        "Deposit Failed: Balance didn't update correctly"
    );
    console.log("Deposit Successful!");

    // ---------------------------------------------------------
    // 5. WITHDRAW
    // ---------------------------------------------------------
    const withdrawAmount = new BN(0.05 * web3.LAMPORTS_PER_SOL);
   const preBalanceWith = await pg.connection.getBalance(vaultPda);

    await program.methods
      .withdraw(uniqueSeed, withdrawAmount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const postBalanceWith = await pg.connection.getBalance(vaultPda);
    
    // Assert Balance
    assert.equal(
        postBalanceWith, 
        preBalanceWith - withdrawAmount.toNumber(), 
        "Withdraw Failed: Balance didn't update correctly"
    );
    console.log("Withdraw Successful!");
  });
});
