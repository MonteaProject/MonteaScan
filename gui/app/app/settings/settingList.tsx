"use client";
import { useState, useRef } from "react";
import { Settings } from "../types/settingTypes";
import {
    Input,
    Button,
    FormLabel,
    FormControl,
    FormHelperText,
    AlertDialog,
    AlertDialogHeader,
    AlertDialogBody,
    AlertDialogFooter,
    AlertDialogContent,
    AlertDialogOverlay,
    AlertDialogCloseButton,
    useDisclosure,
    Modal,
    ModalOverlay,
    ModalContent,
    ModalHeader,
    ModalFooter,
    ModalBody,
    ModalCloseButton,
    Table,
    Thead,
    Tbody,
    Tr,
    Th,
    Td,
    TableCaption,
    TableContainer
} from "../common/components";

async function getConfig() {
    const res = await fetch("http://localhost:3000/api/configList/", {cache: "no-store"});

    if (!res.ok) {
        throw new Error("Failed to fetch config list...");
    }
    const data = await res.json();
    return data as Settings;
}

export default async function ServerList() {
    const config = await getConfig();

    const [ip,   setIP]   = useState('');
    const [port, setPORT] = useState('');
    const [user, setUSER] = useState('');
    const [key,  setKEY]  = useState('');

    // const handleInputChangeIP   = (e: any) => setIP(e.target.value)
    // const handleInputChangePORT = (e: any) => setPORT(e.target.value)
    // const handleInputChangeUSER = (e: any) => setUSER(e.target.value)
    // const handleInputChangeKEY  = (e: any) => setKEY(e.target.value)

    // const isIpError   = ip   === '';
    // const isPortError = port === '';
    // const isUserError = user === '';
    // const isKeyError  = key  === '';

    const createClick = () => {
        console.log("Create:", { ip, port, user, key });
        setIP('');
        setPORT('');
        setUSER('');
        setKEY('');
    };

    const deleteClick = () => {
        console.log("Delete:", { ip, port, user, key });
        setIP('');
        setPORT('');
        setUSER('');
        setKEY('');
        onModalClose();
    };

    const addClick = () => {
        console.log("Create:", { ip, port, user, key });
        setIP('');
        setPORT('');
        setUSER('');
        setKEY('');
    };

    const { isOpen: isAlertOpen, onOpen: onAlertOpen, onClose: onAlertClose } = useDisclosure();
    const { isOpen: isModalOpen, onOpen: onModalOpen, onClose: onModalClose } = useDisclosure();
    const { isOpen: isModalAddOpen, onOpen: onModalAddOpen, onClose: onModalAddClose } = useDisclosure();
    const cancelRef  = useRef<HTMLButtonElement>(null);
    const initialRef = useRef(null);
    const finalRef   = useRef(null);

    return (
        <>
        <>
        <TableContainer>
            <Table variant='simple'>
            <TableCaption>指定したSSH秘密鍵で、スキャン対象サーバにSSH公開鍵認証ができる必要があります。</TableCaption>
                <Thead>
                    <Tr>
                        <Th>IPアドレス</Th>
                        <Th>ユーザー名</Th>
                        <Th isNumeric>ポート番号</Th>
                        <Th>SSH秘密鍵</Th>
                        <Th>編集</Th>
                    </Tr>
                </Thead>
                <Tbody>
                    <Tr>
                        <Td></Td>
                        <Td></Td>
                        <Td isNumeric></Td>
                        <Td></Td>
                        <Td><Button onClick={onModalAddOpen} ref={finalRef} colorScheme="teal">追加</Button></Td>
                    </Tr>
                    <Tr>
                        <Td>127.0.0.1</Td>
                        <Td>montea</Td>
                        <Td isNumeric>22</Td>
                        <Td>/home/montea/id_ed25519</Td>
                        <Td><Button onClick={onModalOpen} ref={finalRef} colorScheme="gray">編集</Button></Td>
                    </Tr>
                </Tbody>
            </Table>
        </TableContainer>
        </>

        <>
        {/* 編集 */}
        <Modal
            size={"xl"}
            blockScrollOnMount={false}
            closeOnOverlayClick={false}
            initialFocusRef={initialRef}
            finalFocusRef={finalRef}
            isOpen={isModalOpen}
            onClose={onModalClose}
        >
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>スキャン対象サーバ</ModalHeader>
                <ModalCloseButton />
                <ModalBody pb={6}>
                    {/* <FormControl isInvalid={isIpError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="ip">IPアドレス</FormLabel>
                        <Input
                            id="ip"
                            placeholder="127.0.0.1"
                            value={ip}
                            // onChange={handleInputChangeIP}
                        />
                        <FormHelperText>
                            スキャン対象サーバーのIPアドレスを入力してください。
                            IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。
                        </FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isUserError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="user">ユーザー名</FormLabel>
                        <Input
                            id="user"
                            placeholder="montea"
                            value={user}
                            // onChange={handleInputChangeUSER}
                        />
                        <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isPortError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="port">ポート番号</FormLabel>
                        <Input
                            id="port"
                            placeholder="22"
                            value={port}
                            // onChange={handleInputChangePORT}
                        />
                        <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isKeyError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="key">SSH秘密鍵ファイルパス</FormLabel>
                        <Input
                            id="key"
                            placeholder="/home/montea/id_ed25519"
                            value={key}
                            // onChange={handleInputChangeKEY}
                        />
                        <FormHelperText>
                            スキャン対象サーバにログイン可能な、SSH秘密鍵ファイルをフルパスで入力してください。
                            MonteaScanサーバーにSSH秘密鍵、スキャン対象サーバーにSSH公開鍵が配置されている必要があります。
                        </FormHelperText>
                    </FormControl>
                </ModalBody>
                <ModalFooter>
                    <Button colorScheme="red" onClick={onAlertOpen} mr={3}>
                        削除
                    </Button>
                    <AlertDialog isOpen={isAlertOpen} leastDestructiveRef={cancelRef} onClose={onAlertClose}>
                        <AlertDialogOverlay>
                            <AlertDialogContent>
                                <AlertDialogHeader fontSize="lg" fontWeight="bold">
                                    削除
                                </AlertDialogHeader>
                                <AlertDialogCloseButton />

                                <AlertDialogBody>
                                    この操作を後で元に戻すことはできません。よろしいですか？
                                </AlertDialogBody>

                                <AlertDialogFooter>
                                    <Button ref={cancelRef} onClick={onAlertClose}>
                                        キャンセル
                                    </Button>
                                    <Button colorScheme="red" onClick={deleteClick} ml={3}>
                                        削除
                                    </Button>
                                </AlertDialogFooter>
                            </AlertDialogContent>
                        </AlertDialogOverlay>
                    </AlertDialog>

                    <Button colorScheme="teal" onClick={createClick} mr={3}>
                        保存
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
        </>

        <>
        {/* 追加 */}
        <Modal
            size={"xl"}
            blockScrollOnMount={false}
            closeOnOverlayClick={false}
            initialFocusRef={initialRef}
            finalFocusRef={finalRef}
            isOpen={isModalAddOpen}
            onClose={onModalAddClose}
        >
            <ModalOverlay />
            <ModalContent>
                <ModalHeader>スキャン対象サーバ</ModalHeader>
                <ModalCloseButton />
                <ModalBody pb={6}>
                    {/* <FormControl isInvalid={isIpError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="ip">IPアドレス</FormLabel>
                        <Input
                            id="ip"
                            placeholder="127.0.0.1"
                            value={ip}
                            // onChange={handleInputChangeIP}
                        />
                        <FormHelperText>
                            スキャン対象サーバーのIPアドレスを入力してください。
                            IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。
                        </FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isUserError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="user">ユーザー名</FormLabel>
                        <Input
                            id="user"
                            placeholder="montea"
                            value={user}
                            // onChange={handleInputChangeUSER}
                        />
                        <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isPortError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="port">ポート番号</FormLabel>
                        <Input
                            id="port"
                            placeholder="22"
                            value={port}
                            // onChange={handleInputChangePORT}
                        />
                        <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
                    </FormControl>

                    {/* <FormControl isInvalid={isKeyError}> */}
                    <FormControl isRequired>
                        <FormLabel htmlFor="key">SSH秘密鍵ファイルパス</FormLabel>
                        <Input
                            id="key"
                            placeholder="/home/montea/id_ed25519"
                            value={key}
                            // onChange={handleInputChangeKEY}
                        />
                        <FormHelperText>
                            スキャン対象サーバにログイン可能な、SSH秘密鍵ファイルをフルパスで入力してください。
                            MonteaScanサーバーにSSH秘密鍵、スキャン対象サーバーにSSH公開鍵が配置されている必要があります。
                        </FormHelperText>
                    </FormControl>
                </ModalBody>
                <ModalFooter>
                    <Button colorScheme="gray" onClick={onModalAddClose} mr={3}>
                        キャンセル
                    </Button>
                    <Button colorScheme="teal" onClick={addClick} mr={3}>
                        保存
                    </Button>
                </ModalFooter>
            </ModalContent>
        </Modal>
        </>
        </>
    )
}