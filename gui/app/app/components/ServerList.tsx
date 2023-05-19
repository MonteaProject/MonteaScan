import './ServerList.scss'
import { Host } from "../types";
import NextLink from "next/link";

export default function ServerList({ detect }: { detect: Host[] }) {
  return (
    <div>
      {detect.map((d) => (
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
          <NextLink href={`/select/${d.hostname}`}>
            <tbody className="responsive-table__body">
              <tr className="responsive-table__row">
                <td className="responsive-table__body__text responsive-table__body__text--hostname">{d.hostname.substring(0, 35)}</td>
                <td className="responsive-table__body__text responsive-table__body__text--status">
                  <span className="status-indicator status-indicator--active"></span>Active
                </td>
                <td className="responsive-table__body__text responsive-table__body__text--os">{d.os.substring(0, 35)}</td>
                <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.kernel.substring(0, 35)}</td>
                <td className="responsive-table__body__text responsive-table__body__text--time">{d.time}</td>
              </tr>
            </tbody>
          </NextLink>
        </table>
      ))}
    </div>
  );
}