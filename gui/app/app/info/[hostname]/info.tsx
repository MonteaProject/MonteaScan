"use client";
import "./info.scss";
import { Vulns } from "../../types/cveTypes";
import { notFound } from "next/navigation";
import NextLink from "next/link";
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
  TableCaption,
  TableContainer,
  ExternalLinkIcon,
  InfoIcon,
  InfoOutlineIcon,
  Badge,
  Tooltip
} from "../../common/components";


const getServerInfo = async (hostname: string) => {
  const res = await fetch(`/api/serverInfo/${hostname}`, {cache: "no-store"});

  if (res.status === 404) {
    notFound();
  }

  if (!res.ok) {
    throw new Error("Failed to fetch server infomation...");
  }

  const data = await res.json();
  return data as Vulns;
}

function CweTable({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>CWE情報</Heading>
      <Table variant='simple'>
        <Thead>
          <Tr>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CVE-ID</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>重要度</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>CWE-ID</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>脆弱性の種類</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>リンク</Th>
            <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>公開日</Th>
          </Tr>
        </Thead>
        <Tbody>
          {v.metadata.advisory.cve.map((c: string) => {
            return (
              <Tr>
                <Td>{c["$value"]}</Td>
                <Td>{c["@impact"]}</Td>
                <Td>{c["@cwe"]}</Td>
                <Td>-</Td>
                <Link color="green.400" href={c["@href"]} isExternal>
                  <Td>{c["@href"]} <ExternalLinkIcon mx="2px" /></Td>
                </Link>
                <Td>{c["@public"]}</Td>
              </Tr>
            )
          })}
        </Tbody>
      </Table>
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

// function HostTable({d}: any) {
//   return (
//     <Table variant='simple' mt="10">
//       <Tr>
//         <Th>ホスト名</Th>
//         <Td>{d.hostname}</Td>
//       </Tr>
//       <Tr>
//         <Th>OS</Th>
//         <Td>{d.os}</Td>
//         <Th>カーネル</Th>
//         <Td>{d.kernel}</Td>
//       </Tr>
//     </Table>
//   )
// }

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

// function IpTable({d}: any) {
//   return (
//     <Table variant='simple' mt="10">
//       {d.ip.map((i: string) => {
//         return (
//           <Tr>
//             <Th>ネットワークインターフェイス名</Th>
//             <Td>{i.split('_!_')[0]}</Td>
//             <Th>IPアドレス</Th>
//             <Td>{i.split('_!_')[1]}</Td>
//           </Tr>
//         )
//       })}
//     </Table>
//   )
// }

function OvalInfo({v}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2" mt="10"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>OVAL</Heading>
      <Table variant='simple'>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>ID</Th>
          <Td>{v["@id"]}</Td>
        </Tr>
        <Tr>
          <Th><Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>クラス</Th>
          <Td>{v["@class"]}</Td>
        </Tr>
      </Table>
    </Box>
  )
}

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
        {v.metadata.reference.map((r: string) => {
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

function CvssTable({c}: any) {
  // "7.8/CVSS:3.0/AV:L/AC:L/PR:L/UI:N/S:U/C:H/I:H/A:H"
  let cvssVec = c["@cvss3"].split("/");
  let score;
  let attackVector;
  let attackVector_item;
  let attackVector_value;
  let attackComplexity;
  let attackComplexity_item;
  let attackComplexity_value;
  let privilegesRequired;
  let privilegesRequired_item;
  let privilegesRequired_value;
  let userInteraction;
  let userInteraction_item;
  let userInteraction_value;
  let scope;
  let scope_item;
  let scope_value;
  let confidentiality;
  let confidentiality_item;
  let confidentiality_value;
  let integrity;
  let integrity_item;
  let integrity_value;
  let availability;
  let availability_item;
  let availability_value;

  if (cvssVec.length === 10) {
    score = cvssVec[0];

    attackVector      = cvssVec[2].split(":")[0];
    attackVector_item = cvssVec[2].split(":")[1];
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
    
    attackComplexity      = cvssVec[3].split(":")[0];
    attackComplexity_item = cvssVec[3].split(":")[1];
    if (attackComplexity === "AC") {
      if (attackComplexity_item === "L") {
        attackComplexity_value = "低"
      } else if (attackComplexity_item === "H") {
        attackComplexity_value = "高"
      } else {
        console.log("新しい評価項目が追加されています...", attackComplexity_item);
      }
    }
    
    privilegesRequired      = cvssVec[4].split(":")[0];
    privilegesRequired_item = cvssVec[4].split(":")[1];
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
    
    userInteraction      = cvssVec[5].split(":")[0];
    userInteraction_item = cvssVec[5].split(":")[1];
    if (userInteraction === "UI") {
      if (userInteraction_item === "N") {
        userInteraction_value = "不要"
      } else if (userInteraction_item === "R") {
        userInteraction_value = "要"
      } else {
        console.log("新しい評価項目が追加されています...", userInteraction_item);
      }
    }
    
    scope      = cvssVec[6].split(":")[0];
    scope_item = cvssVec[6].split(":")[1];
    if (scope === "S") {
      if (scope_item === "U") {
        scope_value = "変更なし"
      } else if (scope_item === "C") {
        scope_value = "変更あり"
      } else {
        console.log("新しい評価項目が追加されています...", scope_item);
      }
    }
    
    confidentiality      = cvssVec[7].split(":")[0];
    confidentiality_item = cvssVec[7].split(":")[1];
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
    
    integrity      = cvssVec[8].split(":")[0];
    integrity_item = cvssVec[8].split(":")[1];
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
    
    availability      = cvssVec[9].split(":")[0];
    availability_item = cvssVec[9].split(":")[1];
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
  } else {
    console.log("CVSSの形式が変更されています...");
  }

  return (
    <Box>
      <Heading size="sm" mb="-2" mt="10" textTransform="none"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>CVSS v3情報</Heading>
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
            <Td>{score}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              攻撃元区分（攻撃の難易度を評価）
            </Th>
            <Td>{attackVector_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              攻撃条件の複雑さ（攻撃の難易度を評価）
            </Th>
            <Td>{attackComplexity_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              攻撃に必要な特権レベル（攻撃の難易度を評価）
            </Th>
            <Td>{privilegesRequired_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              利用者の関与（攻撃の難易度を評価）
            </Th>
            <Td>{userInteraction_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              影響の想定範囲（脆弱性による影響の広がりを評価）
            </Th>
            <Td>{scope_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              機密性への影響（攻撃による影響を評価）
            </Th>
            <Td>{confidentiality_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              完全性への影響（攻撃による影響を評価）
            </Th>
            <Td>{integrity_value}</Td>
            <Td>-</Td>
          </Tr>
          <Tr>
            <Th>
              <Tooltip label='test' fontSize='md'><InfoIcon mb="1" mr="1" /></Tooltip>
              可用性への影響（攻撃による影響を評価）
            </Th>
            <Td>{availability_value}</Td>
            <Td>-</Td>
          </Tr>
        </Tbody>
      </Table>
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
          {v.metadata.advisory.bugzilla.map((b: string) => {
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
            {/* <Th>CPE名（影響を受ける共通プラットフォーム一覧）</Th> */}
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
        {cpeVec.map((v) => {
          return (
            <Tr>
              {/* <Td>{v.cpe}</Td> */}
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
          {v.criteria.criterion.map((criterion: string) => {
            return (
              <Tr>
                <Td>{criterion["@comment"]}</Td>
              </Tr>
            )
          })}
        </Tbody>
      </Table>
      {v.criteria.criteria.map((c: any) => {
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
              {c.criterion.map((c: any) => {
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

function Body({d, v, c}: any) {
  return (
    <Box>
      <Heading size="sm" mb="2"><Tooltip label='test' fontSize='md'><InfoOutlineIcon mb="1" mr="1" /></Tooltip>説明</Heading>
      <p>{v.metadata.description}</p>
      <TableContainer overflowX="unset" overflowY="unset">
        <CweTable
          v = {v}
        />
        <HostTable
          d = {d}
        />
        <IpTable
          d = {d}
        />
        <OvalInfo
          v = {v}
        />
        <FamilyTable
          v = {v}
        />
        <TitleTable
          v = {v}
        />
        <ReferenceTable
          v = {v}
        />
        <AdvisoryTable
          v = {v}
        />
        <CvssTable
          c = {c}
        />
        <BugzillaTable
          v = {v}
        />
        <CpeTable
          v = {v}
        />
        <SubjectTable
          v = {v}
        />
      </TableContainer>
    </Box>
  )
}

function MyTbody({d}: any) {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const handleClick = () => {
    onOpen()
  }

  if (d.pkg.detect === null) {
    return (
      <tbody className="responsive-info-table__body">
        <tr className="responsive-info-table__row">
          <td className="responsive-info-table__body__text responsive-table__body__text">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgver + "-" + d.pkg.pkgrelease === d.pkg.upver + "-" + d.pkg.uprelease ? "-" : "〇"}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgname}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgrelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.upver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.uprelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgarch}</td>
        </tr>
      </tbody>
    )
  }
  return (
    <tbody className="responsive-info-table__body">
      {d.pkg.detect.map((v: any) => {
        return (
          v.metadata.advisory.cve.map((c: any) => (
            <button className="responsive-info-table__button" onClick={() => {
              handleClick();
            }}>
              <tr className="responsive-info-table__row">
                <td className="responsive-info-table__body__text responsive-table__body__text">{c["$value"]}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{c["@impact"]}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{v.metadata.advisory.issued["@date"]}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{v.metadata.advisory.updated["@date"]}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgver + "-" + d.pkg.pkgrelease === d.pkg.upver + "-" + d.pkg.uprelease ? "-" : "〇"}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgname}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgver}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgrelease}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.upver}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.uprelease}</td>
                <td className="responsive-info-table__body__text responsive-table__body__text">{d.pkg.pkgarch}</td>
              </tr>
              <Drawer onClose={onClose} isOpen={isOpen} size="xl" blockScrollOnMount={true}>
                <DrawerOverlay />
                <DrawerContent>
                  <DrawerCloseButton />
                  <DrawerHeader>
                    <Badge variant='outline' colorScheme='green' fontSize='lg'>
                      {c["$value"]}
                    </Badge>
                  </DrawerHeader>
                  <DrawerBody>
                    <Body
                      d = {d}
                      v = {v}
                      c = {c}
                    />
                  </DrawerBody>
                  <DrawerFooter>検出時刻: {d.time}</DrawerFooter>
                </DrawerContent>
              </Drawer>
            </button>
          ))
        )
      })}
    </tbody>
  )
}

export default async function Info ({ infoPass }: { infoPass: string }) {
  const info = await getServerInfo(infoPass);

  return (
    <Box>
      <table className="responsive-info-table">
        <thead className="responsive-info-table__head">
          <tr className="responsive-info-table__row">
            <th className="responsive-info-table__head__title responsive-table__head__title">CVE-ID</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">深刻度</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">発行日</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">更新日</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">アップデート有無</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">パッケージ名称</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">現行バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">現行リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">最新バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">最新リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">アーキテクチャ</th>
          </tr>
        </thead>
        {info.vulns.map((d) => (
          <MyTbody
            d = {d}
          />
        ))}
      </table>
    </Box>
  )
}