import type { NextApiRequest, NextApiResponse } from "next";
import {readdirSync, readFileSync} from "fs";
import { Result } from "postcss";
import { resolve } from "path";
import { Bubblegum_Sans } from "next/font/google";
import { Host } from "../../../app/types";

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    await delay(1000);

    let dirList: string[] = new Array();
    dirList = readdirSync("../../src/vluns_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);

    // for (let v of dirList) {
    //   const file = JSON.parse(readFileSync(`../../src/vluns_result/${v}`, "utf8"));
    //   res.status(200).json(file);
    // }

    let aaa = new Array();

    for (let v of dirList) {
      const file = JSON.parse(readFileSync(`../../src/vluns_result/${v}`, "utf8"));
      aaa.push(file);
    }

    return res.status(200).json(aaa);
    
    // const file = JSON.parse(readFileSync("../../src/vluns_result/rocky9.localdomain.json", "utf8"));

    // tmp.hostname.sort((a: any, b: any) => {
    //   return new Date(b.time).valueOf() - new Date(a.time).valueOf();
    // });

    // res.status(200).json(file);
  
  } else {
    res.status(405).end();
  }
}