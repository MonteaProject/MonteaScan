// import './serverList.scss'
import { Impact } from "../types/impactTypes";
import NextLink from "next/link";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data as Impact[];
}

// [ { total: 12, critical: 0, important: 5, moderate: 5, low: 2 } ]

export default async function VulnsList() {
  const v = await getVulnsList();
  return (
    <div>
      {v.map((d) => (
        <div>
          <p>合計: {d.total}</p>
          <p>重要: {d.critical}</p>
          <p>高: {d.important}</p>
          <p>中: {d.moderate}</p>
          <p>小: {d.low}</p>
        </div>
      ))}
    </div>
  );
}