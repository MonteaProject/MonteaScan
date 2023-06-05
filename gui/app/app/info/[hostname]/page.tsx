import Info from "./info";
import { Box } from "../../common/components";

export default async function Home({ params, }: { params: { hostname: string }; }) {
  return (
    <Box>
      {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
      <Info infoPass={params.hostname} />
    </Box>
  );
}