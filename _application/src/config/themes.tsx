"use client";
// In themes.tsx
import { useEffect, useState } from "react";

export interface Theme {
  darkMode: boolean;
  background: string;
  input: string;
  overlay: string;
  accent: string;
  textPrimary: string;
  textSecondary: string;
}

export const defaultLightTheme: Theme = {
  darkMode: false,
  background: "hsl(0, 0%, 51%)",
  input: "#646464",
  overlay: "#656565",
  accent: "#ffffff",
  textPrimary: "#000000",
  textSecondary: "#151515",
};

export const defaultDarkTheme: Theme = {
  darkMode: true,
  background: "#18181b",
  input: "#000000",
  overlay: "#09090b",
  accent: "#000000",
  textPrimary: "#717571",
  textSecondary: "#65658f",
};

export const ubuntuTheme: Theme = {
  darkMode: true,
  background: "#dd4814",
  input: "#dd4814",
  overlay: "#dd4814",
  accent: "#dd4814",
  textPrimary: "#ffffff",
  textSecondary: "#09090b",
};

export const draculaTheme: Theme = {
  darkMode: true,
  background: "#282a36",
  input: "#282a36",
  overlay: "#282a36",
  accent: "#282a36",
  textPrimary: "#ffffff",
  textSecondary: "#09090b",
};
