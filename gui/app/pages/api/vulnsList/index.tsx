import type { NextApiRequest, NextApiResponse } from "next";
import { readdirSync, readFileSync } from "fs";
import { Vulns } from "../../../app/types/cveTypes";
import { Impact, VulnsList, Sum } from "../../../app/types/impactTypes";


export default async function handler(req: NextApiRequest, res: NextApiResponse){
  if (req.method === "GET") {
    try {
      let dirList: string[] = new Array();
      dirList = readdirSync("../../src/vulns_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);

      let eachVulnsCount: Impact[] = new Array();

      let totalVulnsCount: Sum[] = new Array();
      let totalTotal    : number = 0;
      let criticalTotal : number = 0;
      let importantTotal: number = 0;
      let moderateTotal : number = 0;
      let lowTotal      : number = 0;

      if (!dirList.length) {
        console.log("/vulns_result/配下にファイルがありません...");
        let impact: Impact = {
          hostname : "null",
          total    : 0,
          critical : 0,
          important: 0,
          moderate : 0,
          low      : 0
        };
        eachVulnsCount.push(impact);

        let count: Sum = {
          totalTotal    : totalTotal,
          criticalTotal : criticalTotal,
          importantTotal: importantTotal,
          moderateTotal : moderateTotal,
          lowTotal      : lowTotal
        }
        totalVulnsCount.push(count);
      } else {
        for (let v of dirList) {
          const json = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8")) as Vulns[];
          if (!json[0]) {
            console.log("/vulns_result/のファイル内にデータが記録されていません...");
          } else {
            let hostname  = json[0].hostname.substring(0, 25);
            let total     = 0;
            let critical  = 0;
            let important = 0;
            let moderate  = 0;
            let low       = 0;
            for(let i = 0; i < json.length; i++) {
              if (json[i].impact === "-") {
                continue;
              } else {
                if (json[i].impact === "Critical") {
                  criticalTotal += 1;
                  critical += 1;
                } else if (json[i].impact === "High") {
                  importantTotal += 1;
                  important += 1;
                } else if (json[i].impact === "Medium") {
                  moderateTotal += 1;
                  moderate += 1;
                } else if (json[i].impact === "Low") {
                  lowTotal += 1;
                  low += 1;
                } else {
                  console.log("Found new cve impact...", json[i].impact);
                }
              }
            }
            total = critical + important + moderate + low;
            let impact: Impact = {
              hostname : hostname,
              total    : total,
              critical : critical,
              important: important,
              moderate : moderate,
              low      : low
            }
            eachVulnsCount.push(impact);
          }
        }
        totalTotal = criticalTotal + importantTotal + moderateTotal + lowTotal;
        let count: Sum = {
          totalTotal    : totalTotal,
          criticalTotal : criticalTotal,
          importantTotal: importantTotal,
          moderateTotal : moderateTotal,
          lowTotal      : lowTotal
        };
        totalVulnsCount.push(count);
      }

      let vulns: VulnsList = {
        impact: eachVulnsCount,
        sum   : totalVulnsCount
      };

      return res.status(200).json(vulns);
    } catch(e) {
      return res.status(500).end();
    }
  } else {
    res.status(405).end();
  }
}