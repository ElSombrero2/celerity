import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Project } from "./pages/Project/Project";
import { Home } from "./pages/Home/Home";
import { TitleBar } from "./shared/Bars/TitleBar/TitleBar";
import { Navbar } from "./shared/Bars/Navbar/Navbar";

function App() {
  
  return (
    <BrowserRouter>
      <TitleBar />
      <Navbar />
      <Routes>
        <Route path="" element={<Home />} />
        <Route path="/project/:id" element={<Project />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App;
