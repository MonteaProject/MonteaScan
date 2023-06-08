"use client";
import "./info.scss";
import { Vulns } from "../../types/cveTypes";
import { notFound } from "next/navigation";
import NextLink from "next/link";
import {
  Box,
  Link,
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
  ExternalLinkIcon
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

function Body({d, v}: any) {
  return (
    <Box>
      <TableContainer>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>ホスト名</Th>
              <Th>OS</Th>
              <Th>カーネル</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>{d.hostname}</Td>
              <Td>{d.os}</Td>
              <Td>{d.kernel}</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>ネットワークインターフェイス名</Th>
              <Th>IPアドレス</Th>
            </Tr>
          </Thead>
          <Tbody>
            {d.ip.map((i: string) => {
              return (
                <Tr>
                  <Td>{i.split(':')[0]}</Td>
                  <Td>{i.split(':')[1]}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>OVAL-ID</Th>
              <Th>OVAL-CLASS</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>{v["@id"]}</Td>
              <Td>{v["@class"]}</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>タイトル</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>{v.metadata.title}</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>ファミリー</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>{v.metadata.affected["@family"]}</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>影響プラットフォーム</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.metadata.affected.platform.map((p: string) => {
              return (
                <Tr>
                  <Td>{p}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>リファレンスID</Th>
              <Th>リファレンスURL</Th>
              <Th>ソース</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.metadata.reference.map((r: string) => {
              return (
                <Tr>
                  <Td>{r["@ref_id"]}</Td>
                  <Link color="green.400" href={r["@ref_url"]} isExternal>
                    <Td>{r["@ref_url"]} <ExternalLinkIcon mx="2px" /></Td>
                  </Link>
                  <Td>{r["@source"]}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>参考</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td>{v.metadata.description}</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>アドバイザリー:提供元</Th>
              <Th>アドバイザリー:重大度</Th>
              <Th>アドバイザリー:コピーライト</Th>
              <Th>アドバイザリー:発行日</Th>
              <Th>アドバイザリー:更新日</Th>
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
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th></Th>
              <Th textTransform="none">Red Hat</Th>
              <Th>NVD</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Th>CVSS v3 基本スコア</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>攻撃ベクトル</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>攻撃の複雑さ</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>必要な権限</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>ユーザーインタラクション</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>スコープ</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>機密保持への影響</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>完全性への影響</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
            <Tr>
              <Th>可用性への影響</Th>
              <Td>c</Td>
              <Td>-</Td>
            </Tr>
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>CWE-ID</Th>
              <Th>脆弱性の種類</Th>
              <Th>リンク</Th>
              <Th>重要度</Th>
              <Th>公開日</Th>
              <Th>CVE-ID</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.metadata.advisory.cve.map((c: string) => {
              return (
                <Tr>
                  <Td>{c["@cwe"]}</Td>
                  <Td>-</Td>
                  <Link color="green.400" href={c["@href"]} isExternal>
                    <Td>{c["@href"]} <ExternalLinkIcon mx="2px" /></Td>
                  </Link>
                  <Td>{c["@impact"]}</Td>
                  <Td>{c["@public"]}</Td>
                  <Td>{c["$value"]}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <p>RedHat Bugzilla</p>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>リンク</Th>
              <Th>ID</Th>
              <Th>参考</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.metadata.advisory.bugzilla.map((b: string) => {
              return (
                <Tr>
                  <Link color="green.400" href={b["@href"]} isExternal>
                    <Td>{b["@href"]} <ExternalLinkIcon mx="2px" /></Td>
                  </Link>
                  <Td>{b["@id"]}</Td>
                  <Td>{b["$value"]}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <p>影響を受ける共通プラットフォーム一覧</p>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>CPE名</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.metadata.advisory.affected_cpe_list.cpe.map((cpe: string) => {
              return (
                <Tr>
                  <Td>{cpe}</Td>
                </Tr>
              )
            })}
          </Tbody>
        </Table>
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>条件</Th>
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
        <Table variant='simple'>
          <Thead>
            <Tr>
              <Th>条件</Th>
            </Tr>
          </Thead>
          <Tbody>
            {v.criteria.criteria.map((criteria: any) => {
              return (
                criteria.criterion.map((c: any) => {
                  return (
                    <Tr>
                      <Td>{c["@comment"]}</Td>
                    </Tr>
                  )
                })
              )
            })}
          </Tbody>
        </Table>
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
                  <DrawerHeader>{c["$value"]}</DrawerHeader>
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
            <th className="responsive-info-table__head__title responsive-table__head__title--cve">CVE-ID</th>
            <th className="responsive-info-table__head__title responsive-table__head__title--impact">深刻度</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">発行日</th>
            <th className="responsive-info-table__head__title responsive-table__head__title">更新日</th>
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
          <MyTbody
            d = {d}
          />
        ))}
      </table>
    </Box>
  )
}