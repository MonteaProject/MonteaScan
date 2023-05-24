// import './serverList.scss'
import { Cve } from "../types/cveTypes";
import NextLink from "next/link";

async function getVulnsList() {
  const res = await fetch("http://localhost:3000/api/vulnsList/", {cache: "no-store"});

  if (!res.ok) {
    throw new Error("Failed to fetch vulns list...");
  }

  const data = await res.json();
  return data.detect.oval.metadata.advisory.cve as Cve[];
}

export default async function VulnsList() {
  const v = await getVulnsList();
  return (
    <div>
      <p>Total</p>
      <p>Critical</p>
      <p>Important</p>
      <p>Moderate</p>
      <p>Low</p>
    </div>
  );
}