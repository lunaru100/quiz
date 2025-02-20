import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import WelcomePage from "./routes/WelcomePage";
import Quiz from "./routes/Quiz";
import Login from "./routes/Login";
import Register from "./routes/Register";
import Result from "./routes/Result";
import "./index.css";

const AppContainer: React.FC = () => {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <WelcomePage />,
    },
    {
      path: "/quiz",
      element: <Quiz />,
    },
    {
      path: "/login",
      element: <Login />,
    },
    {
      path: "/register",
      element: <Register />,
    },
    {
      path: "/results",
      element: <Result />,
    },
  ]);

  return <RouterProvider router={router} />;
};

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <AppContainer />
  </StrictMode>
);
