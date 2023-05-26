import Image from 'next/image';
import type { Host } from "../types/hostTypes";
import ServerList from './serverList';
import { Heading, Box } from "../common/components";

export default async function Home() {
  return (
    <main>
      <Box>
          <Heading as="h1" mb={4}>サーバ一覧</Heading>
          {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
          <ServerList />
      </Box>
    </main>
  );
}