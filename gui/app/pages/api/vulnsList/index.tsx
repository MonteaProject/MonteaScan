import type { NextApiRequest, NextApiResponse } from "next";
import { readdirSync, readFileSync } from "fs";
import { Detects } from "../../../app/types/cveTypes";
import { Impact, VulnsList, Sum } from "../../../app/types/impactTypes";

// const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    // await delay(1000);

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
        const json = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8")) as Detects;
        if (!json.detect[0]) {
          console.log("/vulns_result/のファイル内にデータが記録されていません...");
        } else {
          let hostname  = json.detect[0].hostname;
          let total     = 0;
          let critical  = 0;
          let important = 0;
          let moderate  = 0;
          let low       = 0;
          for(let i = 0; i < json.detect.length; i++) {
            for(let c = 0; c < json.detect[i].oval.length; c++) {
              for(let p = 0; p < json.detect[i].oval[c].metadata.advisory.cve.length; p++) {
                if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "critical") {
                  criticalTotal += 1;
                  critical += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "important") {
                  importantTotal += 1;
                  important += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "moderate") {
                  moderateTotal += 1;
                  moderate += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "low") {
                  lowTotal += 1;
                  low += 1;
                } else {
                  console.log("新たなCVE重要度が追加されています...");
                }
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

  } else {
    res.status(405).end();
  }
}