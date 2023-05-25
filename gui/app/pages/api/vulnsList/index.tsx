import type { NextApiRequest, NextApiResponse } from "next";
import {readdirSync, readFileSync} from "fs";
import { Detects } from "../../../app/types/cveTypes";
import { Impact, Sum, VulnsList } from "../../../app/types/impactTypes";

const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    await delay(1000);

    let dirList: string[] = new Array();
    dirList = readdirSync("../../src/vulns_result/", {withFileTypes: true}).filter(dirent => dirent.isFile()).map(dirent => dirent.name);

    let impactList: Impact[] = new Array();

    let vulnsCount: Sum[] = new Array();
    let total_sum = 0;
    let critical_sum = 0;
    let important_sum = 0;
    let moderate_sum = 0;
    let low_sum = 0;

    if (!dirList.length) {
      console.log("Vulns結果がありません...");
      let impact: Impact = {
        hostname: "null",
        total: 0,
        critical: 0,
        important: 0,
        moderate: 0,
        low: 0
      };
      impactList.push(impact);

      let count: Sum = {
        total_sum: total_sum,
        critical_sum: critical_sum,
        important_sum: important_sum,
        moderate_sum: moderate_sum,
        low_sum: low_sum
      };
      vulnsCount.push(count);
    } else {
      for (let v of dirList) {
        const json = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8")) as Detects;
        if (!json.detect[0]) {
          console.log("Vulnsデータがありません...");
        } else {
          let hostname = "null";
          let total = 0;
          let critical = 0;
          let important = 0;
          let moderate = 0;
          let low = 0;
          hostname = json.detect[0].hostname;
          for(let i = 0; i < json.detect.length; i++) {
            for(let c = 0; c < json.detect[i].oval.length; c++) {
              for(let p = 0; p < json.detect[i].oval[c].metadata.advisory.cve.length; p++) {
                if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "critical") {
                  critical_sum += 1;
                  critical += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "important") {
                  important_sum += 1;
                  important += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "moderate") {
                  moderate_sum += 1;
                  moderate += 1;
                } else if (json.detect[i].oval[c].metadata.advisory.cve[p]["@impact"] === "low") {
                  low_sum += 1;
                  low += 1;
                } else {
                  console.log("新たなImpactが追加されています...");
                }
              }
            }
          }
          total = critical + important + moderate + low;
          let impact: Impact = {
            hostname: hostname,
            total: total,
            critical: critical,
            important: important,
            moderate: moderate,
            low: low
          };
          impactList.push(impact);
        };
      }
      total_sum = critical_sum + important_sum + moderate_sum + low_sum;
      let count: Sum = {
        total_sum: total_sum,
        critical_sum: critical_sum,
        important_sum: important_sum,
        moderate_sum: moderate_sum,
        low_sum: low_sum
      };
      vulnsCount.push(count);
    };

    let vulns: VulnsList = {
      impact: impactList,
      sum: vulnsCount
    };

    return res.status(200).json(vulns);

  } else {
    res.status(405).end();
  };
}