import Image from 'next/image';
import type { Host } from "./types";
import ServerList from './components/ServerList';
import { Heading } from "./common/components";

async function getServerList() {
  const res = await fetch("http://localhost:3000/api/serverlist/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch server list...");
  }

  const data = await res.json();
  return data.detect as Host[];
}

export default async function Home() {

  const f = await getServerList();

  return (
    <main>
      <div>
          <Heading as="h1" mb={4}>サーバ一覧</Heading>
          <ServerList detect={f} />
      </div>
    </main>
  );
}