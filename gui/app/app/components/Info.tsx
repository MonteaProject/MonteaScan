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
    useDisclosure
} from '@chakra-ui/react';

export default async function Info ({ infoPromises }: { infoPromises: Pkg[] }) {
    const info = infoPromises;

    const { isOpen, onOpen, onClose } = useDisclosure()
    const handleClick = () => {
        onOpen()
    }

    return (
        <div>
            <table className="responsive-table">
                <thead className="responsive-table__head">
                    <tr className="responsive-table__row">
                        <th className="responsive-table__head__title responsive-table__head__title--hostname">パッケージ名称</th>
                        <th className="responsive-table__head__title responsive-table__head__title--status">現行バージョン番号</th>
                        <th className="responsive-table__head__title responsive-table__head__title--os">現行リリース番号</th>
                        <th className="responsive-table__head__title responsive-table__head__title--kernel">最新バージョン番号</th>
                        <th className="responsive-table__head__title responsive-table__head__title--kernel">最新リリース番号</th>
                        <th className="responsive-table__head__title responsive-table__head__title--kernel">アーキテクチャ</th>
                    </tr>
                </thead>
                {info.map((d) => (
                <tbody className="responsive-table__body">
                    <button onClick={() => handleClick()}>
                    <tr className="responsive-table__row">
                        <td className="responsive-table__body__text responsive-table__body__text--hostname">{d.pkgname.substring(0, 35)}</td>
                        <td className="responsive-table__body__text responsive-table__body__text--os">{d.pkgver.substring(0, 35)}</td>
                        <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.pkgrelease.substring(0, 35)}</td>
                        <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.upver.substring(0, 35)}</td>
                        <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.uprelease.substring(0, 35)}</td>
                        <td className="responsive-table__body__text responsive-table__body__text--time">{d.pkgarch.substring(0, 35)}</td>
                    </tr>
                    </button>
                </tbody>
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
        </div>
    );
}