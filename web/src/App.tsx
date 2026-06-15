import { createBrowserRouter } from 'react-router'
import { RouterProvider } from 'react-router/dom'
import Layout from './Layout'
import { Landing } from './pages/Landing'
import Start from './pages/Start'

export default function App() {
  return <RouterProvider router={router} />
}

const router = createBrowserRouter([
  { path: '/', Component: Landing },
  { Component: Layout, children: [{ path: '/start', Component: Start }] },
])
