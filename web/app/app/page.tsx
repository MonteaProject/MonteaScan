import Image from 'next/image';
import type { Host } from "./types";
import ServerList from './components/ServerList';
import { Heading } from "./common/components";

async function getArticles() {
  const res = await fetch("http://localhost:3000/api/articles", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch host list...");
  }

  const data = await res.json();
  return data.articles as Host[];
}

export default async function Home() {

  const tmp = await getArticles();

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="container">
          <Heading as="h1" mb={4}>サーバ一覧</Heading>
          <ServerList articles={tmp} />
      </div>
    </main>
  );
}