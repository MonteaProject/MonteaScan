import Image from 'next/image';
import type { Host } from "./types/hostTypes";
import VulnsList from './vulns/vulnsList';
import { Heading } from "./common/components";

export default async function Home() {
  return (
    <main>
      <div>
          <Heading as='h3' size='lg' mb={4}>脆弱性件数</Heading>
          {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
          <VulnsList />
      </div>
    </main>
  );
}