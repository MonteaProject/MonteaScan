import NextLink from "next/link";
import Image from 'next/image';
import { 
  Box,
  Flex,
  Heading,
  Stack,
  HStack,
  VStack,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  MenuItemOption,
  MenuGroup,
  MenuOptionGroup,
  MenuDivider,
  IconButton,

  ExternalLinkIcon,
  SettingsIcon,
  LinkIcon,
  ChevronDownIcon
} from "./common/components";

export default function Header() {
  return (
    <Flex align="center" minH={"60px"} color="green.300" bg="white" borderBottom={1} borderStyle="solid" borderColor="gray.200">
      <HStack direction={["column", "row"]} spacing="24px">
        <Box pl={10}>
          <NextLink href="/">
            <Image src="/icon.png" width={64} height={64} alt="icon" />
          </NextLink>
        </Box>

        <Menu>
          <MenuButton><ChevronDownIcon /> メニュー</MenuButton>
          <MenuList>
            <MenuGroup title="メニュー">
              <MenuItem as='a' href="/components/" icon={<ExternalLinkIcon />}>
                サーバ一覧
              </MenuItem>
              <MenuItem as='a' href="/settings/" icon={<SettingsIcon />}>
                設定
              </MenuItem>
            </MenuGroup>
            <MenuDivider />
            <MenuGroup title="ヘルプ">
              <MenuItem as='a' href='#' icon={<LinkIcon />}>
                ドキュメント
              </MenuItem>
            </MenuGroup>
          </MenuList>
        </Menu>
      </HStack>
    </Flex>
  );
}