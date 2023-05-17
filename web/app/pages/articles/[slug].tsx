import { notFound } from "next/navigation";
import { Article, Comment } from "../../app/types";
import { Suspense } from "react";

const getArticle = async (slug: string) => {
    const res = await fetch(
        `http://localhost:3000/api/articles/${slug}`, {cache: "no-store"}
    );

    if (res.status === 404) {
        notFound();
    }

    if (!res.ok) {
        throw new Error("Failed to fetch article");
    }

    const data = await res.json();
    return data as Article;
};

const getComments =async (slug: string) => {
    const res = await fetch (
        `http://localhost:3000/api/articles/${slug}/comments`, {cache: "no-store"}
    );

    if (!res.ok) {
        throw new Error("Failed to fetch comments");
    }

    const data = await res.json();
    return data.comments as Comment[];
};

export default async function ArticleDetail({
        params,
    }: {
        params: { slug: string };
    }) {
        const articlePromise = getArticle(params.slug);
        const commentPromise = getComments(params.slug);

        const article = await articlePromise;

        return (
            <div>
                <h1>{article.title}</h1>
                <p>{article.content}</p>
                <h2>Comments</h2>
                <Suspense fallback={<div>Loading comments...</div>}>
                    {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
                    <Comments commentPromise={commentPromise} />
                </Suspense>
            </div>
        );
    }

    async function Comments({
        commentPromise,
    }: {
        commentPromise: Promise<Comment[]>;
    }) {
        const comments = await commentPromise;
    return (
        <ul>
            {comments.map((comment) => (
                <li key={comment.id}>{comment.body}</li>
            ))}
        </ul>
    );
}