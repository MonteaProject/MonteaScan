import './ServerList.scss'
import { Host } from "../types";
import NextLink from "next/link";

export default function ServerList({ articles }: { articles: Host[] }) {
  return (
    <div>
      {articles.map((article) => (
        <table className="responsive-table">
          <NextLink href={`/articles/${article.slug}`}>
            <thead className="responsive-table__head">
              <tr className="responsive-table__row">
                  <th className="responsive-table__head__title responsive-table__head__title--name">ホスト名</th>
                  <th className="responsive-table__head__title responsive-table__head__title--status">ステータス</th>
                  <th className="responsive-table__head__title responsive-table__head__title--types">OS</th>
                  <th className="responsive-table__head__title responsive-table__head__title--update">カーネル</th>
                  <th className="responsive-table__head__title responsive-table__head__title--country">最終スキャン時間</th>
              </tr>
            </thead>
            <tbody className="responsive-table__body">
              <tr className="responsive-table__row">
                <td className="responsive-table__body__text responsive-table__body__text--name">{article.hostname}</td>
                <td className="responsive-table__body__text responsive-table__body__text--status">
                  <span className="status-indicator status-indicator--active"></span>Active
                </td>
                <td className="responsive-table__body__text responsive-table__body__text--types">{article.os}</td>
                <td className="responsive-table__body__text responsive-table__body__text--update">{article.kernel}</td>
                <td className="responsive-table__body__text responsive-table__body__text--country">{article.time}</td>
              </tr>
            </tbody>
          </NextLink>
        </table>
      ))}
    </div>
  );
}