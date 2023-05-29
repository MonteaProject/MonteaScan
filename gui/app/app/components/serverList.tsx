import "./serverList.scss";
import { Host } from "../types/hostTypes";
import NextLink from "next/link";
import { Box } from "../common/components";

async function getServerList() {
  const res = await fetch("http://localhost:3000/api/serverList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch server list...");
  }

  const data = await res.json();
  return data as Host[];
}

export default async function ServerList() {
  const f = await getServerList();
  return (
    <Box>
      <table className="responsive-serverlist-table">
        <thead className="responsive-serverlist-table__head">
          <tr className="responsive-serverlist-table__row">
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--hostname">ホスト名</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--impact-1">深刻度:重大</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--impact-2">深刻度:高</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--impact-3">深刻度:中</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--impact-4">深刻度:小</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--os">OS</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--kernel">カーネル</th>
            <th className="responsive-serverlist-table__head__title responsive-serverlist-table__head__title--time">最終スキャン時間</th>
          </tr>
        </thead>
        {f.map((d) => (
        <tbody className="responsive-serverlist-table__body">
          <NextLink className="responsive-serverlist-table__link" href={`/select/${d.hostname}`}>
          <tr className="responsive-serverlist-table__row">
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--hostname">{d.hostname.substring(0, 35)}</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-1">5</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-2">5</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-3">5</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-4">5</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--os">{d.os.substring(0, 35)}</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--kernel">{d.kernel.substring(0, 35)}</td>
            <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--time">{d.time}</td>
          </tr>
          </NextLink>
        </tbody>
        ))}
      </table>
    </Box>
  );
}