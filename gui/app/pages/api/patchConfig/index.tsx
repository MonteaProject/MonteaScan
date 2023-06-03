import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync, writeFileSync } from "fs";
import { Config } from "../../../app/types/configTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
    if (req.method === "PATCH"){
        try {
            const tmp = req.body;
            const json = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Config;
            json.server.filter((v, index, array) => {
                if (v.host === tmp.host) {
                    array[index] = tmp;
                };
            });
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