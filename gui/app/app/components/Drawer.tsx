"use client";
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

import React from 'react';

export default function Drawers() {
    const [size, setSize] = React.useState('')
    const { isOpen, onOpen, onClose } = useDisclosure()

    const handleClick = (newSize: string) => {
        setSize(newSize)
        onOpen()
    }

    const sizes = ['xs', 'sm', 'md', 'lg', 'xl', 'full']

    return (
        <>
        {/* <Button ref={btnRef} onClick={onOpen}>
            <HamburgerIcon />
        </Button> */}

        {sizes.map((size) => (
            <Button
            onClick={() => handleClick(size)}
            key={size}
            m={4}
            >{`Open ${size} Drawer`}</Button>
        ))}

        <Drawer onClose={onClose} isOpen={isOpen} size={size}>
            <DrawerOverlay />
            <DrawerContent>
                <DrawerCloseButton />
                <DrawerHeader>{`${size} drawer contents`}</DrawerHeader>
                <DrawerBody>
                    <p>
                        test
                    </p>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
        </>
    )
}