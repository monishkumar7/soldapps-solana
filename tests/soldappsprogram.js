const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log('🚀 Starting test ...');

  // create and set provider
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Soldappsprogram;
  // create an account pair for our program to use
  const baseAccount = anchor.web3.Keypair.generate();
  console.log(baseAccount.publicKey.toString());

  // call start_stuff_off and pass it the params it needs
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    },
    signers: [baseAccount]
  });

  console.log('🗒️  Your transaction signature - ', tx);

  // fetch data from the account
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('📖 Project Count - ', account.totalProjects.toString());

  await program.rpc.addProject(
    'SolDApps',
    'One DApp to list them all',
    'https://soldapps.xyz',
    {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey
      }
    }
  );

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('📖 Project Count - ', account.totalProjects.toString());

  console.log('📖 Project List - ', account.projectList);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
