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
  TableContainer,
  Radio,
  RadioGroup,
  Stack,
} from "../common/components";


async function getConfig() {
  const res = await fetch("/api/config/get/", {cache: "no-store"});

  if (res.status === 404) {
    notFound();
  }

  if (!res.ok) {
    throw new Error("Failed to fetch config list...");
  }
  const data = await res.json();
  return data.server as EachConfig[];
}

export default async function ConfigList() {
  const router = useRouter();

  const[value, setEdit] = useState({
    user: "",
    host: "",
    port: "",
    key : "",
    os  : "",
  });

  const hostRef = useRef<HTMLInputElement>(null);
  const portRef = useRef<HTMLInputElement>(null);
  const userRef = useRef<HTMLInputElement>(null);
  const keyRef  = useRef<HTMLInputElement>(null);

  const rhelRef   = useRef<HTMLInputElement>(null);
  const almaRef   = useRef<HTMLInputElement>(null);
  const rockyRef  = useRef<HTMLInputElement>(null);
  const ubuntuRef = useRef<HTMLInputElement>(null);

  const cancelRef  = useRef<HTMLButtonElement>(null);
  const initialRef = useRef<HTMLButtonElement>(null);
  const finalRef   = useRef<HTMLButtonElement>(null);
  
  const { isOpen: isAlertOpen, onOpen: onAlertOpen, onClose: onAlertClose } = useDisclosure();
  const { isOpen: isModalOpen, onOpen: onModalOpen, onClose: onModalClose } = useDisclosure();
  const { isOpen: isModalAddOpen, onOpen: onModalAddOpen, onClose: onModalAddClose } = useDisclosure();

  useEffect(() => {
    if (hostRef.current != null) {
      hostRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (portRef.current != null) {
      portRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (userRef.current != null) {
      userRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (keyRef.current != null) {
      keyRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (rhelRef.current != null) {
      rhelRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (almaRef.current != null) {
      almaRef.current.focus();
    }
  }, []);

  useEffect(() => {
    if (rockyRef.current != null) {
      rockyRef.current.focus();
    }
  }, []);
  
  useEffect(() => {
    if (ubuntuRef.current != null) {
      ubuntuRef.current.focus();
    }
  }, []);

  async function postClick() {
    let os;
    if (rhelRef.current?.checked === true) {
      os = rhelRef.current.value;
    } else if (almaRef.current?.checked === true) {
      os = almaRef.current.value;
    } else if (rockyRef.current?.checked === true) {
      os = rockyRef.current.value;
    } else if (ubuntuRef.current?.checked === true) {
      os = ubuntuRef.current.value;
    } else {
      throw Error("ERROR: os null...");
    }
    
    const user = userRef.current?.value;
    if (!user) throw Error("ERROR: user null...");

    const host = hostRef.current?.value;
    if (!host) throw Error("ERROR: host null...");
    
    const port = portRef.current?.value;
    if (!port) throw Error("ERROR: port null...");
    
    const key  = keyRef.current?.value;
    if (!key) throw Error("ERROR: key null...");

    try {
      await fetch("/api/config/post/", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          user: user,
          host: host,
          port: port,
          key : key,
          os  : os,
        }),
        cache: "no-store"
      }).then((res) => {
        if (res.status === 200) {
          onModalAddClose();
          router.push("/config/");
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

  async function patchClick() {
    let os;
    if (rhelRef.current?.checked === true) {
      os = rhelRef.current.value;
    } else if (almaRef.current?.checked === true) {
      os = almaRef.current.value;
    } else if (rockyRef.current?.checked === true) {
      os = rockyRef.current.value;
    } else if (ubuntuRef.current?.checked === true) {
      os = ubuntuRef.current.value;
    } else {
      throw Error("ERROR: os null...");
    }

    const user = userRef.current?.value;
    if (!user) throw Error("ERROR: user null...");

    const host = hostRef.current?.value;
    if (!host) throw Error("ERROR: host null...");
    
    const port = portRef.current?.value;
    if (!port) throw Error("ERROR: port null...");
    
    const key  = keyRef.current?.value;
    if (!key) throw Error("ERROR: key null...");

    try {
      await fetch("/api/config/patch/", {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          user: user,
          host: host,
          port: port,
          key : key,
          os  : os,
        }),
        cache: "no-store"
      }).then((res) => {
          if (res.status === 200) {
            onModalClose();
            router.push("/config/");
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
      await fetch(`/api/config/delete/${host}`, {
        method: "DELETE",
        cache: "no-store"
      }).then((res) => {
        if (res.status === 200) {
          onAlertClose();
          onModalClose();
          router.push("/config/");
        } else {
          onAlertClose();
          onModalClose();
          throw new Error("Failed to delete config list...");
        }
      });
    } catch(e) {
      onAlertClose();
      onModalClose();
      throw new Error("Failed to delete config list...");
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
              <Th>OS</Th>
              <Th>編集</Th>
            </Tr>
          </Thead>
          <Tbody>
            <Tr>
              <Td></Td>
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
              <Td>{v.os}</Td>
              <Td>
                <Button onClick={() => {
                  modalOpen();
                  setEdit({ ...value, user: v.user, host: v.host, port: v.port, key: v.key, os: v.os });
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

      {/* 編集/削除 */}
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
                ref={hostRef}
              />
              <FormHelperText mb={-1.5}>スキャン対象サーバーのIPアドレスを入力してください。</FormHelperText>
              <FormHelperText>IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="user">ユーザー名</FormLabel>
              <Input
                id="user"
                defaultValue={value.user}
                ref={userRef}
              />
              <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="port">ポート番号</FormLabel>
              <Input
                id="port"
                defaultValue={value.port}
                ref={portRef}
              />
              <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="key">SSH秘密鍵ファイル名【フルパス】</FormLabel>
              <Input
                id="key"
                defaultValue={value.key}
                ref={keyRef}
              />
              <FormHelperText mb={-1.5}>スキャン対象サーバーにログイン可能な、ローカルのSSH秘密鍵ファイルをフルパスで入力してください。</FormHelperText>
              <FormHelperText>MonteaScanサーバーにSSH秘密鍵ファイル、スキャン対象サーバーにSSH公開鍵ファイルを配置してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="key">OS</FormLabel>
                <RadioGroup defaultValue={value.os}>
                  <Stack direction='row'>
                    <Radio type="radio" id="RedHat" name="mee" value="RedHat" ref={rhelRef}>RedHat</Radio>
                    <Radio type="radio" id="AlmaLinux" name="mee" value="AlmaLinux" ref={almaRef}>AlmaLinux</Radio>
                    <Radio type="radio" id="RockyLinux" name="mee" value="RockyLinux" ref={rockyRef}>RockyLinux</Radio>
                    <Radio type="radio" id="Ubuntu" name="mee" value="Ubuntu" ref={ubuntuRef}>Ubuntu</Radio>
                  </Stack>
                </RadioGroup>
              <FormHelperText mb={-1.5}>スキャン対象サーバーのOSを選択してください。</FormHelperText>
              <FormHelperText>サポートされている各種OSバージョンの詳細は、ドキュメントを参照してください。</FormHelperText>
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

            <FormControl isRequired>
              <FormLabel htmlFor="host">IPアドレス</FormLabel>
              <Input
                id="host"
                placeholder="127.0.0.1"
                ref={hostRef}
              />
              <FormHelperText mb={-1.5}>スキャン対象サーバーのIPアドレスを入力してください。</FormHelperText>
              <FormHelperText>IPアドレスが固定でない場合は、名前解決可能な、ホスト名を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="user">ユーザー名</FormLabel>
              <Input
                id="user"
                placeholder="montea"
                ref={userRef}
              />
              <FormHelperText>スキャン対象サーバーに、ログイン可能な、ユーザー名を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="port">ポート番号</FormLabel>
              <Input
                id="port"
                placeholder="22"
                ref={portRef}
              />
              <FormHelperText>スキャン対象サーバーで、SSHサーバーが起動しているポート番号を入力してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="key">SSH秘密鍵ファイル名【フルパス】</FormLabel>
              <Input
                id="key"
                placeholder="/home/montea/id_ed25519"
                ref={keyRef}
              />
              <FormHelperText mb={-1.5}>スキャン対象サーバーにログイン可能な、ローカルのSSH秘密鍵ファイルをフルパスで入力してください。</FormHelperText>
              <FormHelperText>MonteaScanサーバーにSSH秘密鍵ファイル、スキャン対象サーバーにSSH公開鍵ファイルを配置してください。</FormHelperText>
            </FormControl>

            <FormControl isRequired mt={5}>
              <FormLabel htmlFor="key">OS</FormLabel>
                <RadioGroup>
                  <Stack direction='row'>
                    <Radio type="radio" id="RedHat" name="drone" value="RedHat" ref={rhelRef}>RedHat</Radio>
                    <Radio type="radio" id="AlmaLinux" name="drone" value="AlmaLinux" ref={almaRef}>AlmaLinux</Radio>
                    <Radio type="radio" id="RockyLinux" name="drone" value="RockyLinux" ref={rockyRef}>RockyLinux</Radio>
                    <Radio type="radio" id="Ubuntu" name="mee" value="Ubuntu" ref={ubuntuRef}>Ubuntu</Radio>
                  </Stack>
                </RadioGroup>
              <FormHelperText mb={-1.5}>スキャン対象サーバーのOSを選択してください。</FormHelperText>
              <FormHelperText>サポートされている各種OSバージョンの詳細は、ドキュメントを参照してください。</FormHelperText>
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