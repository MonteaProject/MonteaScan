import type { NextApiRequest, NextApiResponse } from "next";
import {readdirSync, readFileSync} from "fs";


const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    await delay(1000);

    let dirList: string[] = new Array();
    dirList = readdirSync("../../src/vulns_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);

    let files = new Array();

    for (let v of dirList) {
      const file = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8"));
      files.push(file);
    }

    return res.status(200).json(files);

  } else {
    res.status(405).end();
  }
}