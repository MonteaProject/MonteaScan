import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync } from "fs";
import { Setting } from "../../../app/types/settingTypes";

export default async function handler(req: NextApiRequest, res: NextApiResponse){
    if (req.method === "POST"){
        console.log("API", req.body);
        let json = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Setting;
        json.server.push(req.body);

        return res.status(200);
    } else {
        return res.status(405).end();
    }
}