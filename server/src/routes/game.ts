import { Router } from 'express';

const router = Router();

// Create a new game room
router.post('/create', (req, res) => {
  // TODO: Implement room creation logic
  res.json({ success: true, roomId: 'room_' + Date.now() });
});

// Join an existing game room
router.post('/join', (req, res) => {
  // TODO: Implement join logic
  res.json({ success: true, message: 'Joined room' });
});

export default router; 