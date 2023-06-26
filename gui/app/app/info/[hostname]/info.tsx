"use client";
import "./info.scss";
import { useEffect, useState, useMemo } from "react";
import { notFound } from "next/navigation";
import { Vulns, pkgDetect, Cve, Reference, Bugzilla, Criterion, Criteria2, Criterion2 } from "../../types/cveTypes";
import { rhelCPE, cpeVec } from "../../types/cpeTypes";
import {
  Box,
  Link,
  Heading,
  Drawer,
  DrawerBody,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerContent,
  DrawerCloseButton,
  useDisclosure,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
  ExternalLinkIcon,
  InfoIcon,
  InfoOutlineIcon,
  Badge,
  Tooltip,
  IconButton,
  ArrowUpIcon,
  ArrowDownIcon
} from "../../common/components";


function CveTable({v, d}: any) {
  return (
    <Box>
      {v.metadata.advisory.cve.map((c: Cve) => {
        return (
          <Box>
            <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>CVE情報（{c["$value"]}）</Heading>
            <Table variant='simple'>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CVE-ID</Th>
                  <Td>{c["$value"]}</Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>重要度</Th>
                  <Td>{c["@impact"]}</Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CVEリンク</Th>
                  <Td>
                    <Link color="green.400" href={c["@href"]} isExternal>
                      {c["@href"]} <ExternalLinkIcon mx="2px" />
                    </Link>
                  </Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CWE-ID</Th>
                  <Td>{c["@cwe"]}</Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>脆弱性の種類</Th>
                  <Td>{d.cwe_name}</Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CWEリンク</Th>
                  <Td>
                    {d.cwe_url_vec.map((r: string) => {
                      return (
                        <Tr>
                          <Link color="green.400" href={r} isExternal>
                            {r} <ExternalLinkIcon mx="2px" />
                          </Link>
                        </Tr>
                      )
                    })}
                  </Td>
                </Tr>
                <Tr>
                  <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>公開日</Th>
                  <Td>{c["@public"].slice(0, 4) + '-' + c["@public"].slice(4, 6) + '-' + c["@public"].slice(6, 8)}</Td>
                </Tr>

                {/* <Tr>
                  <Td>{c["$value"]}</Td>
                  <Td>{c["@impact"]}</Td>
                  <Td>{c["@cwe"]}</Td>
                  <Td>{d.cwe_name}</Td>
                  {d.cwe_url_vec.map((r: string) => {
                    return (
                      <Link color="green.400" href={r} isExternal>
                        <Td>{r} <ExternalLinkIcon mx="2px" /></Td>
                      </Link>
                    )
                  })}
                  <Link color="green.400" href={c["@href"]} isExternal>
                    <Td>{c["@href"]} <ExternalLinkIcon mx="2px" /></Td>
                  </Link>
                  <Td>{c["@public"]}</Td>
                </Tr> */}
            </Table>
          </Box>
        )
      })}
    </Box>
  )
}

function HostTable({d}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>ホスト情報</Heading>
      <Table variant='simple'>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ホスト名</Th>
          <Td>{d.hostname}</Td>
        </Tr>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>OS</Th>
          <Td>{d.os}</Td>
        </Tr>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>カーネル</Th>
          <Td>{d.kernel}</Td>
        </Tr>
      </Table>
    </Box>
  )
}

function IpTable({d}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>IPアドレス</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>IPアドレス</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ネットワークインターフェイス名</Th>
          </Tr>
        </Thead>
        <Tbody>
        {d.ip.map((i: string) => {
          return (
            <Tr>
              <Td>{i.split('_!_')[1]}</Td>
              <Td>{i.split('_!_')[0]}</Td>
            </Tr>
          )
        })}
        </Tbody>
      </Table>
    </Box>
  )
}

// function OvalInfo({v}: any) {
//   return (
//     <Box>
//       <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>OVAL</Heading>
//       <Table variant='simple'>
//         <Tr>
//           <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ID</Th>
//           <Td>{v["@id"]}</Td>
//         </Tr>
//         <Tr>
//           <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>クラス</Th>
//           <Td>{v["@class"]}</Td>
//         </Tr>
//       </Table>
//     </Box>
//   )
// }

function FamilyTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>対象プラットフォーム</Heading>
      <Table variant='simple'>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ファミリー</Th>
          <Td>{v.metadata.affected["@family"]}</Td>
        </Tr>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>影響プラットフォーム</Th>
          {v.metadata.affected.platform.map((p: string) => {
            return (
              <Td>{p}</Td>
            )
          })}
        </Tr>
      </Table>
    </Box>
  )
}

function TitleTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>タイトル</Heading>
      <Table variant='simple'>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>タイトル</Th>
          <Td>{v.metadata.title}</Td>
        </Tr>
      </Table>
    </Box>
  )
}

function ReferenceTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>リファレンス</Heading>
      <Table variant='simple'>
      <Thead>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ソース</Th>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>リファレンスID</Th>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>リファレンスURL</Th>
        </Tr>
      </Thead>
      <Tbody>
        {v.metadata.reference.map((r: Reference) => {
          return (
            <Tr>
              <Td>{r["@source"]}</Td>
              <Td>{r["@ref_id"]}</Td>
              <Link color="green.400" href={r["@ref_url"]} isExternal>
                <Td>{r["@ref_url"]} <ExternalLinkIcon mx="2px" /></Td>
              </Link>
            </Tr>
          )
        })}
      </Tbody>
    </Table>
    </Box>
  )
}

function AdvisoryTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>アドバイザリー</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th textTransform="none"><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>提供元</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>重大度</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>コピーライト</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>発行日</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>更新日</Th>
          </Tr>
        </Thead>
        <Tbody>
          <Tr>
            <Td>{v.metadata.advisory["@from"]}</Td>
            <Td>{v.metadata.advisory.severity}</Td>
            <Td>{v.metadata.advisory.rights}</Td>
            <Td>{v.metadata.advisory.issued["@date"]}</Td>
            <Td>{v.metadata.advisory.updated["@date"]}</Td>
          </Tr>
        </Tbody>
      </Table>
    </Box>
  )
}

