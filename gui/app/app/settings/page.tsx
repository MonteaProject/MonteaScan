import SettingList from './settingList';
import { Box } from "../common/components";
import { notFound } from 'next/navigation';
import { Settings } from "../types/settingTypes";

const getConfig = async() => {
  const res = await fetch("http://localhost:3000/api/getConfig/", {cache: "no-store"});

  if (res.status === 404) {
    notFound();
  }

  if (!res.ok) {
      throw new Error("Failed to fetch config list...");
  }
  const data = await res.json();
  return data.server as Settings;
}

export default async function Home() {
  const configPromise = getConfig();
  const config = await configPromise;

  return (
    <main>
      <Box>
          {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
          <SettingList configPromise={config} />
      </Box>
    </main>
  );
}