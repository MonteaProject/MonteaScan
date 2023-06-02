import type { NextApiRequest, NextApiResponse } from "next";
import {readFileSync} from "fs";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
  if (req.method === "GET") {
    try {
      const hostname = req.query.hostname;
      const file = JSON.parse(readFileSync(`../../src/scan_result/${hostname}.json`, "utf8"));
      res.status(200).json(file);
    } catch(e) {
      return res.status(500).end();
    };
  } else {
    return res.status(405).end();
  }
}