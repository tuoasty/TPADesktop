import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./App.css"
import {BrowserRouter, Route, Routes} from "react-router-dom";
import LoginPage from "@/pages/LoginPage.tsx";
import RegisterPage from "@/pages/RegisterPage.tsx";
import Dashboard from "@/pages/Dashboard.tsx";
import {AuthProvider, ProtectedRoute} from "@/auth/AuthProvider.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
        <AuthProvider>
            <Routes>
                <Route path="/login" element={<LoginPage/>} />
                <Route path="/register" element={<RegisterPage/>}/>
                <Route path="*" element={<App/>} />
                <Route path="/dashboard" element={
                    <ProtectedRoute>
                        <Dashboard/>
                    </ProtectedRoute>
                }/>
            </Routes>
        </AuthProvider>
    </BrowserRouter>
  </React.StrictMode>,
);
