import VulnsList from './vulns/vulnsList';
import { Box } from "./common/components";

export default async function Home() {
  return (
    <Box>
        {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
        <VulnsList />
    </Box>
  );
}