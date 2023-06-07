import type { NextApiRequest, NextApiResponse } from "next";
import {readFileSync} from "fs";
import { Vulns } from "../../../../app/types/cveTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method === "GET") {
    try {
      const hostname = req.query.hostname;
      const file = JSON.parse(readFileSync(`../../src/vulns_result/${hostname}.json`, "utf8")) as Vulns;
      res.status(200).json(file);
    } catch(e) {
      return res.status(500).end();
    }
  } else {
    return res.status(405).end();
  }
}