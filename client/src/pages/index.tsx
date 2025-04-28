import React from 'react';
import Image from 'next/image';
import HelloWorld from '../components/HelloWorld';

export default function Home() {
  return (
    <div style={{ textAlign: 'center', marginTop: 40 }}>
      <Image src="/Logo.png" alt="BittyWorld Logo" width={180} height={180} />
      <h1>Welcome to BittyWorld</h1>
      <p>The Next Generation of Decentralized Web3 Social Gaming on Solana.</p>
      <HelloWorld />
    </div>
  );
} 