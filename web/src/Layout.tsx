import { Outlet } from 'react-router'
import Logo from './components/Logo'

export default function Layout() {
  return (
    <>
      <header className="flex items-center justify-between px-4 py-3">
        <Logo />
      </header>
      <main>
        <Outlet />
      </main>
    </>
  )
}
