import type { NextApiRequest, NextApiResponse } from "next";
import {fstat, read, readdirSync, readFileSync} from "fs";
import { randomUUID } from "crypto";
import { Host } from "../../../../app/types";

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler (
  req: NextApiRequest,
  res: NextApiResponse
) {
  const hostname = req.query.hostname;
  if (req.method === "GET") {
    await delay(1000);
    const file = JSON.parse(readFileSync(`../../src/vluns_result/${hostname}.json`, "utf8"));
    if (!file) {
      res.status(404).end();
    }
    res.status(200).json(file);
  }
  // } else if (req.method === "PUT") {
  //   delay(1000);
  //   const {title, content} = req.body;
  //   const articles = JSON.parse(readFileSync("hoge.json", "utf8"));
  //   const article = articles.find((a: any) => a.slug === slug);
  //   if (!article) {
  //     res.status(404).end();
  //   }
  //   article.title = title;
  //   article.content = content;
  //   article.updatedAd = new Date();
  //   fs.writeFileSync("hogehoge.json", JSON.stringify(articles));
  //   res.status(200).json(article);
  // }
}
