const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

async function createUser(program, provider) {
  const userAccount = anchor.web3.Keypair.generate();

  const name = "Pandit Dhamdhere";
  const avatar =
    "https://avatars.githubusercontent.com/u/85685981?v=4";

  await program.rpc.signupUser(name, avatar, {
    accounts: {
      authority: provider.wallet.publicKey,
      userAccount: userAccount.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [userAccount],
  });

  const user = await program.account.userState.fetch(userAccount.publicKey);
  return { user, userAccount, name, avatar };
}

module.exports = {
  createUser,
};
