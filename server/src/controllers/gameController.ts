// Placeholder for game business logic

export function createRoom() {
  // TODO: Implement room creation logic
  return { roomId: 'room_' + Date.now() };
}

export function joinRoom(roomId: string) {
  // TODO: Implement join logic
  return { success: true, roomId };
} 