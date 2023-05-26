import type { NextApiRequest, NextApiResponse } from "next";
import {existsSync, fstat, mkdirSync, readdirSync, readFileSync, writeFileSync} from "fs";
import { Detects } from "../../../app/types/cveTypes";
import { Impact, VulnsList, Sum, Diff, HostDiff } from "../../../app/types/impactTypes";
import { arrayBuffer } from "stream/consumers";

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

    let lastCount: Diff[] = new Array();
    let lastCountData: Diff[] = new Array();

    if (!dirList.length) {
      console.log("Vulns結果がありません...");
      let tmp0: HostDiff = {
        totalHostDiff: 0,
        criticalHostDiff: 0,
        importantHostDiff: 0,
        moderateHostDiff: 0,
        lowHostDiff: 0
      }

      let impact: Impact = {
        hostname: "null",
        total: 0,
        critical: 0,
        important: 0,
        moderate: 0,
        low: 0,
        hostDiff: [tmp0]
      }
      impactList.push(impact);

      let count: Sum = {
        total_sum: total_sum,
        critical_sum: critical_sum,
        important_sum: important_sum,
        moderate_sum: moderate_sum,
        low_sum: low_sum
      }
      vulnsCount.push(count);
    } else {
      for (let v of dirList) {
        const json = JSON.parse(readFileSync(`../../src/vulns_result/${v}`, "utf8")) as Detects;
        if (!json.detect[0]) {
          console.log("Vulnsデータがありません...");
        } else {
          let hostname = "null";
          let total = 0;
          let total_diff = 0;
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
          let hostDiff: HostDiff[] = new Array();
          try {
            const save = JSON.parse(readFileSync(`../../save/${v}`, "utf8"));
            total = critical + important + moderate + low;
            let totalHostDiff = total - save.lastVulnsCount.total_diff
            let criticalHostDiff = critical - save.lastVulnsCount.critical_diff
            let importantHostDiff = important - save.lastVulnsCount.important_diff
            let moderateHostDiff = moderate - save.lastVulnsCount.moderate_diff
            let lowHostDiff = low - save.lastVulnsCount.low_diff

            let tmp2: Diff = {
              total_diff: total,
              critical_diff: critical,
              important_diff: important,
              moderate_diff: moderate,
              low_diff: low
            }
            let tmp3 = JSON.stringify({lastVulnsCount: tmp2});
            if (!existsSync("../../save/")) {
              mkdirSync("../../save/");
            }
            writeFileSync(`../../save/${v}`, tmp3);

            let tmp0: HostDiff = {
              totalHostDiff: totalHostDiff,
              criticalHostDiff: criticalHostDiff,
              importantHostDiff: importantHostDiff,
              moderateHostDiff: moderateHostDiff,
              lowHostDiff: lowHostDiff
            }
            hostDiff.push(tmp0);
          } catch (e) {
            console.log("過去のデータがありません...");
            let tmp2: Diff = {
              total_diff: 0,
              critical_diff: 0,
              important_diff: 0,
              moderate_diff: 0,
              low_diff: 0
            }
            let tmp3 = JSON.stringify({lastVulnsCount: tmp2});
            if (!existsSync("../../save/")) {
              mkdirSync("../../save/");
            }
            writeFileSync(`../../save/${v}`, tmp3);

            total_diff = critical + important + moderate + low;
            let tmp0: HostDiff = {
              totalHostDiff: total_diff,
              criticalHostDiff: critical,
              importantHostDiff: important,
              moderateHostDiff: moderate,
              lowHostDiff: low
            }
            hostDiff.push(tmp0);
          }

          total = critical + important + moderate + low;
          let impact: Impact = {
            hostname: hostname,
            total: total,
            critical: critical,
            important: important,
            moderate: moderate,
            low: low,
            hostDiff: hostDiff
          }
          impactList.push(impact);
        }
      }
      total_sum = critical_sum + important_sum + moderate_sum + low_sum;
      let count: Sum = {
        total_sum: total_sum,
        critical_sum: critical_sum,
        important_sum: important_sum,
        moderate_sum: moderate_sum,
        low_sum: low_sum
      }
      vulnsCount.push(count);

      try {
        const bbb = JSON.parse(readFileSync("../../save/vulnsCount.json", "utf8"));
        let t = total_sum - bbb.lastVulnsCount.total_diff;
        let c = critical_sum - bbb.lastVulnsCount.critical_diff;
        let i = important_sum - bbb.lastVulnsCount.important_diff;
        let m = moderate_sum - bbb.lastVulnsCount.moderate_diff;
        let l = low_sum - bbb.lastVulnsCount.low_diff;

        let vulnsC: Diff = {
          total_diff: t,
          critical_diff: c,
          important_diff: i,
          moderate_diff: m,
          low_diff: l
        }
        lastCountData.push(vulnsC);

        let newVulnsC: Diff = {
          total_diff: total_sum,
          critical_diff: critical_sum,
          important_diff: important_sum,
          moderate_diff: moderate_sum,
          low_diff: low_sum
        }
        let aaa = JSON.stringify({lastVulnsCount: newVulnsC});
        if (!existsSync("../../save/")) {
          mkdirSync("../../save/");
        }
        writeFileSync("../../save/vulnsCount.json", aaa);
      } catch (e) {
        let vulnsC: Diff = {
          total_diff: total_sum,
          critical_diff: critical_sum,
          important_diff: important_sum,
          moderate_diff: moderate_sum,
          low_diff: low_sum
        }
        lastCountData.push(vulnsC);
        let aaa = JSON.stringify({lastVulnsCount: vulnsC});
        if (!existsSync("../../save/")) {
          mkdirSync("../../save/");
        }
        writeFileSync("../../save/vulnsCount.json", aaa);
      }
    }

    let vulns: VulnsList = {
      impact: impactList,
      sum: vulnsCount,
      last: lastCountData
    }
    return res.status(200).json(vulns);

  } else {
    res.status(405).end();
  }
}