import { notFound } from "next/navigation";
import { Host } from "../../types";
import { Suspense } from "react";
import NextLink from "next/link";
import './page.scss'

const getServerInfo = async (hostname: string) => {
    const res = await fetch(
        `http://localhost:3000/api/serverinfo/${hostname}`, {cache: "no-store"}
    );

    if (res.status === 404) {
        notFound();
    }

    if (!res.ok) {
        throw new Error("Failed to fetch article");
    }

    const data = await res.json();
    return data.detect as Host[];
};

const getHogeHoge = async (hostname: string) => {
    const res = await fetch(
        `http://localhost:3000/api/serverinfo/${hostname}`, {cache: "no-store"}
    );

    if (res.status === 404) {
        notFound();
    }

    if (!res.ok) {
        throw new Error("Failed to fetch article");
    }

    const data = await res.json();
    return data.detect as Host[];
};


export default async function ArticleDetail({
    params,
}: {
    params: { hostname: string };
}) {
    const infoPromise = getServerInfo(params.hostname);
    const hogehogePromise = getHogeHoge(params.hostname);

    return (
        // <div>
            <Suspense fallback={<div>Loading infomation...</div>}>
                {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
                <Info infoPromises={infoPromise} />
            </Suspense>
        // {/* </div> */}
    );

    // const [info, hogehoge] = await Promise.all([
    //     infoPromise,
    //     hogehogePromise,
    // ]);

    // return (
    //     <div>
    //     <h1>{article.title}</h1>
    //     <p>{article.content}</p>
    //     <h2>Comments</h2>
    //     <ul>
    //         {comments.map((comment) => (
    //         <li key={comment.id}>{comment.body}</li>
    //         ))}
    //     </ul>
    //     </div>
    // );

    // const info = await infoPromise;
}

async function Info ({
    infoPromises,
}: {
    infoPromises: Promise<Host[]>;
}) {
    const info = await infoPromises;
    return (
        <div>
            {info.map((d) => (
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
                <NextLink href={`/articles/${d.hostname}`}>
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

// async function Comments({
//     commentPromise,
// }: {
//     commentPromise: Promise<Comment[]>;
// }) {
//     const comments = await commentPromise;
// return (
//     <ul>
//         {comments.map((comment) => (
//             <li key={comment.id}>{comment.body}</li>
//         ))}
//     </ul>
// );
