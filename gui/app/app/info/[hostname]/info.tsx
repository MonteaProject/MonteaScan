"use client";
import "./info.scss";
import { Vulns } from "../../types/cveTypes";
import { notFound } from "next/navigation";
import {
  Box,
  Drawer,
  DrawerBody,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerContent,
  DrawerCloseButton,
  useDisclosure,
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

function Tr({d}: any) {
  if (d.pkg.detect === null) {
    return (
      <tbody className="responsive-info-table__body">
        <tr className="responsive-info-table__row">
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--impact">-</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">〇</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">{d.pkg.pkgname}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgver">{d.pkg.pkgver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgrelease">{d.pkg.pkgrelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--upver">{d.pkg.upver}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--uprelease">{d.pkg.uprelease}</td>
          <td className="responsive-info-table__body__text responsive-table__body__text--pkgarch">{d.pkg.pkgarch}</td>
        </tr>
      </tbody>
    )
  }
  return (
    <tbody className="responsive-info-table__body">
      {d.pkg.detect.map((v: any) => {
        return (
          v.metadata.advisory.cve.map((c: any) => (
            <tr className="responsive-info-table__row">
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">{c["$value"]}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--impact">{c["@impact"]}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">〇</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">{d.pkg.pkgname}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgver">{d.pkg.pkgver}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgrelease">{d.pkg.pkgrelease}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--upver">{d.pkg.upver}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--uprelease">{d.pkg.uprelease}</td>
              <td className="responsive-info-table__body__text responsive-table__body__text--pkgarch">{d.pkg.pkgarch}</td>
            </tr>
          ))
        )
      })}
    </tbody>
  )
}

export default async function Info ({ infoPass }: { infoPass: string }) {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const handleClick = () => {
    onOpen()
  }

  const info = await getServerInfo(infoPass);
  
  return (
    <Box>
      <table className="responsive-info-table">
        <thead className="responsive-info-table__head">
          <tr className="responsive-info-table__row">
            <th className="responsive-info-table__head__title responsive-table__head__title--cve">CVE-ID</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--impact">深刻度</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--update">アップデート有無</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--pkgname">パッケージ名称</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--pkgver">現行バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--pkgrelease">現行リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--upver">最新バージョン番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--uprelease">最新リリース番号</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--pkgarch">アーキテクチャ</th>
          </tr>
        </thead>
        {info.vulns.map((d) => (
          <button className="responsive-info-table__button" onClick={() => handleClick()}>
          <Tr
            d = {d}
          />
          </button>
        ))}
      </table>
      <Drawer onClose={onClose} isOpen={isOpen} size="xl">
        <DrawerOverlay />
        <DrawerContent>
          <DrawerCloseButton />
          <DrawerHeader>{"test"}</DrawerHeader>
          <DrawerBody>
            <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
            Consequat nisl vel pretium lectus quam id. Semper quis lectus
            nulla at volutpat diam ut venenatis. Dolor morbi non arcu risus
            quis varius quam quisque. Massa ultricies mi quis hendrerit dolor
            magna eget est lorem. Erat imperdiet sed euismod nisi porta.
            Lectus vestibulum mattis ullamcorper velit.
            </p>
          </DrawerBody>
          <DrawerFooter>{"end"}</DrawerFooter>
        </DrawerContent>
      </Drawer>
    </Box>
  );
}