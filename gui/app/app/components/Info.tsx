"use client";
import { Host } from "../hostTypes";
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

export default async function Info ({ infoPromises }: { infoPromises: Host[] }) {
    const info = infoPromises;

    const { isOpen, onOpen, onClose } = useDisclosure()
    const handleClick = () => {
        onOpen()
    }

    return (
        <div>
            {info.map((d) => (
                <table className="responsive-table">
                <thead className="responsive-table__head">
                    <tr className="responsive-table__row">
                        <th className="responsive-table__head__title responsive-table__head__title--hostname">ホスト名</th>
                        <th className="responsive-table__head__title responsive-table__head__title--status">ステータス</th>
                        <th className="responsive-table__head__title responsive-table__head__title--os">OS</th>
                        <th className="responsive-table__head__title responsive-table__head__title--kernel">カーネル</th>
                        <th className="responsive-table__head__title responsive-table__head__title--time">最終スキャン時間</th>
                    </tr>
                </thead>
                    <tbody className="responsive-table__body">
                    <button onClick={() => handleClick()}>
                        <tr className="responsive-table__row">
                            <td className="responsive-table__body__text responsive-table__body__text--hostname">{d.hostname.substring(0, 35)}</td>
                            <td className="responsive-table__body__text responsive-table__body__text--status">
                            <span className="status-indicator status-indicator--active"></span>Active</td>
                            <td className="responsive-table__body__text responsive-table__body__text--os">{d.os.substring(0, 35)}</td>
                            <td className="responsive-table__body__text responsive-table__body__text--kernel">{d.kernel.substring(0, 35)}</td>
                            <td className="responsive-table__body__text responsive-table__body__text--time">{d.time}</td>
                        </tr>
                    </button>
                    </tbody>
                </table>
            ))}
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