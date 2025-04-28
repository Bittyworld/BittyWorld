import express from 'express';
import http from 'http';
import { Server as SocketIOServer } from 'socket.io';
import healthRouter from './routes/health';

const app = express();
const server = http.createServer(app);
const io = new SocketIOServer(server, { cors: { origin: '*' } });

app.use('/api/health', healthRouter);

io.on('connection', (socket) => {
  console.log('A user connected:', socket.id);
  socket.on('disconnect', () => {
    console.log('User disconnected:', socket.id);
  });
});

const PORT = process.env.API_PORT || 4000;
server.listen(PORT, () => {
  console.log(`BittyWorld server running on port ${PORT}`);
}); 