import { createBrowserRouter } from 'react-router'
import { RouterProvider } from 'react-router/dom'
import { Landing } from './pages/Landing'

export default function App() {
  return <RouterProvider router={router} />
}

const router = createBrowserRouter([{ path: '/', Component: Landing }])