// "7.8/CVSS:3.0/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:H/A:H"
function CvssTable({v}: any) {
  let rhelCpe: any[] = [];
  v.metadata.advisory.cve.map((c: Cve) => {
    let cvssVec = c["@cvss3"].split("/");
    let cveId   = c["$value"];

    if (cvssVec.length === 10) {
      let score = cvssVec[0];

      let attackVector_value;
      let attackComplexity_value;
      let privilegesRequired_value;
      let userInteraction_value;
      let scope_value;
      let confidentiality_value;
      let integrity_value;
      let availability_value;

      let attackVector      = cvssVec[2].split(":")[0];
      let attackVector_item = cvssVec[2].split(":")[1];
      if (attackVector === "AV") {
        if (attackVector_item === "N") {
          attackVector_value = "ネットワーク"
        } else if (attackVector_item === "A") {
          attackVector_value = "隣接"
        } else if (attackVector_item === "L") {
          attackVector_value = "ローカル"
        } else if (attackVector_item === "P") {
          attackVector_value = "物理"
        } else {
          console.log("新しい評価項目が追加されています...", attackVector_item);
        }
      }
      
      let attackComplexity      = cvssVec[3].split(":")[0];
      let attackComplexity_item = cvssVec[3].split(":")[1];
      if (attackComplexity === "AC") {
        if (attackComplexity_item === "L") {
          attackComplexity_value = "低"
        } else if (attackComplexity_item === "H") {
          attackComplexity_value = "高"
        } else {
          console.log("新しい評価項目が追加されています...", attackComplexity_item);
        }
      }
      
      let privilegesRequired      = cvssVec[4].split(":")[0];
      let privilegesRequired_item = cvssVec[4].split(":")[1];
      if (privilegesRequired === "PR") {
        if (privilegesRequired_item === "N") {
          privilegesRequired_value = "不要"
        } else if (privilegesRequired_item === "L") {
          privilegesRequired_value = "低"
        } else if (privilegesRequired_item === "H") {
          privilegesRequired_value = "高"
        } else {
          console.log("新しい評価項目が追加されています...", privilegesRequired_item);
        }
      }
      
      let userInteraction      = cvssVec[5].split(":")[0];
      let userInteraction_item = cvssVec[5].split(":")[1];
      if (userInteraction === "UI") {
        if (userInteraction_item === "N") {
          userInteraction_value = "不要"
        } else if (userInteraction_item === "R") {
          userInteraction_value = "要"
        } else {
          console.log("新しい評価項目が追加されています...", userInteraction_item);
        }
      }
      
      let scope      = cvssVec[6].split(":")[0];
      let scope_item = cvssVec[6].split(":")[1];
      if (scope === "S") {
        if (scope_item === "U") {
          scope_value = "変更なし"
        } else if (scope_item === "C") {
          scope_value = "変更あり"
        } else {
          console.log("新しい評価項目が追加されています...", scope_item);
        }
      }
      
      let confidentiality      = cvssVec[7].split(":")[0];
      let confidentiality_item = cvssVec[7].split(":")[1];
      if (confidentiality === "C") {
        if (confidentiality_item === "N") {
          confidentiality_value = "なし"
        } else if (confidentiality_item === "L") {
          confidentiality_value = "低"
        } else if (confidentiality_item === "H") {
          confidentiality_value = "高"
        } else {
          console.log("新しい評価項目が追加されています...", confidentiality_item);
        }
      }
      
      let integrity      = cvssVec[8].split(":")[0];
      let integrity_item = cvssVec[8].split(":")[1];
      if (integrity === "I") {
        if (integrity_item === "N") {
          integrity_value = "なし"
        } else if (integrity_item === "L") {
          integrity_value = "低"
        } else if (integrity_item === "H") {
          integrity_value = "高"
        } else {
          console.log("新しい評価項目が追加されています...", integrity_item);
        }
      }
      
      let availability      = cvssVec[9].split(":")[0];
      let availability_item = cvssVec[9].split(":")[1];
      if (availability === "A") {
        if (availability_item === "N") {
          availability_value = "なし"
        } else if (availability_item === "L") {
          availability_value = "低"
        } else if (availability_item === "H") {
          availability_value = "高"
        } else {
          console.log("新しい評価項目が追加されています...", availability_item);
        }
      }

      rhelCpe.push(
        {
          "score"                    : score,
          "cveId"                    : cveId,
          "attackVector_value"       : attackVector_value,
          "attackComplexity_value"   : attackComplexity_value,
          "privilegesRequired_value" : privilegesRequired_value,
          "userInteraction_value"    : userInteraction_value,
          "scope_value"              : scope_value,
          "confidentiality_value"    : confidentiality_value,
          "integrity_value"          : integrity_value,
          "availability_value"       : availability_value,
        }
      )
    } else {
      console.log("CVSSの形式が変更されています...");
    }
  })
  
  return (
    <Box>
      {rhelCpe.map((r: rhelCPE) => {
        return (
          <Box>
            <Heading size="sm" mb="-2" mt="10" textTransform="none"><Tooltip label='test' fontSize='md'>
              <InfoOutlineIcon mb="1" mr="1" /></Tooltip>CVSS v3情報（{r.cveId}）
            </Heading>
            <Table variant='simple'>
              <Thead>
                <Tr>
                  <Th></Th>
                  <Th textTransform="none">
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    Red Hat
                  </Th>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    NVD
                  </Th>
                </Tr>
              </Thead>
              <Tbody>
                <Tr>
                  <Th textTransform="none">
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    CVSS v3 基本評価値（スコア）
                  </Th>
                  <Td>{r.score}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    攻撃元区分（攻撃の難易度を評価）
                  </Th>
                  <Td>{r.attackVector_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    攻撃条件の複雑さ（攻撃の難易度を評価）
                  </Th>
                  <Td>{r.attackComplexity_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    攻撃に必要な特権レベル（攻撃の難易度を評価）
                  </Th>
                  <Td>{r.privilegesRequired_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    利用者の関与（攻撃の難易度を評価）
                  </Th>
                  <Td>{r.userInteraction_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    影響の想定範囲（脆弱性による影響の広がりを評価）
                  </Th>
                  <Td>{r.scope_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    機密性への影響（攻撃による影響を評価）
                  </Th>
                  <Td>{r.confidentiality_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    完全性への影響（攻撃による影響を評価）
                  </Th>
                  <Td>{r.integrity_value}</Td>
                  <Td>-</Td>
                </Tr>
                <Tr>
                  <Th>
                    <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                    可用性への影響（攻撃による影響を評価）
                  </Th>
                  <Td>{r.availability_value}</Td>
                  <Td>-</Td>
                </Tr>
              </Tbody>
            </Table>
          </Box>
        )
      })}
    </Box>
  )
}

function BugzillaTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>RedHat Bugzilla</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th textTransform="none"><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>バグ番号</Th>
            <Th textTransform="none"><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>リンク</Th>
            <Th textTransform="none"><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>参考</Th>
          </Tr>
        </Thead>
        <Tbody>
          {v.metadata.advisory.bugzilla.map((b: Bugzilla) => {
            return (
              <Tr>
                <Td>{b["@id"]}</Td>
                <Link color="green.400" href={b["@href"]} isExternal>
                  <Td>{b["@href"]} <ExternalLinkIcon mx="2px" /></Td>
                </Link>
                <Td>{b["$value"]}</Td>
              </Tr>
            )
          })}
        </Tbody>
      </Table>
    </Box>
  )
}

function CpeTable({v}: any) {
  let cpeVec: any[] = [];
  v.metadata.advisory.affected_cpe_list.cpe.map((cpe: string) => {
    if (cpe.split(":").length === 1) {
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :"全て",
          "vendor"  :"全て",
          "product" :"全て",
          "version" :"全て",
          "update"  :"全て",
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 2) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :"全て",
          "product" :"全て",
          "version" :"全て",
          "update"  :"全て",
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 3) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :"全て",
          "version" :"全て",
          "update"  :"全て",
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 4) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :cpe.split(":")[3],
          "version" :"全て",
          "update"  :"全て",
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 5) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :cpe.split(":")[3],
          "version" :cpe.split(":")[4],
          "update"  :"全て",
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 6) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :cpe.split(":")[3],
          "version" :cpe.split(":")[4],
          "update"  :cpe.split(":")[5],
          "edition" :"全て",
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 7) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :cpe.split(":")[3],
          "version" :cpe.split(":")[4],
          "update"  :cpe.split(":")[5],
          "edition" :cpe.split(":")[6],
          "language":"全て"
        }
      )
    }
    if (cpe.split(":").length === 8) {
      let k;
      if (cpe.split(":")[1] === "/h") {
        k = "ハードウェア"
      } else if (cpe.split(":")[1] === "/o") {
        k = "OS"
      } else if (cpe.split(":")[1] === "/a") {
        k = "アプリケーション"
      } else {
        console.log("新しい製品種別が追加されています...");
      }
      cpeVec.push(
        {
          "cpe"     :cpe,
          "kind"    :k,
          "vendor"  :cpe.split(":")[2],
          "product" :cpe.split(":")[3],
          "version" :cpe.split(":")[4],
          "update"  :cpe.split(":")[5],
          "edition" :cpe.split(":")[6],
          "language":cpe.split(":")[7]
        }
      )
    }
  })

  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>CPE情報</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>種別</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ベンダ名</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>製品名</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>バージョン</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>アップデート</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>エディション</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>言語</Th>
          </Tr>
        </Thead>
        <Tbody>
        {cpeVec.map((v: cpeVec) => {
          return (
            <Tr>
              <Td>{v.kind === "" ? "全て" : v.kind}</Td>
              <Td>{v.vendor === "" ? "全て" : v.vendor}</Td>
              <Td>{v.product === "" ? "全て" : v.product}</Td>
              <Td>{v.version === "" ? "全て" : v.version}</Td>
              <Td>{v.update === "" ? "全て" : v.update}</Td>
              <Td>{v.edition === "" ? "全て" : v.edition}</Td>
              <Td>{v.language === "" ? "全て" : v.language}</Td>
            </Tr>
          )
        })}
        </Tbody>
      </Table>
    </Box>
  )
}

function SubjectTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>対象条件</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              {v.criteria["@operator"] === "OR" ? "対象条件：いずれかに該当する場合" : "対象条件：いずれも該当する場合"}
            </Th>
          </Tr>
        </Thead>
        <Tbody>
          {v.criteria.criterion.map((criterion: Criterion) => {
            return (
              <Tr>
                <Td>{criterion["@comment"]}</Td>
              </Tr>
            )
          })}
        </Tbody>
      </Table>
      {v.criteria.criteria.map((c: Criteria2) => {
        return (
          <Table variant='simple' mt="5">
            <Thead>
              <Tr>
                <Th>
                  <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
                  {c["@operator"] === "OR" ? "対象条件：いずれかに該当する場合" : "対象条件：いずれも該当する場合"}
                </Th>
              </Tr>
            </Thead>
            <Tbody>
              {c.criterion.map((c: Criterion2) => {
                return (
                  <Tr>
                    <Td>{c["@comment"]}</Td>
                  </Tr>
                )
              })}
            </Tbody>
          </Table>
        )
      })}
    </Box>
  )
}

function Body({d, v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>説明</Heading>
      <p>{v.metadata.description}</p>
      <TableContainer overflowX="unset" overflowY="unset">
        <HostTable
          d = {d}
        />
        <IpTable
          d = {d}
        />
        <FamilyTable
          v = {v}
        />
        <CveTable
          v = {v}
          d = {d}
        />
        <BugzillaTable
          v = {v}
        />
        <CvssTable
          v = {v}
        />
        <AdvisoryTable
          v = {v}
        />
        <TitleTable
          v = {v}
        />
        <ReferenceTable
          v = {v}
        />
        <CpeTable
          v = {v}
        />
        <SubjectTable
          v = {v}
        />
        {/* <OvalInfo
          v = {v}
        /> */}
      </TableContainer>
    </Box>
  )
}

