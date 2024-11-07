import * as mainstay from '@project-serum/mainstay';
import { Program } from '@project-serum/mainstay';
import { Raw } from '../target/types/raw';

describe('raw', () => {
  // Configure the client to use the local cluster.
  mainstay.setProvider(mainstay.Provider.env());

  const program = mainstay.workspace.Raw as Program<Raw>;

  it('Is initialized!', async () => {

  });
});
