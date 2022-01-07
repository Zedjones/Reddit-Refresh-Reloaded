import React, { useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { useIsLoggedIn } from "..";

interface AuthWrapperProps {
  children: React.ReactElement;
};

export const AuthWrapper = ({ children }: AuthWrapperProps) => {
  const navigate = useNavigate();
  const location = useLocation();
  const [isLoggedIn] = useIsLoggedIn();

  useEffect(() => {
    // We're not logged in
    if (!isLoggedIn && !(location.pathname === "/login" || location.pathname === "/signup")) {
      navigate("/login");
    }
  }, [isLoggedIn, location.pathname, navigate]);

  return children;
}