function MyTbody({d}: any) {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const handleClick = () => {
    onOpen()
  }

  if (d.detect === null) {
    return (
      <tbody className="responsive-info-table__body">
        <tr className="responsive-info-table__row">
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.cveid}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.impact}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.cvssv3_oval}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.cwe_oval}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.issued}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.updated}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.update_flag}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgname}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgrelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.upver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.uprelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgarch}</td>
        </tr>
      </tbody>
    )
  }
  return (
    <tbody className="responsive-info-table__body">
      {d.detect.map((v: pkgDetect) => {
        return (
            <button className="responsive-info-table__button" onClick={() => {
              handleClick();
            }}>
              <tr className="responsive-info-table__row">
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.cveid}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.impact}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.cvssv3_oval}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.cwe_oval}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.issued}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.updated}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.update_flag}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgname}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgver}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgrelease}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.upver}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.uprelease}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkgarch}</td>
              </tr>
              <Drawer onClose={onClose} isOpen={isOpen} size="xl" blockScrollOnMount={true}>
                <DrawerOverlay />
                <DrawerContent>
                  <DrawerCloseButton />
                  <DrawerHeader>
                    <Badge variant='outline' colorScheme='green' fontSize='lg'>
                      {d.cveid}
                    </Badge>
                  </DrawerHeader>
                  <DrawerBody>
                    <Body
                      d = {d}
                      v = {v}
                    />
                  </DrawerBody>
                  <DrawerFooter>検出時刻: {d.time}</DrawerFooter>
                </DrawerContent>
              </Drawer>
            </button>
        )
      })}
    </tbody>
  )
}

