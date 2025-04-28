import * as anchor from '@project-serum/anchor';

describe('bittyworld', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it('Can initialize the game state', async () => {
    // TODO: Add real test logic
    expect(true).toBe(true);
  });
}); 