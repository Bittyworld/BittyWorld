// Sample unit test for game logic

describe('Game Logic', () => {
  it('should calculate player health correctly', () => {
    const initialHealth = 100;
    const damage = 30;
    const health = initialHealth - damage;
    expect(health).toBe(70);
  });
}); 