"use client";
import React from 'react';
import {
    Drawer,
    DrawerBody,
    DrawerFooter,
    DrawerHeader,
    DrawerOverlay,
    DrawerContent,
    DrawerCloseButton,
    useDisclosure,
    Button
} from '../common/components';

export default function Drawers() {
    const [size, setSize] = React.useState('')
    const { isOpen, onOpen, onClose } = useDisclosure()

    const handleClick = (newSize: string) => {
        setSize(newSize)
        onOpen()
    }

    return (
        <>
        <Button
            onClick={() => handleClick("xl")}
            key={"xl"}
            m={4}
        >{`Open xl Drawer`}</Button>

        <Drawer onClose={onClose} isOpen={isOpen} size={size}>
            <DrawerOverlay />
            <DrawerContent>
                <DrawerCloseButton />
                <DrawerHeader />
                <DrawerBody>
                    <p>
                        test
                    </p>
                </DrawerBody>
                <DrawerFooter />
            </DrawerContent>
        </Drawer>
        </>
    )
}