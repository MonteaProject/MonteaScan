import SettingList from './settingList';
import { Box } from "../common/components";

export default async function Home() {
  return (
    <main>
      <Box>
          {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
          <SettingList />
      </Box>
    </main>
  );
}