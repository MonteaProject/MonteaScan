import type { NextApiRequest, NextApiResponse } from "next";
import {readdirSync, readFileSync} from "fs";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
  if (req.method === "GET") {
    try {
      let dirList: string[] = new Array();
      dirList = readdirSync("../../src/scan_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);
      let files = new Array();
      for (let v of dirList) {
        const file = JSON.parse(readFileSync(`../../src/scan_result/${v}`, "utf8"));
        files.push(file);
      }
      return res.status(200).json(files);
    } catch(e) {
      return res.status(500).end();
    };
  } else {
    return res.status(405).end();
  }
}