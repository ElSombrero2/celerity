import "@fortawesome/fontawesome-free/css/all.min.css";
import "./styles.scss";
import "./ui/ui.scss"
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { Provider } from "react-redux";
import { useMaximized } from "./app/core/hooks/maximized/maximized";
import { store } from "./app/store";
import { useConfiguration } from "./app/core/hooks/configuration";

const Main = ({ children }: { children: React.ReactElement }) => {
  const maximized = useMaximized();
  const { loading } = useConfiguration()
  return (
    <div className={`main ${maximized && 'maximized'}`}>
      {!loading && children}
    </div>
  )
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Provider store={store}>
      <Main>
        <App />
      </Main>
    </Provider>
  </React.StrictMode>,
);
