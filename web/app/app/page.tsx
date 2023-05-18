import Image from 'next/image';
import type { Article } from "./types";
import ArticleList from './components/AriticleList';
import { Heading } from "./common/components";

async function getArticles() {
  const res = await fetch("http://localhost:3000/api/articles", {cache: "no-store"});
  // throw new Error("Failed");

  if (!res.ok) {
    throw new Error("Failed to fetch articles");
  }

  const data = await res.json();
  return data.articles as Article[];
}

export default async function Home() {

  const articles = await getArticles();

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">

      <div>
        <Heading as="h1" mb={4}>新着記事</Heading>
        <ArticleList articles={articles} />
        <ul>
          {articles.map((article) => (
            <li key={article.id}>{article.title}</li>
          ))}
        </ul>
      </div>

      <div className="container">
          <table className="responsive-table">
              <thead className="responsive-table__head">
                <tr className="responsive-table__row">
                    <th className="responsive-table__head__title responsive-table__head__title--name">Id</th>
                    <th className="responsive-table__head__title responsive-table__head__title--status">Types</th>
                    <th className="responsive-table__head__title responsive-table__head__title--types">Name</th>
                    <th className="responsive-table__head__title responsive-table__head__title--update">Username</th>
                    <th className="responsive-table__head__title responsive-table__head__title--country">Email</th>
                </tr>
              </thead>
              <tbody className="responsive-table__body">
                <tr className="responsive-table__row">
                    <td className="responsive-table__body__text responsive-table__body__text--name">id</td>
                    <td className="responsive-table__body__text responsive-table__body__text--status">
                      <span className="status-indicator status-indicator--active"></span>
                      Active
                    </td>
                    <td className="responsive-table__body__text responsive-table__body__text--types">name</td>
                    <td className="responsive-table__body__text responsive-table__body__text--update">name</td>
                    <td className="responsive-table__body__text responsive-table__body__text--country">email</td>
                </tr>
              </tbody>
          </table>
      </div>
    </main>
  );
}