'use client';
import { useEffect, useState, useMemo } from "react";
import { notFound } from "next/navigation";
import { Vulns } from "../types/cveTypes";
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
} from "../common/components";

function MyTbody({d}: any) {
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
}

export default async function Home() {
  const [data, setData] = useState([]);
  const [sortType, setSortType] = useState("default");

  const sortedData = useMemo(() => {
    // let result = data;
    let result = data;

    if (sortType === "descending") {
      result = [...data].sort((a, b) => {
        return b.pkg.pkgname.localeCompare(a.pkg.pkgname, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    } else if (sortType === "ascending") {
      result = [...data].sort((a, b) => {
        return a.pkg.pkgname.localeCompare(b.pkg.pkgname, "en", {sensitivity: "variant", ignorePunctuation: false, caseFirst: "false", numeric: true});
      });
    }

    return result;
  }, [data, sortType]);
  
  useEffect(() => {// コンポーネントマウント時、1回だけfetchを実行
    fetchData();
  }, []);

  const fetchData = async () => {
    // const res = await fetch(
    //   "https://restcountries.com/v2/all?fields=name,region,area"
    // );

    const res = await fetch(`/api/serverInfo/vsred89.msr.co.jp`, {cache: "no-store"});

    if (res.status === 404) {
      notFound();
    }
  
    if (!res.ok) {
      throw new Error("Failed to fetch server infomation...");
    }

    const data = await res.json();
    setData(data.vulns);
  };

  // return (
  //   <div>
  //     <div>
  //       <div>
  //         <select defaultValue="default" onChange={(e) => setSortType(e.target.value)}>
  //           <option disabled value="default">
  //             Sort by
  //           </option>
  //           <option value="ascending">Ascending</option>
  //           <option value="descending">Descending</option>
  //         </select>
  //       </div>
        
  //       <ul>
  //         {sortedData.map((country, index) => {
  //           return (
  //             <li key={index}>
  //                 {country.pkg.pkgname}
  //                 {country.pkg.pkgver}
  //                 {country.pkg.pkgrelease}
  //                 {country.pkg.upver}
  //                 {country.pkg.uprelease}
  //                 {country.pkg.pkgarch}
  //             </li>
  //           );
  //         })}
  //       </ul>
  //     </div>
  //   </div>
  // );

  return (
    <Box>
      <div>
        <select defaultValue="default" onChange={(e) => setSortType(e.target.value)}>
          <option disabled value="default">
            Sort by
          </option>
          <option value="ascending">Ascending</option>
          <option value="descending">Descending</option>
        </select>
      </div>

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
        {sortedData.map((d, index) => (
          <MyTbody
            d = {d}
          />
        ))}
      </table>
    </Box>
  )
};