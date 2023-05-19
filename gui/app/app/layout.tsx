import { Inter } from 'next/font/google'
import Provider from './provider';
import Header from './header';
import Main from './main';
import Footer from './footer';

const inter = Inter({ subsets: ['latin'] })

export const metadata = {
  title: 'MonteaScan',
  description: 'MonteaScan by XXX',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="ja">
      <body className={inter.className}>
        <Provider>
          <Header />
            <Main>{children}</Main>
          <Footer />
        </Provider>
      </body>
    </html>
  );
}