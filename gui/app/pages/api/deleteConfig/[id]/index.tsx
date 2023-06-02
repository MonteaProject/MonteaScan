import type { NextApiRequest, NextApiResponse } from "next";
import { readFileSync } from "fs";
import { Setting } from "../../../../app/types/settingTypes";

export default async function handler( req: NextApiRequest, res: NextApiResponse , { params, }: { params: {id: string}; }) {
    if (req.method === "DELETE") {
        const file = JSON.parse(readFileSync("../../src/config/config.json", "utf8")) as Setting;
        // file.id.sort((a: any, b: any) => {
        //   return new Date(b.time).valueOf() - new Date(a.time).valueOf();
        // });

        // file.server.delete(params.id);

        return res.status(200).json(file);
    } else {
        res.status(405).end();
    }
}