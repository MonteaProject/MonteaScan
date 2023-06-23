import "./serverList.scss";
import { Server } from "../types/serverTypes";
import NextLink from "next/link";
import { Box } from "../common/components";
import { notFound } from "next/navigation";

async function getServerList() {
  const res = await fetch("http://localhost:3000/api/serverList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch server list...");
  }

  if (res.status === 404) {
    notFound();
  }

  const data = await res.json();
  return data as Server[];
}

export default async function ServerList() {
  const f = await getServerList();
  
  return (
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
        <NextLink className="responsive-serverlist-table__link" href={`/info/${d.hostname}`}>
        <tr className="responsive-serverlist-table__row">
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--hostname">{d.hostname}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-1">{d.critical}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-2">{d.important}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-3">{d.moderate}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--impact-4">{d.low}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--os">{d.os}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--kernel">{d.kernel}</td>
          <td className="responsive-serverlist-table__body__text responsive-serverlist-table__body__text--time">{d.time}</td>
        </tr>
        </NextLink>
      </tbody>
      ))}
    </table>
  );
}