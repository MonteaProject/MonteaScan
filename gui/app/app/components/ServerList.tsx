import './ServerList.scss'
import { Host } from "../hostTypes";
import NextLink from "next/link";

async function getServerList() {
  const res = await fetch("http://localhost:3000/api/serverlist/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch server list...");
  }

  const data = await res.json();
  return data as Host[];
}

export default async function ServerList() {
  const f = await getServerList();
  return (
    <table className="responsive-table">
      <thead className="responsive-table__head">
        <tr className="responsive-table__row">
          <th className="responsive-table__head__title responsive-table__head__title--hostname">ホスト名</th>
          <th className="responsive-table__head__title responsive-table__head__title--status">ステータス</th>
          <th className="responsive-table__head__title responsive-table__head__title--os">OS</th>
          <th className="responsive-table__head__title responsive-table__head__title--kernel">カーネル</th>
          <th className="responsive-table__head__title responsive-table__head__title--time">最終スキャン時間</th>
        </tr>
      </thead>
      {f.map((d) => (
            <NextLink href={`/select/${d.hostname}`}>
              <tbody className="responsive-table__body">
                <tr className="responsive-table__row">
                  <td className="responsive-table__body__text responsive-table__body__text--hostname">{d.hostname}</td>
                  <td className="responsive-table__body__text responsive-table__body__text--status">
                  <span className="status-indicator status-indicator--active"></span>Active</td>
                  <td className="responsive-table__body__text responsive-table__body__text--os">{d.os}</td>
                  <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.kernel}</td>
                  <td className="responsive-table__body__text responsive-table__body__text--time">{d.time}</td>
                </tr>
              </tbody>
            </NextLink>
      ))}
    </table>
  );
}