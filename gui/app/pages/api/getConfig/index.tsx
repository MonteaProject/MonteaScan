import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync } from "fs";
import { Setting } from "../../../app/types/settingTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
    if (req.method === "GET") {
        try {
            const file = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Setting;
            return res.status(200).json(file);
        } catch(e) {
            return res.status(500).end();
        };
    } else {
        return res.status(405).end();
    }
}