import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync, writeFileSync } from "fs";
import { Config } from "../../../app/types/configTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
    if (req.method === "DELETE") {
        try {
            const { host } = req.query;
            const json = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Config;
            const deleteJson = json.server.filter((v) => v.host !== host);
            const newJson = {server: deleteJson}
            let save = JSON.stringify(newJson, null, 2);
            writeFileSync("../../src/config/config.json", save);
            return res.status(200).end();
        } catch(e) {
            return res.status(500).end();
        };
    } else {
        return res.status(405).end();
    }
}