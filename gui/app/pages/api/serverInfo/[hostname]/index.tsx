import type { NextApiRequest, NextApiResponse } from "next";
import {readFileSync} from "fs";

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler (
  req: NextApiRequest,
  res: NextApiResponse
) {
  const hostname = req.query.hostname;
  if (req.method === "GET") {
    await delay(1000);
    
    const file = JSON.parse(readFileSync(`../../src/scan_result/${hostname}.json`, "utf8"));
    
    if (!file) {
      res.status(404).end();
    }

    res.status(200).json(file);
  }
}