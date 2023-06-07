"use client";
import React, { useState, useRef, useEffect } from "react";
import { useRouter } from "next/navigation";
import { notFound } from 'next/navigation';
import { EachConfig } from "../types/configTypes";
import {
    Input,
    Button,
    Box,
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
    const res = await fetch("http://localhost:3000/api/getConfig/", {cache: "no-store"});

    if (res.status === 404) {
        notFound();
    }

    if (!res.ok) {
        throw new Error("Failed to fetch config list...");
    }
    const data = await res.json();
    return data.server as EachConfig[];
}

export default async function SettingList() {
    const router = useRouter();

    const[value, setEdit] = useState({
        user: "",
        host: "",
        port: "",
        key : ""
    });

    const inputHost = useRef<HTMLInputElement>(null);
    const inputPort = useRef<HTMLInputElement>(null);
    const inputUser = useRef<HTMLInputElement>(null);
    const inputKey  = useRef<HTMLInputElement>(null);
    
    const { isOpen: isAlertOpen, onOpen: onAlertOpen, onClose: onAlertClose } = useDisclosure();
    const { isOpen: isModalOpen, onOpen: onModalOpen, onClose: onModalClose } = useDisclosure();
    const { isOpen: isModalAddOpen, onOpen: onModalAddOpen, onClose: onModalAddClose } = useDisclosure();
    const cancelRef  = useRef<HTMLButtonElement>(null);
    const initialRef = useRef(null);
    const finalRef   = useRef(null);

    useEffect(() => {
        if (inputHost.current != null) {
            inputHost.current.focus();
        }
    }, []);

    useEffect(() => {
        if (inputPort.current != null) {
            inputPort.current.focus();
        }
    }, []);

    useEffect(() => {
        if (inputUser.current != null) {
            inputUser.current.focus();
        }
    }, []);

    useEffect(() => {
        if (inputKey.current != null) {
            inputKey.current.focus();
        }
    }, []);

    async function patchClick() {
        try {
            await fetch("/api/patchConfig/", {
                method: "PATCH",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    user: inputUser.current.value,
                    host: inputHost.current.value,
                    port: inputPort.current.value,
                    key : inputKey.current.value
                }),
                cache: "no-store"
            }).then((res) => {
                if (res.status === 200) {
                    onModalClose();
                    router.push("/settings/");
                    router.refresh();
                } else {
                    onModalClose();
                    throw new Error("Failed to patch config list...");
                }
            });
        } catch(e) {
            onModalClose();
            throw new Error("Failed to patch config list...");
        }
    };

    async function deleteClick(host: string) {
        try {
            await fetch(`/api/deleteConfig/${host}`, {
                method: "DELETE",
                cache: "no-store"
            }).then((res) => {
                if (res.status === 200) {
                    onModalClose();
                    router.push("/settings/");
                    router.refresh();
                } else {
                    onModalClose();
                    throw new Error("Failed to delete config list...");
                }
            });
        } catch(e) {
            onModalClose();
            throw new Error("Failed to delete config list...");
        }
    };

    async function postClick() {
        try {
            await fetch("/api/postConfig/", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify({
                    user: inputUser.current.value,
                    host: inputHost.current.value,
                    port: inputPort.current.value,
                    key : inputKey.current.value
                }),
                cache: "no-store"
            }).then((res) => {
                if (res.status === 200) {
                    onModalAddClose();
                    router.push("/settings/");
                } else {
                    onModalAddClose();
                    throw new Error("Failed to post config list...");
                }
            });
        } catch(e) {
            onModalAddClose();
            throw new Error("Failed to post config list...");
        }
    };

    function modalOpen() {
        onModalOpen();
    }

    const config = await getConfig();

    return (
        <Box>
            <TableContainer>
                <Table variant='simple'>
                <TableCaption>指定したSSH秘密鍵を使用して、MonteaScanサーバからスキャン対象サーバにSSH公開鍵認証ができる必要があります。</TableCaption>
                    <Thead>
                        <Tr>
                            <Th>IPアドレス</Th>
                            <Th>ユーザー名</Th>
                            <Th>ポート番号</Th>
                            <Th>SSH秘密鍵</Th>
                            <Th>編集</Th>
                        </Tr>
                    </Thead>
                    <Tbody>
                        <Tr>
                            <Td></Td>
                            <Td></Td>
                            <Td></Td>
                            <Td></Td>
                            <Td><Button onClick={onModalAddOpen} ref={finalRef} colorScheme="teal">追加</Button></Td>
                        </Tr>
                        {config.map((v) => (
                        <Tr key={v.host}>
                            <Td>{v.host}</Td>
                            <Td>{v.user}</Td>
                            <Td>{v.port}</Td>
                            <Td>{v.key}</Td>
                            <Td>
                                <Button onClick={() => {
                                    modalOpen();
                                    setEdit({ ...value, user: v.user, host: v.host, port: v.port, key: v.key});
                                }}
                                ref={finalRef}
                                colorScheme="gray">編集
                                </Button>
                            </Td>
                        </Tr>
                        ))}
                    </Tbody>
                </Table>
            </TableContainer>

            {/* PATCH/DELETE */}
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
                        <FormControl isRequired>
                            <FormLabel htmlFor="host">IPアドレス</FormLabel>
                            <Input
                                id="host"
                                defaultValue={value.host}
                                ref={inputHost}
                            />
                            <FormHelperText>
                                スキャン対象サーバーのIPアドレスを入力してください。
                                IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。
                            </FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="user">ユーザー名</FormLabel>
                            <Input
                                id="user"
                                defaultValue={value.user}
                                ref={inputUser}
                            />
                            <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="port">ポート番号</FormLabel>
                            <Input
                                id="port"
                                defaultValue={value.port}
                                ref={inputPort}
                            />
                            <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="key">SSH秘密鍵ファイルパス</FormLabel>
                            <Input
                                id="key"
                                defaultValue={value.key}
                                ref={inputKey}
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
                                        <Button colorScheme="red" ml={3} onClick={() => deleteClick(value.host)}>
                                            削除
                                        </Button>
                                    </AlertDialogFooter>
                                </AlertDialogContent>
                            </AlertDialogOverlay>
                        </AlertDialog>

                        <Button colorScheme="teal" mr={3} onClick={patchClick}>
                            保存
                        </Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>

            {/* POST */}
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
                        <FormControl isRequired>
                            <FormLabel htmlFor="host">IPアドレス</FormLabel>
                            <Input
                                id="host"
                                placeholder="127.0.0.1"
                                ref={inputHost}
                            />
                            <FormHelperText>
                                スキャン対象サーバーのIPアドレスを入力してください。
                                IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。
                            </FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="user">ユーザー名</FormLabel>
                            <Input
                                id="user"
                                placeholder="montea"
                                ref={inputUser}
                            />
                            <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="port">ポート番号</FormLabel>
                            <Input
                                id="port"
                                placeholder="22"
                                ref={inputPort}
                            />
                            <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
                        </FormControl>

                        <FormControl isRequired>
                            <FormLabel htmlFor="key">SSH秘密鍵ファイルパス</FormLabel>
                            <Input
                                id="key"
                                placeholder="/home/montea/id_ed25519"
                                ref={inputKey}
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
                        <Button colorScheme="teal" onClick={postClick} mr={3}>
                            保存
                        </Button>
                    </ModalFooter>
                </ModalContent>
            </Modal>
        </Box>
    )
}