import type { NextApiRequest, NextApiResponse } from "next";
import { readdirSync, readFileSync } from "fs";
import { Vulns } from "../../../app/types/cveTypes";
import { Server } from "../../../app/types/serverTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
  if (req.method === "GET") {
    try {
      let dirList: string[] = new Array();
      dirList = readdirSync("../../src/vulns_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);

      let eachVulnsCount: Server[] = new Array();

      if (!dirList.length) {
        console.log("/vulns_result/配下にファイルがありません...");
        let impact: Server = {
          hostname : "null",
          os       : "null",
          kernel   : "null",
          time     : "null",
          total    : 0,
          critical : 0,
          important: 0,
          moderate : 0,
          low      : 0
        };
        eachVulnsCount.push(impact);
      } else {
        for (let v of dirList) {
          const json = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8")) as Vulns;
          if (!json.vulns[0]) {
            console.log("/vulns_result/のファイル内にデータが記録されていません...");
          } else {
            let hostname  = json.vulns[0].hostname;
            let os        = json.vulns[0].os;
            let kernel    = json.vulns[0].kernel;
            let time      = json.vulns[0].time;
            let total     = 0;
            let critical  = 0;
            let important = 0;
            let moderate  = 0;
            let low       = 0;
            for(let i = 0; i < json.vulns.length; i++) {
              if (json.vulns[i].detect === null) {
                continue;
              } else {
                for(let c = 0; c < json.vulns[i].detect.length; c++) {
                  for(let p = 0; p < json.vulns[i].detect[c].metadata.advisory.cve.length; p++) {
                    if (json.vulns[i].detect[c].metadata.advisory.cve[p]["@impact"] === "critical") {
                      critical += 1;
                    } else if (json.vulns[i].detect[c].metadata.advisory.cve[p]["@impact"] === "important") {
                      important += 1;
                    } else if (json.vulns[i].detect[c].metadata.advisory.cve[p]["@impact"] === "moderate") {
                      moderate += 1;
                    } else if (json.vulns[i].detect[c].metadata.advisory.cve[p]["@impact"] === "low") {
                      low += 1;
                    } else {
                      console.log("新たなCVE重要度が追加されています...");
                    }
                  }
                }
              }
            }
            total = critical + important + moderate + low;
            let impact: Server = {
              hostname : hostname,
              os       : os,
              kernel   : kernel,
              time     : time,
              total    : total,
              critical : critical,
              important: important,
              moderate : moderate,
              low      : low
            }
            eachVulnsCount.push(impact);
          }
        }
      }
      return res.status(200).json(eachVulnsCount);
    } catch(e) {
      return res.status(500).end();
    }
  } else {
    res.status(405).end();
  }
}