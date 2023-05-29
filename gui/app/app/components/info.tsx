"use client";
import { Pkg } from "../types/pkgTypes";
import {
    Drawer,
    DrawerBody,
    DrawerFooter,
    DrawerHeader,
    DrawerOverlay,
    DrawerContent,
    DrawerCloseButton,
    useDisclosure,
    Box
} from "../common/components";
import "./info.scss"

export default async function Info ({ infoPromises }: { infoPromises: Pkg[] }) {
    const info = infoPromises;

    const { isOpen, onOpen, onClose } = useDisclosure()
    const handleClick = () => {
        onOpen()
    }

    return (
        <Box>
            <div className="container">
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
                    <button onClick={() => handleClick()}>
                    {info.map((d) => (
                    <tbody className="responsive-info-table__body">
                        <tr className="responsive-info-table__row">
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">CVE-2023-XXXX</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--impact">Critical</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">〇</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgname">{d.pkgname.substring(0, 35)}</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgver">{d.pkgver.substring(0, 20)}</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgrelease">{d.pkgrelease.substring(0, 20)}</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--upver">{d.upver.substring(0, 20)}</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--uprelease">{d.uprelease.substring(0, 20)}</td>
                            <td className="responsive-info-table__body__text responsive-table__body__text--pkgarch">{d.pkgarch.substring(0, 20)}</td>
                        </tr>
                    </tbody>
                    ))}
                    </button>
                </table>
            </div>
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