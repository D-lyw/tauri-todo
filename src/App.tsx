import "./App.css";
import { Route, Routes } from 'react-router-dom'
import Web3 from "./pages/web3";
import TodoPage from "./pages/todo";
import Setting from "./pages/setting";
import { MainLayout } from "./layout";

function App() {
  return <Routes>
    <Route path="/" element={<MainLayout />} >
      <Route index element={<TodoPage />} />
      <Route path="todo" element={<TodoPage />} />
      <Route path="web3" element={<Web3 />} />
    </Route>
    <Route path="setting" element={<Setting />} />
  </Routes>
}

export default App;
