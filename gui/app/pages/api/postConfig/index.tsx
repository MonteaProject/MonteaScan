import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync, writeFileSync } from "fs";
import { Setting } from "../../../app/types/settingTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
    if (req.method === "POST"){
        try {
            let json = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Setting;
            json.server.push(req.body);
            let save = JSON.stringify(json, null, 2);
            writeFileSync("../../src/config/config.json", save);
            return res.status(200).end();
        } catch(e) {
            return res.status(500).end();
        };
    } else {
        return res.status(405).end();
    }
}