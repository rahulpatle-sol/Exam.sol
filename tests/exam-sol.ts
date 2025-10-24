import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ExamSol } from "../target/types/exam_sol";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

describe("exam_sol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ExamSol as Program<ExamSol>;

  const mintAuthority = anchor.web3.Keypair.generate();
  let mint = null;

  it("Initializes an exam", async () => {
    const exam = anchor.web3.Keypair.generate();

    await program.methods
      .initializeExam("Math", 40)
      .accounts({
        exam: exam.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([exam])
      .rpc();

    const examData = await program.account.exam.fetch(exam.publicKey);
    console.log("✅ Exam created:", examData.subject, "Pass mark:", examData.passMark);
  });

  it("Submits result and mints token if passed", async () => {
    const exam = anchor.web3.Keypair.generate();
    const result = anchor.web3.Keypair.generate();

    // Create mint
    mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      mintAuthority.publicKey,
      null,
      0
    );

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      provider.wallet.payer,
      mint,
      provider.wallet.publicKey
    );

    // Initialize exam
    await program.methods
      .initializeExam("Science", 50)
      .accounts({
        exam: exam.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([exam])
      .rpc();

    // Submit result
    await program.methods
      .submitResult(80)
      .accounts({
        student: provider.wallet.publicKey,
        result: result.publicKey,
        exam: exam.publicKey,
        mint,
        tokenAccount: tokenAccount.address,
        mintAuthority: mintAuthority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([result])
      .rpc();

    const resultData = await program.account.resultRecord.fetch(result.publicKey);
    console.log("🎓 Student:", resultData.student.toBase58());
    console.log("📊 Score:", resultData.score);
    console.log("✅ Passed:", resultData.passed);
  });
});
