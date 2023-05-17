import ArticleContent from "./ArticleContent";
import Comments from "./Comments";
import { Heading } from "../../common/components";
import LoadingComments from "./LoadingComments";
import { Suspense } from "react";

const getArticle = async (slug: string) => {
  // ...
}

const getComments = async (slug: string) => {
  // ...
}

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
      {/* <ArticleContent article={article} /> */}
      <Heading as="h2" mt={8} mb={4}>
        Comments
      </Heading>
      <Suspense fallback={<LoadingComments />}>
        {/* @ts-expect-error 現状は jsx が Promise を返すと TypeScript が型エラーを報告するが、将来的には解決される */}
        <Comments commentPromise={commentPromise} />
      </Suspense>
    </div>
  );
}