export default async function Info ({ infoPass }: { infoPass: string }) {
  const [data, setData]         = useState<Vulns[]>([]);
  const [sortType, setSortType] = useState("pkgAsc");

  const sortedData = useMemo(() => {
    let result = data;

    if (sortType === "pkgDesc") {
      result = [...data].sort((a, b) => {
        return b.pkgname.localeCompare(a.pkgname, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "pkgAsc") {
      result = [...data].sort((a, b) => {
        return a.pkgname.localeCompare(b.pkgname, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "archDesc") {
      result = [...data].sort((a, b) => {
        return b.pkgarch.localeCompare(a.pkgarch, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "archAsc") {
      result = [...data].sort((a, b) => {
        return a.pkgarch.localeCompare(b.pkgarch, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "upFlagDesc") {
      result = [...data].sort((a, b) => {
        return b.update_flag.localeCompare(a.update_flag, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "upFlagAsc") {
      result = [...data].sort((a, b) => {
        return a.update_flag.localeCompare(b.update_flag, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cveIdDesc") {
      result = [...data].sort((a, b) => {
        return b.cveid.localeCompare(a.cveid, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cveIdAsc") {
      result = [...data].sort((a, b) => {
        return a.cveid.localeCompare(b.cveid, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "impactDesc") {
      result = [...data].sort((a, b) => {
        return b.impact.localeCompare(a.impact, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "impactAsc") {
      result = [...data].sort((a, b) => {
        return a.impact.localeCompare(b.impact, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "issuedDesc") {
      result = [...data].sort((a, b) => {
        return b.issued.localeCompare(a.issued, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "issuedAsc") {
      result = [...data].sort((a, b) => {
        return a.issued.localeCompare(b.issued, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "updateDesc") {
      result = [...data].sort((a, b) => {
        return b.updated.localeCompare(a.updated, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "updateAsc") {
      result = [...data].sort((a, b) => {
        return a.updated.localeCompare(b.updated, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cvssv3Desc") {
      result = [...data].sort((a, b) => {
        return b.cvssv3_oval.localeCompare(a.cvssv3_oval, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cvssv3Asc") {
      result = [...data].sort((a, b) => {
        return a.cvssv3_oval.localeCompare(b.cvssv3_oval, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cweIdDesc") {
      result = [...data].sort((a, b) => {
        return b.cwe_oval.localeCompare(a.cwe_oval, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "cweIdAsc") {
      result = [...data].sort((a, b) => {
        return a.cwe_oval.localeCompare(b.cwe_oval, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    }

    return result;
  }, [data, sortType]);
  
  useEffect(() => {
    fetchData();
  }, []);

  const fetchData = async () => {
    const res = await fetch(`/api/serverInfo/${infoPass}`, {cache: "no-store"});

    if (res.status === 404) {
      notFound();
    }
  
    if (!res.ok) {
      throw new Error("Failed to fetch server infomation...");
    }

    const data = await res.json();
    setData(data);
  };

  const PkgAsc = () => {
    setSortType("pkgAsc");
  }

  const PkgDesc = () => {
    setSortType("pkgDesc");
  }

  const ArchAsc = () => {
    setSortType("archAsc");
  }

  const ArchDesc = () => {
    setSortType("archDesc");
  }

  const UpFlagAsc = () => {
    setSortType("upFlagAsc");
  }

  const UpFlagDesc = () => {
    setSortType("upFlagDesc");
  }

  const CveIdAsc = () => {
    setSortType("cveIdAsc");
  }

  const CveIdDesc = () => {
    setSortType("cveIdDesc");
  }

  const ImpactAsc = () => {
    setSortType("impactAsc");
  }

  const ImpactDesc = () => {
    setSortType("impactDesc");
  }

  const IssuedAsc = () => {
    setSortType("issuedAsc");
  }

  const IssuedDesc = () => {
    setSortType("issuedDesc");
  }

  const UpdateAsc = () => {
    setSortType("updateAsc");
  }

  const UpdateDesc = () => {
    setSortType("updateDesc");
  }

  const CVSSv3Asc = () => {
    setSortType("cvssv3Asc");
  }

  const CVSSv3Desc = () => {
    setSortType("cvssv3Desc");
  }

  const CweIdAsc = () => {
    setSortType("cweIdAsc");
  }

  const CweIdDesc = () => {
    setSortType("cweIdDesc");
  }

  return (
    <Box>
      <table className="responsive-info-table">
        <thead className="responsive-info-table__head">
          <tr className="responsive-info-table__row">
            <th className="responsive-info-table__head__title responsive-table__head__title">CVE-ID
              <IconButton
                aria-label = "CVE-ID Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CveIdAsc}
              />
              <IconButton
                aria-label = "CVE-ID Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CveIdDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">深刻度
              <IconButton
                aria-label = "Impact Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {ImpactAsc}
              />
              <IconButton
                aria-label = "Impact Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {ImpactDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">CVSSv3
              <IconButton
                aria-label = "CVSSv3 Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CVSSv3Asc}
              />
              <IconButton
                aria-label = "CVSSv3 Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CVSSv3Desc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">CWE-ID
              <IconButton
                aria-label = "CWE-ID Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CweIdAsc}
              />
              <IconButton
                aria-label = "CWE-ID Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {CweIdDesc}
              />
            </th>

            <th className="responsive-info-table__head__title responsive-table__head__title">発行日
              <IconButton
                aria-label = "Issued Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {IssuedAsc}
              />
              <IconButton
                aria-label = "Issued Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {IssuedDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">更新日
              <IconButton
                aria-label = "Update Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {UpdateAsc}
              />
              <IconButton
                aria-label = "Update Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {UpdateDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">アップデート有無
              <IconButton
                aria-label = "Update Flag Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {UpFlagAsc}
              />
              <IconButton
                aria-label = "Update Flag Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {UpFlagDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">パッケージ名称
              <IconButton
                aria-label = "Pkg Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {PkgAsc}
              />
              <IconButton
                aria-label = "Pkg Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {PkgDesc}
              />
            </th>
            <th className="responsive-info-table__head__title responsive-table__head__title">現行バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">現行リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">最新バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">最新リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">アーキテクチャ
              <IconButton
                aria-label = "Arch Asc"
                icon       = {<ArrowUpIcon />}
                variant    = "outline"
                size       = "xs"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {ArchAsc}
              />
              <IconButton
                aria-label = "Arch Desc"
                icon       = {<ArrowDownIcon />}
                variant    = "outline"
                size       = "xs"
                ml         = "-1"
                fontSize   = "21px"
                _hover     = {{color:"green.300"}}
                onClick    = {ArchDesc}
              />
            </th>
          </tr>
        </thead>
        {sortedData.map((d: Vulns) => (
          <MyTbody
            d = {d}
          />
        ))}
      </table>
    </Box>
  )
}