'use client';
import { useEffect, useState, useMemo } from "react";
import { notFound } from "next/navigation";
import { Vulns } from "../types/cveTypes";

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

  return (
    <div>
      <div>
        <div>
          <select defaultValue="default" onChange={(e) => setSortType(e.target.value)}>
            <option disabled value="default">
              Sort by
            </option>
            <option value="ascending">Ascending</option>
            <option value="descending">Descending</option>
          </select>
        </div>
        
        <ul>
          {sortedData.map((country, index) => {
            return (
              <li key={index}>
                  {country.pkg.pkgname}
                  {country.pkg.pkgver}
                  {country.pkg.pkgrelease}
                  {country.pkg.upver}
                  {country.pkg.uprelease}
                  {country.pkg.pkgarch}
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
};