import './globals.scss'
import { Inter } from 'next/font/google'
import Provider from './Provider';
import Header from './Header';
import Main from './Main';
import Footer from './Footer';

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
    <html lang="en">
      <body className={inter.className}>
        <Provider>
          <Header />
            <Main>{children}</Main>
          <Footer />
        </Provider>
      </body>
    </html>
  )
}